//! 마크다운 링크 추출 · 해석 · 재작성. (스테이징 트리에 적용되는 정규화 규칙)

use std::collections::BTreeSet;

use crate::text::{
    basename, dirname, encode_path_component, extension, nfc, normalize_rel_path, percent_decode,
    with_extension,
};

/// 저장소 안에서 링크가 가리키는 대상.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Kind {
    /// 노트 페이지 (notes 배열 인덱스)
    Note(usize),
    /// 섹션 페이지 (섹션 디렉터리)
    Section(String),
    /// 노트가 아닌 저장소 파일 (이미지 등)
    Asset(String),
    /// 저장소 밖 URL
    External,
    /// 같은 페이지 앵커 (`#...`)
    Anchor,
    /// 해석 실패
    Unresolved,
}

#[derive(Debug, Clone)]
pub struct LinkRecord {
    pub line: usize,
    pub indent: usize,
    pub bullet: bool,
    pub raw: String,
    pub kind: Kind,
}

#[derive(Debug, Default, Clone)]
pub struct Stats {
    /// 스캔한 인라인 링크 총 개수 (`](...)` 형태)
    pub inline_total: usize,
    /// 코드펜스 안이라 건너뛴 개수
    pub in_fence_skipped: usize,
    /// 저장소 밖 URL
    pub external: usize,
    /// 페이지 앵커
    pub anchor: usize,
    /// 저장소 내부로 해석된 링크 (노트/섹션/파일)
    pub internal: usize,
    /// 실제로 바이트가 바뀐 링크 수
    pub rewritten: usize,
    pub note_links: usize,
    pub section_links: usize,
    pub asset_links: usize,
    /// 대소문자 무시 폴백으로 교정된 수 (계약 규칙 d)
    pub case_fixed: usize,
    /// `%XX` 를 디코딩해야 해석된 수
    pub percent_decoded: usize,
    /// 저장소 전체에서 basename 이 유일해 복구된 수
    pub recovered_basename: usize,
    /// 확장자 교정으로 복구된 수
    pub ext_swap: usize,
    /// 해석 실패 목록 (`path:line -> target`)
    pub unresolved: Vec<String>,
    /// 복구 내역 (사람이 확인할 수 있게)
    pub recovered: Vec<String>,
}

impl Stats {
    fn record(&mut self, src: &str, line: usize, raw: &str, kind: &Kind) {
        match kind {
            Kind::Note(_) => self.note_links += 1,
            Kind::Section(_) => self.section_links += 1,
            Kind::Asset(_) => self.asset_links += 1,
            Kind::External => self.external += 1,
            Kind::Anchor => self.anchor += 1,
            Kind::Unresolved => {
                self.unresolved.push(format!("{}:{} -> {}", src, line, raw));
                return;
            }
        }
        if !matches!(kind, Kind::External | Kind::Anchor) {
            self.internal += 1;
        }
    }
}

/// 리포 안에서 노트가 사는 접두 디렉터리(예: `brains`)를 제거한다.
///
/// 사이트는 접두 없는 경로를 콘텐츠 루트로 본다 — 노트를 `brains/` 아래로 옮겨도
/// 공개 URL(`/<섹션>/<노트>.html`)이 바뀌지 않게 하는 장치다.
/// 접두가 없거나 일치하지 않으면 원본을 그대로 돌려준다(하위 호환).
pub fn strip_content_prefix(rel: &str, prefix: &str) -> String {
    let p = prefix.trim_matches('/');
    if p.is_empty() {
        return rel.to_string();
    }
    if rel == p {
        return String::new();
    }
    match rel.strip_prefix(p) {
        Some(rest) if rest.starts_with('/') => rest.trim_start_matches('/').to_string(),
        _ => rel.to_string(),
    }
}

/// 링크 해석기. 노트/파일/디렉터리 인덱스를 담고 있다.
pub struct Resolver {
    /// 노트가 사는 접두 디렉터리 (빈 문자열이면 없음)
    pub content_prefix: String,
    /// 노트의 저장소 상대 경로 (정렬, NFC) — notes 인덱스와 동일 순서
    pub notes: Vec<String>,
    /// 노트의 스테이지 상대 경로 (index.md → _index.md)
    pub staged_notes: Vec<String>,
    /// 노트 페이지의 사이트 루트 기준 구성요소
    pub page_notes: Vec<Vec<String>>,
    /// 스테이지에 복사되는 모든 파일 (정렬, NFC)
    pub files: Vec<String>,
    /// 모든 디렉터리 (정렬, NFC)
    pub dirs: Vec<String>,
    /// index.md 를 가진 디렉터리 (섹션 페이지가 생기는 곳)
    index_dirs: Vec<String>,
    lower_notes: Vec<(String, usize)>,
    lower_files: Vec<(String, usize)>,
    lower_dirs: Vec<(String, usize)>,
    lower_basenames: Vec<(String, usize)>,
}

