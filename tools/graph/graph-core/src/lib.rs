//! L1 content-pipeline 코어.
//!
//! 파이프라인 한 번 실행이 하는 일:
//!  1. 리포 스캔 (제외: `.git`/`.omh`/`site`/`.cache`/`public` 등, `.md` 만 노트로 취급)
//!  2. 스테이지 트리 생성 — 모든 파일 복사 + 노트 링크 정규화 + `DataBase/index.md` → `_index.md`
//!  3. 그래프 모델 (노트 + 섹션 허브 + 루트 허브) 과 CSR 인접 행렬
//!  4. `fa2`(MIT) ForceAtlas2 고정 시드/고정 반복 레이아웃
//!  5. 동결 산출물 4종 + `CONTRACT.md` 기록

pub mod contract;
pub mod graph;
pub mod json;
pub mod layout;
pub mod links;
pub mod related;
pub mod scan;
pub mod stage;
pub mod text;
pub mod timefmt;

use std::path::PathBuf;
use std::time::Instant;

use sha2::{Digest, Sha256};

/// CLI 옵션.
#[derive(Debug, Clone)]
pub struct Options {
    pub root: PathBuf,
    pub out: PathBuf,
    pub stage: PathBuf,
    /// 사이트 루트 기준 경로(앞뒤 슬래시 포함). GitHub Pages 프로젝트 사이트는 `/estudy/`.
    pub base_path: String,
    /// 레이아웃 방식: `spiral`(섹션 순서 나선, 기본) 또는 `fa2`(ForceAtlas2).
    pub layout_kind: String,
    /// 노트가 사는 리포 내 접두 디렉터리(예: `brains`). 사이트 URL 에서는 벗겨진다.
    pub content_prefix: String,
    /// 영어 번역 트리를 스테이지할 디렉터리(예: `.cache/content-en`). 없으면 건너뛴다.
    pub stage_en: Option<PathBuf>,
    pub seed: u64,
    pub iterations: usize,
}

impl Default for Options {
    fn default() -> Self {
        Self {
            root: PathBuf::from("."),
            out: PathBuf::from("site/generated"),
            stage: PathBuf::from(".cache/content"),
            base_path: "/estudy/".to_string(),
            layout_kind: "spiral".to_string(),
            content_prefix: "brains".to_string(),
            stage_en: None,
            seed: layout::DEFAULT_SEED,
            iterations: layout::DEFAULT_ITERATIONS,
        }
    }
}

/// 실행 결과 요약 (콘솔 출력 + CONTRACT.md 에 그대로 기록된다).
#[derive(Debug, Clone)]
pub struct Report {
    pub root: String,
    pub stage: String,
    pub out: String,
    pub base_path: String,
    pub files_scanned: usize,
    pub notes_scanned: usize,
    pub dirs_scanned: usize,
    pub excluded_dirs: Vec<String>,
    pub notes_staged: usize,
    pub files_copied: usize,
    pub gh_blob_left: usize,
    pub stats: links::Stats,
    pub nodes: usize,
    pub section_count: usize,
    pub edges_unique: usize,
    pub csr_targets: usize,
    pub node_hubs: usize,
    pub breakdown: graph::EdgeBreakdown,
    pub sections: Vec<(String, usize)>,
    pub seed: u64,
    pub iterations: usize,
    pub layout_seconds: f64,
    pub determinism_pos: bool,
    pub determinism_graph: bool,
    pub determinism_search: bool,
    pub determinism_related: bool,
    pub related_edges: usize,
    pub related_top_k: usize,
    pub related_min_score: f32,
    pub related_bytes: usize,
    pub excerpt_bytes: usize,
    pub determinism_excerpts: bool,
    pub pos_bounds: (f32, f32, f32, f32),
    pub root_sha256: String,
    pub built_at: String,
    pub titles_from_filename: usize,
    pub pos_bytes: usize,
    pub graph_bytes: usize,
    pub search_bytes: usize,
    pub meta_bytes: usize,
    pub contract_bytes: usize,
}

fn sha256_hex(bytes: &[u8]) -> String {
    let mut h = Sha256::new();
    h.update(bytes);
    let d = h.finalize();
    let mut s = String::with_capacity(64);
    for b in d {
        s.push_str(&format!("{:02x}", b));
    }
    s
}

