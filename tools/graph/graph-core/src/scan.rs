//! 리포 트리 스캔. 제외 규칙과 결정론적 정렬을 담당한다.

use std::path::{Path, PathBuf};

use crate::text::nfc;

/// 어떤 깊이에서든 이름이 일치하면 건너뛰는 디렉터리.
/// 앞의 5개는 계약이 정한 제외 목록(.git/.omh/site/.cache/public)이고,
/// 나머지는 Hugo 콘텐츠가 아니거나 L1 자신이 만든 빌드 산출물이다.
pub const EXCLUDED_DIRS: &[&str] = &[
    ".git",
    ".omh",
    "site",
    ".cache",
    "public",
    ".github",
    "tools",
    "target",
    "node_modules",
    ".cargo",
    // 영어 번역 트리 — KO 스캔에서 노트로 세면 안 된다(EN 스테이지에서 따로 스캔한다).
    "brains-en",
];

/// 리포 루트에서만 제외하는 파일 (cargo 매니페스트, Hugo 잠금 파일).
pub const EXCLUDED_ROOT_FILES: &[&str] = &[
    "Cargo.toml",
    "Cargo.lock",
    ".hugo_build.lock",
    // 영어 README — KO 사이트의 노트가 아니다(EN 스테이지에서 홈 색인 소스로만 쓴다).
    // 한국어 `README.md` 는 레포 기본 문서이자 KO 홈 색인의 출처(노트로도 포함된다).
    "README.en.md",
];

#[derive(Debug, Clone)]
pub struct Entry {
    /// 리포 기준 상대 경로 (NFC, `/` 구분자)
    pub rel: String,
    /// 실제 디스크 경로
    pub abs: PathBuf,
    pub size: u64,
}

#[derive(Debug, Default)]
pub struct Scan {
    /// 상대 경로 오름차순 정렬된 파일 목록 (NFC)
    pub files: Vec<Entry>,
    /// 노트(`.md`) 목록: files 중 .md 만, 상대 경로 오름차순
    pub notes: Vec<usize>,
    /// 디렉터리 상대 경로 (NFC, 오름차순)
    pub dirs: Vec<String>,
    /// 건너뛴 디렉터리 (리포 기준 경로)
    pub skipped: Vec<String>,
}

impl Scan {
    pub fn is_note(&self, rel: &str) -> bool {
        rel.ends_with(".md")
    }
}

fn rel_string(root: &Path, path: &Path) -> Result<String, String> {
    let rel = path
        .strip_prefix(root)
        .map_err(|e| format!("strip_prefix 실패: {} ({})", path.display(), e))?;
    let s = rel
        .to_str()
        .ok_or_else(|| format!("UTF-8 이 아닌 경로: {}", rel.display()))?;
    Ok(nfc(&s.replace('\\', "/")))
}

/// 리포 트리를 스캔한다. 결과는 항상 같은 순서(상대 경로 오름차순)로 정렬된다.
pub fn scan(root: &Path) -> Result<Scan, String> {
    let mut out = Scan::default();
    let mut stack: Vec<PathBuf> = vec![root.to_path_buf()];

    while let Some(dir) = stack.pop() {
        let mut entries: Vec<PathBuf> = Vec::new();
        let rd = std::fs::read_dir(&dir)
            .map_err(|e| format!("디렉터리 읽기 실패 {}: {}", dir.display(), e))?;
        for e in rd {
            let e = e.map_err(|e2| format!("read_dir 항목 실패 {}: {}", dir.display(), e2))?;
            entries.push(e.path());
        }

        for path in entries {
            let name = match path.file_name().and_then(|s| s.to_str()) {
                Some(n) => n.to_string(),
                None => return Err(format!("UTF-8 이 아닌 파일명: {}", path.display())),
            };
            let meta = std::fs::symlink_metadata(&path)
                .map_err(|e| format!("메타데이터 실패 {}: {}", path.display(), e))?;

            if meta.is_dir() {
                if EXCLUDED_DIRS.contains(&name.as_str()) {
                    out.skipped.push(rel_string(root, &path)?);
                    continue;
                }
                out.dirs.push(rel_string(root, &path)?);
                stack.push(path);
                continue;
            }

            // 심볼릭 링크/일반 파일 모두 파일로 취급. 루트 전용 제외 파일은 이름으로 거른다.
            if dir == root && EXCLUDED_ROOT_FILES.contains(&name.as_str()) {
                continue;
            }
            if !meta.is_file() && !meta.file_type().is_symlink() {
                continue; // 소켓/파이프 등은 무시
            }

            let size = std::fs::metadata(&path).map(|m| m.len()).unwrap_or(0);
            out.files.push(Entry {
                rel: rel_string(root, &path)?,
                abs: path,
                size,
            });
        }
    }

    out.files.sort_by(|a, b| a.rel.cmp(&b.rel));
    out.dirs.sort();
    out.dirs.dedup();
    out.skipped.sort();
    out.notes = (0..out.files.len())
        .filter(|i| out.files[*i].rel.ends_with(".md"))
        .collect();
    Ok(out)
}
