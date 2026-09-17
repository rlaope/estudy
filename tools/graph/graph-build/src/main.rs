//! graph-build CLI.
//!
//! 사용:
//!   cargo run -p graph-build --release -- --root . --out site/generated --stage .cache/content
//!
//! 옵션:
//!   --root <dir>        리포 루트 (기본 `.`)
//!   --out <dir>         동결 산출물 출력 디렉터리 (기본 `site/generated`)
//!   --stage <dir>       Hugo 가 contentDir 로 쓸 스테이지 트리 (기본 `.cache/content`)
//!   --base-path <path>  사이트 루트 경로 (기본 `/estudy/`)
//!   --seed <u64>        레이아웃 시드 (기본 20260917)
//!   --iterations <n>    ForceAtlas2 반복 횟수 (기본 300)

use std::path::PathBuf;
use std::process::ExitCode;

use graph_core::{layout, run, Options};

fn parse_u64(s: &str, flag: &str) -> Result<u64, String> {
    s.parse::<u64>()
        .map_err(|e| format!("{} 값이 정수가 아니다: {:?} ({})", flag, s, e))
}

fn main() -> ExitCode {
    let mut opts = Options::default();
    let args: Vec<String> = std::env::args().skip(1).collect();
    let mut i = 0usize;

    while i < args.len() {
        let a = args[i].as_str();
        let take = |name: &str| -> Result<String, String> {
            if i + 1 >= args.len() {
                return Err(format!("{} 뒤에 값이 필요하다", name));
            }
            Ok(args[i + 1].clone())
        };
        let r = match a {
            "--root" => take(a).map(|v| opts.root = PathBuf::from(v)),
            "--out" => take(a).map(|v| opts.out = PathBuf::from(v)),
            "--stage" => take(a).map(|v| opts.stage = PathBuf::from(v)),
            "--base-path" => take(a).map(|v| opts.base_path = v),
            "--seed" => take(a).and_then(|v| parse_u64(&v, "--seed").map(|n| opts.seed = n)),
            "--iterations" => take(a)
                .and_then(|v| parse_u64(&v, "--iterations").map(|n| opts.iterations = n as usize)),
            "-h" | "--help" => {
                println!(
                    "graph-build --root <dir> --out <dir> --stage <dir>\n\
                     \x20 [--base-path /estudy/] [--seed 20260917] [--iterations 300]"
                );
                return ExitCode::SUCCESS;
            }
            other => Err(format!("알 수 없는 인자: {}", other)),
        };
        if let Err(e) = r {
            eprintln!("graph-build: {}", e);
            return ExitCode::from(2);
        }
        i += if a == "-h" || a == "--help" { 1 } else { 2 };
    }

    if opts.iterations == 0 {
        opts.iterations = layout::DEFAULT_ITERATIONS;
    }

    match run(&opts) {
        Ok(r) => {
            println!("[L1] 산출물: pos.bin {} B / graph.bin {} B / search.json {} B / meta.json {} B / CONTRACT.md {} B",
                r.pos_bytes, r.graph_bytes, r.search_bytes, r.meta_bytes, r.contract_bytes);
            println!(
                "[L1] 결정론(단일 프로세스 2회 대조): pos.bin {} · graph.bin {} · search.json {}",
                if r.determinism_pos {
                    "동일"
                } else {
                    "불일치"
                },
                if r.determinism_graph {
                    "동일"
                } else {
                    "불일치"
                },
                if r.determinism_search {
                    "동일"
                } else {
                    "불일치"
                }
            );
            println!(
                "[L1] 완료: 노트 {} 개 / 노드 {} / 엣지(양방향 저장) {} / 미해석 링크 {} 개",
                r.notes_scanned,
                r.nodes,
                r.csr_targets,
                r.stats.unresolved.len()
            );
            if !r.stats.unresolved.is_empty() || r.gh_blob_left != 0 {
                eprintln!("[L1] 실패: 미해석 링크 또는 변환되지 않은 링크가 남아 있다");
                return ExitCode::from(1);
            }
            ExitCode::SUCCESS
        }
        Err(e) => {
            eprintln!("[L1] 실패: {}", e);
            ExitCode::from(1)
        }
    }
}