fn lower_sorted<I: Iterator<Item = (String, usize)>>(it: I) -> Vec<(String, usize)> {
    let mut v: Vec<(String, usize)> = it.collect();
    v.sort();
    v
}

pub fn staged_path_for_note(rel: &str) -> String {
    let b = basename(rel);
    if b == "index.md" {
        // Hugo 는 index.md 를 leaf bundle 로 취급해 형제 노트를 흡수한다.
        // 스테이지에서만 _index.md 로 바꿔 섹션 페이지로 만든다.
        match dirname(rel) {
            "" => "_index.md".to_string(),
            d => format!("{}/_index.md", d),
        }
    } else {
        rel.to_string()
    }
}

pub fn page_components_for_note(staged: &str) -> Vec<String> {
    let b = basename(staged);
    let file = if b == "_index.md" {
        "index.html".to_string()
    } else {
        format!("{}.html", &b[..b.len() - 3])
    };
    let mut v: Vec<String> = if dirname(staged).is_empty() {
        Vec::new()
    } else {
        dirname(staged).split('/').map(|s| s.to_string()).collect()
    };
    v.push(file);
    v
}

impl Resolver {
    pub fn new(notes: &[String], files: &[String], dirs: &[String], content_prefix: &str) -> Self {
        let staged_notes: Vec<String> = notes.iter().map(|n| staged_path_for_note(n)).collect();
        let page_notes: Vec<Vec<String>> = staged_notes
            .iter()
            .map(|s| page_components_for_note(s))
            .collect();

        let lower_notes =
            lower_sorted(notes.iter().enumerate().map(|(i, n)| (n.to_lowercase(), i)));
        let lower_files =
            lower_sorted(files.iter().enumerate().map(|(i, f)| (f.to_lowercase(), i)));
        let lower_dirs = lower_sorted(dirs.iter().enumerate().map(|(i, d)| (d.to_lowercase(), i)));
        let mut basenames: Vec<(String, usize)> = files
            .iter()
            .enumerate()
            .map(|(i, f)| (basename(f).to_lowercase(), i))
            .collect();
        basenames.sort();

        let index_dirs: Vec<String> = dirs
            .iter()
            .filter(|d| {
                let probe = format!("{}/index.md", d);
                files
                    .binary_search_by(|f| f.as_str().cmp(probe.as_str()))
                    .is_ok()
            })
            .cloned()
            .collect();

        Self {
            content_prefix: content_prefix.trim_matches('/').to_string(),
            notes: notes.to_vec(),
            staged_notes,
            page_notes,
            files: files.to_vec(),
            dirs: dirs.to_vec(),
            index_dirs,
            lower_notes,
            lower_files,
            lower_dirs,
            lower_basenames: basenames,
        }
    }

    fn lookup(v: &[(String, usize)], key: &str) -> Option<usize> {
        v.binary_search_by(|(k, _)| k.as_str().cmp(key))
            .ok()
            .map(|i| v[i].1)
    }

    fn contains(v: &[String], key: &str) -> bool {
        v.binary_search_by(|f| f.as_str().cmp(key)).is_ok()
    }

    /// 노트 경로 → (인덱스, 대소문자만 달랐는지)
    pub fn note_of(&self, path: &str) -> Option<(usize, bool)> {
        if let Ok(i) = self.notes.binary_search_by(|n| n.as_str().cmp(path)) {
            return Some((i, false));
        }
        Self::lookup(&self.lower_notes, &path.to_lowercase()).map(|i| (i, true))
    }

    /// 파일 경로 → (파일 인덱스, 대소문자만 달랐는지)
    pub fn file_of(&self, path: &str) -> Option<(usize, bool)> {
        if let Ok(i) = self.files.binary_search_by(|f| f.as_str().cmp(path)) {
            return Some((i, false));
        }
        Self::lookup(&self.lower_files, &path.to_lowercase()).map(|i| (i, true))
    }