/// 노트 제목: 첫 `# ` 제목. 없으면 파일명(확장자 제거).
fn note_title(content: &str, rel: &str) -> (String, bool) {
    for line in content.lines() {
        if let Some(rest) = line.strip_prefix("# ") {
            let t = text::nfc(rest.trim());
            if !t.is_empty() {
                return (t, false);
            }
        }
    }
    let b = text::basename(rel);
    let stem = if b.ends_with(".md") {
        &b[..b.len() - 3]
    } else {
        b
    };
    (text::nfc(stem), true)
}

fn base_path_clean(p: &str) -> String {
    let p = if p.starts_with('/') {
        p.to_string()
    } else {
        format!("/{}", p)
    };
    if p.ends_with('/') {
        p
    } else {
        format!("{}/", p)
    }
}

/// 파이프라인 본체.
pub fn run(opts: &Options) -> Result<Report, String> {
    let root_abs = stage::lexical_absolute(&opts.root);
    let stage_abs = if opts.stage.is_absolute() {
        stage::lexical_absolute(&opts.stage)
    } else {
        stage::lexical_absolute(&root_abs.join(&opts.stage))
    };
    let out_abs = if opts.out.is_absolute() {
        stage::lexical_absolute(&opts.out)
    } else {
        stage::lexical_absolute(&root_abs.join(&opts.out))
    };
    let base_path = base_path_clean(&opts.base_path);
    let prefix = opts.content_prefix.trim_matches('/').to_string();

    println!("[L1] root  = {}", root_abs.display());
    println!("[L1] stage = {}", stage_abs.display());
    println!("[L1] out   = {}", out_abs.display());
    println!("[L1] base  = {}", base_path);
    println!(
        "[L1] content prefix = {}",
        if prefix.is_empty() {
            "(없음)".to_string()
        } else {
            format!("{}/", prefix)
        }
    );

    // ── 1. 스캔 ────────────────────────────────────────────────────────────────
    let mut sc = scan::scan(&root_abs)?;
    // 콘텐츠 접두(`brains/`)를 벗겨 "콘텐츠 루트 기준" 경로로 정규화한다.
    // 리포에서 노트를 접두 아래로 옮겨도 사이트 URL·섹션·그래프 구조가 그대로 유지된다.
    // 접두를 벗기면 정렬 순서가 흐트러지므로(루트 README.md 가 뒤로 밀린다) 반드시 다시 정렬한다 —
    // 이분탐색으로 노트를 찾는 경로들이 전부 여기에 의존한다.
    if !prefix.is_empty() {
        for f in sc.files.iter_mut() {
            f.rel = links::strip_content_prefix(&f.rel, &prefix);
        }
        sc.files.sort_by(|a, b| a.rel.cmp(&b.rel));
        sc.notes = (0..sc.files.len())
            .filter(|i| sc.files[*i].rel.ends_with(".md"))
            .collect();
        let mut dirs: Vec<String> = sc
            .dirs
            .iter()
            .map(|d| links::strip_content_prefix(d, &prefix))
            .filter(|d| !d.is_empty())
            .collect();
        dirs.sort();
        dirs.dedup();
        sc.dirs = dirs;
    }
    let note_paths: Vec<String> = sc.notes.iter().map(|i| sc.files[*i].rel.clone()).collect();
    let file_paths: Vec<String> = sc.files.iter().map(|f| f.rel.clone()).collect();
    let content_dirs: Vec<String> = sc.dirs.clone();
    println!(
        "[L1] 스캔 완료: 파일 {} 개 / 그중 노트(.md) {} 개 / 디렉터리 {} 개 / 제외 디렉터리 {} 개",
        sc.files.len(),
        note_paths.len(),
        sc.dirs.len(),
        sc.skipped.len()
    );

    // ── 2. 루트 지문 (입력 파일 전체 매니페스트 sha256) ────────────────────────
    let mut manifest = Sha256::new();
    for f in &sc.files {
        let bytes =
            std::fs::read(&f.abs).map_err(|e| format!("읽기 실패 {}: {}", f.abs.display(), e))?;
        manifest.update(f.rel.as_bytes());
        manifest.update([0u8]);
        manifest.update(bytes.len().to_string().as_bytes());
        manifest.update([0u8]);
        manifest.update(sha256_hex(&bytes).as_bytes());
        manifest.update(b"\n");
    }
    let root_sha256 = {
        let d = manifest.finalize();
        let mut s = String::with_capacity(64);
        for b in d {
            s.push_str(&format!("{:02x}", b));
        }
        s
    };

    let resolver = links::Resolver::new(&note_paths, &file_paths, &content_dirs, &prefix);

    // ── 3. 스테이지 트리 ───────────────────────────────────────────────────────
    stage::prepare_stage(&stage_abs, &root_abs)?;

    let mut records: Vec<Vec<links::LinkRecord>> = Vec::with_capacity(note_paths.len());
    let mut titles: Vec<String> = Vec::with_capacity(note_paths.len());
    // 관련 엣지용 본문 토큰(노트 순서 = id 순서)
    let mut note_tokens: Vec<Vec<String>> = Vec::with_capacity(note_paths.len());
    // 사이드바 미리보기용 발췌(노트 순서 = id 순서)
    let mut excerpts: Vec<String> = Vec::with_capacity(note_paths.len());
    let mut stats = links::Stats::default();
    let mut gh_blob_left = 0usize;
    let mut titles_from_filename = 0usize;

    for (i, entry_idx) in sc.notes.iter().enumerate() {
        let e = &sc.files[*entry_idx];
        let bytes = std::fs::read(&e.abs)
            .map_err(|err| format!("읽기 실패 {}: {}", e.abs.display(), err))?;
        let content = String::from_utf8(bytes)
            .map_err(|err| format!("UTF-8 디코딩 실패 {}: {}", e.rel, err))?;
        let (title, fallback) = note_title(&content, &e.rel);
        if fallback {
            titles_from_filename += 1;
        }
        titles.push(title);

        let (new_content, recs) = links::rewrite_note(&content, i, &resolver, &mut stats);
        note_tokens.push(related::tokens(&content, &e.rel));
        excerpts.push(text::excerpt(&content, 140));
        gh_blob_left += links::count_gh_blob(&new_content);

        let staged = &resolver.staged_notes[i];
        let dst = stage::stage_path(&stage_abs, staged)?;
        stage::write_bytes(&dst, new_content.as_bytes())?;
        records.push(recs);
    }
    let notes_staged = titles.len();

    let mut files_copied = 0usize;
    for e in &sc.files {
        if e.rel.ends_with(".md") {
            continue; // 노트는 위에서 재작성해 이미 썼다
        }
        let rel = &e.rel;
        if rel.is_empty() {
            continue;
        }
        let dst = stage::stage_path(&stage_abs, rel)?;
        stage::copy_file(&e.abs, &dst)?;
        files_copied += 1;
    }
    println!(
        "[L1] 스테이지 생성: 노트 {} 개 기록 + 그 외 파일 {} 개 복사",
        notes_staged, files_copied
    );
    println!(
        "[L1] 링크: 스캔 {} 개 / 내부 해석 {} 개 (노트 {} · 섹션 {} · 자산 {}) / 재작성 {} 개 / 대소문자 교정 {} / basename 복구 {} / 확장자 교정 {} / 미해석 {} 개",
        stats.inline_total,
        stats.internal,
        stats.note_links,
        stats.section_links,
        stats.asset_links,
        stats.rewritten,
        stats.case_fixed,
        stats.recovered_basename,
        stats.ext_swap,
        stats.unresolved.len()
    );
    if gh_blob_left == 0 {
        println!("[L1] 스테이지에 남은 github blob URL: 0 개");
    } else {
        println!(
            "[L1] 경고: 스테이지에 남은 github blob URL: {} 개",
            gh_blob_left
        );
    }
    if !stats.unresolved.is_empty() {
        println!("[L1] 미해석 링크 목록:");
        for u in stats.unresolved.iter().take(50) {
            println!("      - {}", u);
        }
    }

    // ── 3b. EN 스테이지 트리 (brains-en → .cache/content-en) ──────────────────
    // 번역 트리는 같은 상대 경로를 유지하므로 KO 와 동일한 링크 재작성기를 그대로 쓴다.
    // 자산(.md 이외: 이미지 등)은 KO 트리에서 복사한다 — EN 트리에는 번역문만 둔다.
    if let Some(en_stage_arg) = opts.stage_en.clone() {
        let en_root = root_abs.join("brains-en");
        if !en_root.is_dir() {
            println!("[L1] EN 스테이지: brains-en 없음 → 건너뜀");
        } else {
            let en_stage_abs = if en_stage_arg.is_absolute() {
                stage::lexical_absolute(&en_stage_arg)
            } else {
                stage::lexical_absolute(&root_abs.join(&en_stage_arg))
            };
            let mut en_sc = scan::scan(&en_root)?;
            for f in en_sc.files.iter_mut() {
                f.rel = links::strip_content_prefix(&f.rel, &prefix);
            }
            en_sc.files.sort_by(|a, b| a.rel.cmp(&b.rel));
            en_sc.notes = (0..en_sc.files.len())
                .filter(|i| en_sc.files[*i].rel.ends_with(".md"))
                .collect();
            let mut en_dirs: Vec<String> = en_sc
                .dirs
                .iter()
                .map(|d| links::strip_content_prefix(d, &prefix))
                .filter(|d| !d.is_empty())
                .collect();
            en_dirs.sort();
            en_dirs.dedup();
            let en_notes: Vec<String> = en_sc.notes.iter().map(|i| en_sc.files[*i].rel.clone()).collect();
            // 자산(.md 이외)은 KO 트리에서 복사하므로, 리졸버 파일 목록에도 넣어야
            // `image/foo.png` 같은 상대 링크가 미해석으로 세어지지 않는다.
            let mut en_files: Vec<String> = en_sc.files.iter().map(|f| f.rel.clone()).collect();
            for e in &sc.files {
                if !e.rel.ends_with(".md") && !e.rel.is_empty() {
                    en_files.push(e.rel.clone());
                }
            }
            en_files.sort();
            en_files.dedup();
            let en_resolver = links::Resolver::new(&en_notes, &en_files, &en_dirs, &prefix);
            stage::prepare_stage(&en_stage_abs, &en_root)?;
            let mut en_stats = links::Stats::default();
            for (i, entry_idx) in en_sc.notes.iter().enumerate() {
                let e = &en_sc.files[*entry_idx];
                let bytes = std::fs::read(&e.abs)
                    .map_err(|err| format!("읽기 실패 {}: {}", e.abs.display(), err))?;
                let content = String::from_utf8(bytes)
                    .map_err(|err| format!("UTF-8 디코딩 실패 {}: {}", e.rel, err))?;
                let (new_content, _) = links::rewrite_note(&content, i, &en_resolver, &mut en_stats);
                let staged = &en_resolver.staged_notes[i];
                let dst = stage::stage_path(&en_stage_abs, staged)?;
                stage::write_bytes(&dst, new_content.as_bytes())?;
            }
            let mut en_assets = 0usize;
            for e in &sc.files {
                if e.rel.ends_with(".md") || e.rel.is_empty() {
                    continue;
                }
                let dst = stage::stage_path(&en_stage_abs, &e.rel)?;
                stage::copy_file(&e.abs, &dst)?;
                en_assets += 1;
            }
            println!(
                "[L1] EN 스테이지: 노트 {} 개 + 자산 {} 개 → {} (미해석 링크 {} 개)",
                en_notes.len(),
                en_assets,
                en_stage_abs.display(),
                en_stats.unresolved.len()
            );
        }
    }

    // ── 4. 그래프 ──────────────────────────────────────────────────────────────
    let g = graph::build(&note_paths, &records);
    let graph_bytes = g.csr_bytes();
    let graph_bytes2 = g.csr_bytes();
    let determinism_graph = graph_bytes == graph_bytes2;
    println!(
        "[L1] 그래프: 노드 {} (노트 {} + 섹션 허브 {} + 루트 허브 1) / 무방향 엣지 {} / CSR 타깃 {}",
        g.node_count,
        g.note_count,
        g.section_hubs.len(),
        g.edges.len(),
        g.targets.len()
    );

    // ── 4b. 관련 엣지(본문 토큰 코사인 유사 · 명시 링크 제외) ───────────────────
    let rel_pairs = related::build(
        note_paths.len(),
        &note_tokens,
        &g.edges,
        related::TOP_K,
        related::MIN_SCORE,
    );
    let rel_pairs_b = related::build(
        note_paths.len(),
        &note_tokens,
        &g.edges,
        related::TOP_K,
        related::MIN_SCORE,
    );
    let determinism_related = rel_pairs == rel_pairs_b;
    let related_bytes = related::to_le_bytes(&rel_pairs);
    let related_path = out_abs.join("related.bin");
    stage::write_bytes(&related_path, &related_bytes)?;
    println!(
        "[L1] 관련 엣지: {} 개 (노트당 상위 {} · 임계 {:.2} · 명시 링크 쌍 제외)",
        rel_pairs.len(),
        related::TOP_K,
        related::MIN_SCORE
    );

    // ── 5. 레이아웃 (고정 시드 · 고정 반복) ───────────────────────────────────
    // 나선 배치는 섹션 순서를 쓰므로 노트별 섹션 서수를 미리 계산한다.
    let section_of: Vec<u32> = {
        let mut names: Vec<&str> = note_paths.iter().map(|p| graph::top_section(p)).collect();
        names.sort_unstable();
        names.dedup();
        let mut v: Vec<u32> = note_paths
            .iter()
            .map(|p| names.binary_search(&graph::top_section(p)).unwrap_or(0) as u32)
            .collect();
        // 섹션 허브는 자기 섹션 블록 안에 놓는다(허브가 중앙에 몰리지 않게). 루트 허브는 0.
        for (name, _) in g.sections.iter() {
            let ord = names.binary_search(&name.as_str()).unwrap_or(0) as u32;
            v.push(ord);
        }
        while v.len() < g.node_count {
            v.push(0);
        }
        v.truncate(g.node_count);
        v
    };
    let compute = |seed: u64| -> Vec<(f32, f32)> {
        match opts.layout_kind.as_str() {
            "fa2" => layout::force_atlas2(g.node_count, &g.edges, seed, opts.iterations),
            _ => layout::spiral_by_section(g.node_count, &section_of, seed),
        }
    };
    let t0 = Instant::now();
    let pos_a = compute(opts.seed);
    let layout_seconds = t0.elapsed().as_secs_f64();
    let pos_b = compute(opts.seed);
    let pos_bytes = layout::positions_to_le_bytes(&pos_a);
    let pos_bytes_b = layout::positions_to_le_bytes(&pos_b);
    let determinism_pos = pos_bytes == pos_bytes_b;

    let mut min_x = f32::INFINITY;
    let mut max_x = f32::NEG_INFINITY;
    let mut min_y = f32::INFINITY;
    let mut max_y = f32::NEG_INFINITY;
    for (x, y) in &pos_a {
        min_x = min_x.min(*x);
        max_x = max_x.max(*x);
        min_y = min_y.min(*y);
        max_y = max_y.max(*y);
    }
    println!(
        "[L1] 레이아웃: fa2 pairwise, seed={}, iterations={}, {} 초 (2회 실행 대조: {})",
        opts.seed,
        opts.iterations,
        format!("{:.2}", layout_seconds),
        if determinism_pos {
            "동일"
        } else {
            "불일치"
        }
    );

    // ── 6. 동결 산출물 ─────────────────────────────────────────────────────────
    let pos_path = out_abs.join("pos.bin");
    stage::write_bytes(&pos_path, &pos_bytes)?;

    let graph_path = out_abs.join("graph.bin");
    stage::write_bytes(&graph_path, &graph_bytes)?;

    let build_search = || {
        let mut search_arr = json::Arr::new();
        for i in 0..note_paths.len() {
            let page = &resolver.page_notes[i];
            let url = format!(
                "{}{}",
                base_path,
                page.iter()
                    .map(|c| text::encode_path_component(c))
                    .collect::<Vec<_>>()
                    .join("/")
            );
            let section = match graph::top_section(&note_paths[i]) {
                "" => "root".to_string(),
                s => s.to_string(),
            };
            let size = sc.files[sc.notes[i]].size;
            let degree = g.degrees[i];
            search_arr.push(format!(
                "{{\"id\":{},\"title\":{},\"path\":{},\"url\":{},\"section\":{},\"choseong\":{},\"size\":{},\"degree\":{}}}",
                i,
                json::string(&titles[i]),
                json::string(&note_paths[i]),
                json::string(&url),
                json::string(&section),
                json::string(&text::choseong(&titles[i])),
                size,
                degree
            ));
        }
        search_arr.finish()
    };
    let search_json = build_search();
    let search_json_b = build_search();
    let determinism_search = search_json == search_json_b;
    let search_path = out_abs.join("search.json");
    stage::write_bytes(&search_path, search_json.as_bytes())?;

    // ── 4c. 발췌(그래프 사이드바 미리보기) ─────────────────────────────────────
    let build_excerpts = || {
        let mut arr = json::Arr::new();
        for e in &excerpts {
            arr.push(json::string(e));
        }
        arr.finish()
    };
    let excerpts_json = build_excerpts();
    let excerpts_json_b = build_excerpts();
    let determinism_excerpts = excerpts_json == excerpts_json_b;
    let excerpts_path = out_abs.join("excerpts.json");
    stage::write_bytes(&excerpts_path, excerpts_json.as_bytes())?;

    let built_at = timefmt::now_iso8601_utc();
    let mut sec_arr = json::Arr::new();
    for (name, count) in &g.sections {
        sec_arr.push(format!(
            "{{\"name\":{},\"count\":{}}}",
            json::string(name),
            count
        ));
    }
    let meta_json = format!(
        "{{\n  \"nodes\": {},\n  \"edges\": {},\n  \"related_edges\": {},\n  \"related_top_k\": {},\n  \"related_min_score\": {},\n  \"sections\": {},\n  \"built_at\": {},\n  \"root_sha256\": {},\n  \"layout\": {},\n  \"seed\": {}\n}}\n",
        g.node_count,
        g.targets.len(),
        rel_pairs.len(),
        related::TOP_K,
        format!("{:.2}", related::MIN_SCORE),
        sec_arr.finish_pretty("    "),
        json::string(&built_at),
        json::string(&root_sha256),
        json::string(&opts.layout_kind),
        opts.seed
    );
    let meta_path = out_abs.join("meta.json");
    stage::write_bytes(&meta_path, meta_json.as_bytes())?;

    let mut report = Report {
        root: root_abs.display().to_string(),
        stage: stage_abs.display().to_string(),
        out: out_abs.display().to_string(),
        base_path: base_path.clone(),
        files_scanned: sc.files.len(),
        notes_scanned: note_paths.len(),
        dirs_scanned: sc.dirs.len(),
        excluded_dirs: sc.skipped.clone(),
        notes_staged,
        files_copied,
        gh_blob_left,
        stats: stats.clone(),
        nodes: g.node_count,
        section_count: g.sections.len(),
        edges_unique: g.edges.len(),
        csr_targets: g.targets.len(),
        node_hubs: g.section_hubs.len() + 1,
        breakdown: g.breakdown.clone(),
        sections: g.sections.clone(),
        seed: opts.seed,
        iterations: opts.iterations,
        layout_seconds,
        determinism_pos,
        determinism_graph,
        determinism_search,
        determinism_related,
        related_edges: rel_pairs.len(),
        related_top_k: related::TOP_K,
        related_min_score: related::MIN_SCORE,
        related_bytes: related_bytes.len(),
        excerpt_bytes: excerpts_json.len(),
        determinism_excerpts,
        pos_bounds: (min_x, max_x, min_y, max_y),
        root_sha256: root_sha256.clone(),
        built_at: built_at.clone(),
        titles_from_filename,
        pos_bytes: pos_bytes.len(),
        graph_bytes: graph_bytes.len(),
        search_bytes: search_json.len(),
        meta_bytes: meta_json.len(),
        contract_bytes: 0,
    };

    let contract_md = contract::render(&report);
    report.contract_bytes = contract_md.len();
    // contract_bytes 가 문서 본문에 들어가므로 최종 길이로 한 번 더 만든다.
    let contract_md = contract::render(&report);
    report.contract_bytes = contract_md.len();
    let contract_path = out_abs.join("CONTRACT.md");
    stage::write_bytes(&contract_path, contract_md.as_bytes())?;

    Ok(report)
}