    /// 디렉터리 경로 → (정규 이름, 대소문자만 달랐는지)
    pub fn dir_of(&self, path: &str) -> Option<(String, bool)> {
        if Self::contains(&self.dirs, path) {
            return Some((path.to_string(), false));
        }
        Self::lookup(&self.lower_dirs, &path.to_lowercase()).map(|i| (self.dirs[i].clone(), true))
    }

    /// 디렉터리 타깃을 섹션 페이지로 해석한다.
    ///
    /// Hugo 는 톱레벨 디렉터리에만 섹션 페이지를 만들고(실측: out/MSA/pattern/index.html 없음),
    /// 중첩 디렉터리에는 index.md 가 있을 때만 만든다. 그래서 자기 자신부터 위로 올라가며
    /// "섹션 페이지가 실제로 존재하는" 가장 가까운 조상을 돌려준다.
    pub fn section_for_dir(&self, dir: &str) -> Option<String> {
        let mut cur = dir.to_string();
        loop {
            if self.index_dirs.binary_search(&cur).is_ok() || !cur.contains('/') {
                return Some(cur);
            }
            let parent = dirname(&cur).to_string();
            if parent.is_empty() {
                return None;
            }
            cur = parent;
        }
    }

    /// 저장소 전체에서 basename 이 유일한 파일을 찾는다.
    fn basename_unique(&self, path: &str) -> Option<usize> {
        let key = basename(path).to_lowercase();
        let lo = self
            .lower_basenames
            .partition_point(|(k, _)| k.as_str() < key.as_str());
        let hi = self
            .lower_basenames
            .partition_point(|(k, _)| k.as_str() <= key.as_str());
        if hi - lo == 1 {
            Some(self.lower_basenames[lo].1)
        } else {
            None
        }
    }

    /// 저장소 상대 경로의 대상 구성요소(사이트 루트 기준)를 만든다.
    pub fn url_components(&self, kind: &Kind) -> Vec<String> {
        match kind {
            Kind::Note(i) => self.page_notes[*i].clone(),
            Kind::Section(dir) => {
                let mut v: Vec<String> = dir.split('/').map(|s| s.to_string()).collect();
                v.push("index.html".to_string());
                v
            }
            Kind::Asset(p) => p.split('/').map(|s| s.to_string()).collect(),
            _ => Vec::new(),
        }
    }

    /// 링크 목적지를 저장소 기준으로 해석한다.
    pub fn resolve(&self, raw: &str, src_rel: &str, stats: &mut Stats) -> Kind {
        if raw.is_empty() || raw.starts_with('#') {
            return Kind::Anchor;
        }
        if raw.starts_with("data:") || raw.starts_with("mailto:") || raw.starts_with("tel:") {
            return Kind::External;
        }

        // 프래그먼트는 유지하고 경로만 쓴다.
        let (mut path_part, _frag) = match raw.find('#') {
            Some(i) => (&raw[..i], Some(&raw[i..])),
            None => (raw, None),
        };

        let repo_path: String;
        if path_part.starts_with("http://")
            || path_part.starts_with("https://")
            || path_part.starts_with("//")
        {
            const MARKER: &str = "github.com/rlaope/estudy/blob/master/";
            match path_part.find(MARKER) {
                Some(i) => {
                    let mut rest = &path_part[i + MARKER.len()..];
                    if let Some(q) = rest.find('?') {
                        rest = &rest[..q];
                    }
                    repo_path = normalize_rel_path(&percent_decode(rest));
                }
                None => return Kind::External,
            }
        } else {
            if let Some(q) = path_part.find('?') {
                path_part = &path_part[..q];
            }
            let decoded = percent_decode(path_part);
            let joined = if decoded.starts_with('/') {
                decoded.trim_start_matches('/').to_string()
            } else if dirname(src_rel).is_empty() {
                decoded
            } else {
                format!("{}/{}", dirname(src_rel), decoded)
            };
            repo_path = normalize_rel_path(&joined);
        }

        if repo_path.is_empty() {
            // 저장소 루트 (Hugo 홈) 로 보낸다.
            return Kind::Anchor;
        }
        // 리포 절대 링크(`blob/master/brains/...`)도 접두를 벗겨 콘텐츠 루트 기준으로 해석한다.
        let repo_path = nfc(&strip_content_prefix(&repo_path, &self.content_prefix));

        // (1) 노트 정확 일치 → (d) 대소문자 무시 폴백
        if let Some((i, case)) = self.note_of(&repo_path) {
            if case {
                stats.case_fixed += 1;
            }
            return Kind::Note(i);
        }
        // (2) 디렉터리 타깃 → 그 디렉터리의 섹션 페이지
        if let Some((dir, case)) = self.dir_of(repo_path.trim_end_matches('/')) {
            if case {
                stats.case_fixed += 1;
            }
            if let Some(sec) = self.section_for_dir(&dir) {
                return Kind::Section(sec);
            }
        }
        // (3) 그 밖의 저장소 파일 (이미지 / ipynb 등) — 사이트 안에 머무르게 한다.
        if let Some((i, case)) = self.file_of(&repo_path) {
            if case {
                stats.case_fixed += 1;
            }
            let f = self.files[i].clone();
            if let Some((ni, _)) = self.note_of(&f) {
                return Kind::Note(ni);
            }
            return Kind::Asset(f);
        }
        // (4) 저장소 전체에서 basename 이 유일한 파일로 복구 (원문이 옮겨진 자산을 가리키는 경우)
        if let Some(i) = self.basename_unique(&repo_path) {
            stats.recovered_basename += 1;
            let f = self.files[i].clone();
            if let Some((ni, _)) = self.note_of(&f) {
                return Kind::Note(ni);
            }
            return Kind::Asset(f);
        }
        // (5) 확장자 오타 교정 (예: paxra.m → paxra.md)
        let ext = extension(&repo_path);
        if !ext.is_empty() && ext != ".md" {
            let swapped = with_extension(&repo_path, ".md");
            if let Some((i, _)) = self.note_of(&swapped) {
                stats.ext_swap += 1;
                return Kind::Note(i);
            }
        }
        Kind::Unresolved
    }
}

/// 원본 페이지 기준 상대 URL 을 만든다.
pub fn relative_url(from_page: &[String], to: &[String], fragment: &str) -> String {
    let from_dir: &[String] = if from_page.is_empty() {
        &[]
    } else {
        &from_page[..from_page.len() - 1]
    };
    let to_dir_len = to.len().saturating_sub(1);
    let mut common = 0;
    while common < from_dir.len() && common < to_dir_len && from_dir[common] == to[common] {
        common += 1;
    }
    let mut parts: Vec<String> = Vec::new();
    for _ in common..from_dir.len() {
        parts.push("..".to_string());
    }
    for c in &to[common..] {
        parts.push(encode_path_component(c));
    }
    let mut s = if parts.is_empty() {
        ".".to_string()
    } else {
        parts.join("/")
    };
    s.push_str(fragment);
    s
}

/// 코드펜스 범위(바이트 오프셋)를 계산한다. 시작 오프셋 목록과 함께 (start, end) 쌍을 돌려준다.
fn fence_ranges(text: &str) -> Vec<(usize, usize)> {
    let mut ranges = Vec::new();
    let mut open: Option<usize> = None;
    let mut off = 0usize;
    for line in text.split_inclusive('\n') {
        let trimmed = line.trim_start();
        if trimmed.starts_with("```") || trimmed.starts_with("~~~") {
            match open {
                None => open = Some(off),
                Some(start) => {
                    ranges.push((start, off + line.len()));
                    open = None;
                }
            }
        }
        off += line.len();
    }
    if let Some(start) = open {
        ranges.push((start, text.len()));
    }
    ranges
}

fn line_index(line_starts: &[usize], pos: usize) -> usize {
    match line_starts.binary_search(&pos) {
        Ok(i) => i,
        Err(i) => i.saturating_sub(1),
    }
}

/// 노트 본문 하나를 재작성한다: 링크 목적지를 스테이지 페이지 URL 로 바꾸고 기록을 남긴다.
pub fn rewrite_note(
    content: &str,
    src_note: usize,
    resolver: &Resolver,
    stats: &mut Stats,
) -> (String, Vec<LinkRecord>) {
    let bytes = content.as_bytes();
    let fences = fence_ranges(content);

    let mut line_starts: Vec<usize> = vec![0];
    for (i, b) in bytes.iter().enumerate() {
        if *b == b'\n' {
            line_starts.push(i + 1);
        }
    }

    let from_page = &resolver.page_notes[src_note];
    let src_rel = &resolver.notes[src_note];

    struct Edit {
        start: usize,
        end: usize,
        replacement: String,
    }
    let mut edits: Vec<Edit> = Vec::new();
    let mut records: Vec<LinkRecord> = Vec::new();

    let mut i = 0usize;
    while i + 1 < bytes.len() {
        if bytes[i] != b']' || bytes[i + 1] != b'(' {
            i += 1;
            continue;
        }
        let mut j = i + 2;
        while j < bytes.len() && (bytes[j] == b' ' || bytes[j] == b'\t') {
            j += 1;
        }
        if j >= bytes.len() {
            break;
        }
        let (url_start, url_end, next) = if bytes[j] == b'<' {
            match content[j + 1..].find('>') {
                Some(k) => (j + 1, j + 1 + k, j + 1 + k + 1),
                None => break,
            }
        } else {
            let mut k = j;
            while k < bytes.len() && !matches!(bytes[k], b' ' | b'\t' | b'\n' | b'\r' | b')') {
                k += 1;
            }
            (j, k, k)
        };
        if url_end <= url_start {
            i = next;
            continue;
        }
        let raw = &content[url_start..url_end];
        stats.inline_total += 1;

        if fences
            .iter()
            .any(|(s, e)| url_start >= *s && url_start < *e)
        {
            stats.in_fence_skipped += 1;
            i = next;
            continue;
        }

        let rec_before = stats.recovered_basename;
        let ext_before = stats.ext_swap;
        let kind = resolver.resolve(raw, src_rel, stats);
        let used_recovery = stats.recovered_basename > rec_before || stats.ext_swap > ext_before;
        let line = line_index(&line_starts, url_start) + 1;
        let line_text = {
            let ls = line_starts[line - 1];
            let le = content[ls..]
                .find('\n')
                .map(|k| ls + k)
                .unwrap_or(content.len());
            &content[ls..le]
        };
        let indent = line_text
            .chars()
            .take_while(|c| *c == ' ' || *c == '\t')
            .count();
        let rest = line_text.trim_start();
        let bullet = rest.starts_with("- ") || rest.starts_with("* ") || rest.starts_with("+ ");

        stats.record(src_rel, line, raw, &kind);

        if !matches!(kind, Kind::External | Kind::Anchor | Kind::Unresolved) {
            let to = resolver.url_components(&kind);
            let frag = match raw.find('#') {
                Some(k) => &raw[k..],
                None => "",
            };
            let new_raw = relative_url(from_page, &to, frag);
            // 이미 스테이지 안을 가리키는 상대 링크(예: `./image/x.png`)는 의미가 같으므로
            // 손대지 않는다 — 스테이지 diff 를 실제 수정분만 남기기 위해서다.
            let raw_path = match raw.find('#') {
                Some(k) => &raw[..k],
                None => raw,
            };
            let is_abs_url =
                raw.starts_with("http://") || raw.starts_with("https://") || raw.starts_with("//");
            let equivalent =
                !is_abs_url && format!("{}{}", normalize_rel_path(raw_path), frag) == new_raw;
            if new_raw != raw && !equivalent {
                edits.push(Edit {
                    start: url_start,
                    end: url_end,
                    replacement: new_raw.clone(),
                });
                stats.rewritten += 1;
                if used_recovery && stats.recovered.len() < 64 {
                    stats
                        .recovered
                        .push(format!("{}:{} -> {}  =>  {}", src_rel, line, raw, new_raw));
                }
            }
        }

        records.push(LinkRecord {
            line,
            indent,
            bullet,
            raw: raw.to_string(),
            kind,
        });
        i = next;
    }

    if edits.is_empty() {
        return (content.to_string(), records);
    }

    let mut out = String::with_capacity(content.len() + 64);
    let mut cursor = 0usize;
    for e in &edits {
        out.push_str(&content[cursor..e.start]);
        out.push_str(&e.replacement);
        cursor = e.end;
    }
    out.push_str(&content[cursor..]);
    (out, records)
}

/// 스테이지 트리에 남아 있는 github blob URL 개수 (검증용).
pub fn count_gh_blob(text: &str) -> usize {
    text.match_indices("github.com/rlaope/estudy/blob/master/")
        .count()
}

/// 중복 제거된 정렬 집합 → Vec
pub fn sorted_vec(set: BTreeSet<(u32, u32)>) -> Vec<(u32, u32)> {
    set.into_iter().collect()
}
