//! `site/generated/CONTRACT.md` 생성.

use crate::Report;

/// 산출물 형식 문서를 만든다. 구조 설명은 고정, 실측 수치는 실행 결과에서 채운다.
pub fn render(r: &Report) -> String {
    let s = &r.stats;
    let mut out = String::new();

    out.push_str("# site/generated 산출물 계약 (L1 content-pipeline)\n\n");
    out.push_str("이 디렉터리의 파일은 `tools/graph` 의 Rust 파이프라인이 **빌드타임에** 생성한다. 손으로 편집하지 말 것.\n\n");
    out.push_str("```\n");
    out.push_str("cargo run -p graph-build --release -- --root . --out site/generated --stage .cache/content\n");
    out.push_str("```\n\n");
    out.push_str(&format!(
        "- root: `{}`\n- stage: `{}`\n- out: `{}`\n- base path: `{}`\n\n",
        r.root, r.stage, r.out, r.base_path
    ));

    out.push_str("## 1. 파일\n\n");
    out.push_str("| 파일 | 형식 |\n| --- | --- |\n");
    out.push_str("| `pos.bin` | `f32` little-endian, `[x, y] * N` (노드 순서 = `search.json` 의 `id` 순서, 그 뒤에 허브 노드) |\n");
    out.push_str("| `graph.bin` | `u32` little-endian, `[offsets N+1][targets E]` (CSR) |\n");
    out.push_str("| `search.json` | JSON 배열 `[{\"id\",\"title\",\"path\",\"url\",\"section\",\"choseong\",\"size\",\"degree\"}]` |\n");
    out.push_str("| `meta.json` | JSON 객체 `{\"nodes\",\"edges\",\"sections\",\"built_at\",\"root_sha256\",\"layout\",\"seed\"}` |\n");
    out.push_str("| `CONTRACT.md` | 이 문서 |\n\n");

    out.push_str("## 2. 노드 / 엣지 모델\n\n");
    out.push_str(&format!(
        "- 노드 수 `N = {}` = 노트 `{}` + 섹션 허브 `{}` + 루트 허브 `1`.\n",
        r.nodes, r.notes_scanned, r.section_count
    ));
    out.push_str(&format!(
        "- node id `0..{}`: 노트. 정렬 기준은 **저장소 상대 경로(NFC) 오름차순(바이트 순)** 이며 `search.json` 의 `id` 와 완전히 같은 순서다.\n",
        r.notes_scanned.saturating_sub(1)
    ));
    out.push_str(&format!(
        "- node id `{}..{}`: 톱레벨 섹션 허브(디렉터리 이름 오름차순). node id `{}`: 루트 허브(가장 마지막).\n",
        r.notes_scanned,
        r.notes_scanned + r.section_count.saturating_sub(1),
        r.nodes.saturating_sub(1)
    ));
    out.push_str("- 엣지는 **무방향**이며 중복을 제거한다(작은 id가 앞).\n");
    out.push_str("- `graph.bin` 의 CSR 은 무방향 엣지를 **양방향으로 2번** 저장한다. 따라서 `targets` 길이 `= 2 * |엣지|` 이고 `meta.json` 의 `edges` 는 그 `targets` 길이(= `offsets[N]`)다. 노드 `v` 의 차수는 `offsets[v+1] - offsets[v]`.\n");
    out.push_str(&format!(
        "- 엣지 구성(중복 제거 후 실제 삽입 기준): 섹션 소속 {} + 루트↔섹션 {} + 노트↔노트 {} + README→노트 {} + README 부모→자식 {} + README→섹션 {} = **{} 무방향 엣지** (CSR 타깃 {}).\n\n",
        r.breakdown.section_membership,
        r.breakdown.root_to_section,
        r.breakdown.intra_note,
        r.breakdown.readme_direct,
        r.breakdown.readme_parent_child,
        r.breakdown.readme_to_section,
        r.edges_unique,
        r.csr_targets
    ));
    out.push_str("엣지 규칙(이 순서로 삽입 후 중복 제거):\n\n");
    out.push_str("1. **섹션 소속**: 노트 ↔ 그 노트의 톱레벨 섹션 허브. 루트에 있는 노트(`README.md`)는 루트 허브에 붙는다.\n");
    out.push_str("2. **루트 허브 ↔ 각 섹션 허브**.\n");
    out.push_str("3. **본문 내 노트 링크**: 노트 A의 링크가 노트 B로 해석되면 A ↔ B.\n");
    out.push_str("4. **README 직접 링크**: `README.md` 의 모든 내부 링크(디렉터리 타깃은 섹션 허브로) → README ↔ 대상.\n");
    out.push_str("5. **README 중첩 목록 계층**: 같은 목록에서 더 얕은 들여쓰기의 직전 항목이 부모다 → 부모 ↔ 자식. 부모 링크가 외부/미해석이면 그 위 조상에 붙는다.\n\n");

    out.push_str("## 3. 좌표 (pos.bin)\n\n");
    out.push_str(&format!(
        "- 알고리즘: `fa2` 크레이트(MIT)의 ForceAtlas2 — pairwise repulsion, `from_graph_order(N)`, `parallel(false)`.\n- seed: `{}` (splitmix64 로 초기 좌표 생성), iterations: `{}` (고정).\n",
        r.seed, r.iterations
    ));
    out.push_str("- 좌표 단위는 임의 단위다. 표시할 때 화면에 맞춰 fit 할 것.\n");
    out.push_str(&format!(
        "- 실측 범위: x [{:.3}, {:.3}] / y [{:.3}, {:.3}].\n",
        r.pos_bounds.0, r.pos_bounds.1, r.pos_bounds.2, r.pos_bounds.3
    ));
    out.push_str(
        "- 결정론: 같은 입력이면 `pos.bin` 이 바이트 단위로 동일하다(CI 해시 게이트 대상).\n\n",
    );

    out.push_str("## 4. search.json\n\n");
    out.push_str("| 키 | 뜻 |\n| --- | --- |\n");
    out.push_str("| `id` | 노드 id (`0..노트수-1`). `pos.bin` 의 같은 인덱스와 대응. |\n");
    out.push_str("| `title` | 본문 첫 `# ` 제목. 없으면 파일명(확장자 제거). NFC. |\n");
    out.push_str("| `path` | 저장소 기준 노트 상대 경로(스테이지가 아니라 원본 기준, NFC) |\n");
    out.push_str("| `url` | 사이트 루트 기준 페이지 URL (`base path` + 페이지 경로). 페이지 내 상대 링크와 달리 절대 형태다. |\n");
    out.push_str("| `section` | 톱레벨 섹션 이름. 루트 노트는 `root`. |\n");
    out.push_str("| `choseong` | `title` 의 한글 초성 문자열. 완성형 음절은 `(음절-0xAC00)/588` 로 초성 인덱스를 구하고, 비한글 문자는 그대로 둔다. |\n");
    out.push_str("| `size` | 노트 파일 크기(byte) |\n");
    out.push_str("| `degree` | 무방향 그래프에서의 이웃 수 |\n\n");
    out.push_str("배열 순서 = `id` 오름차순. 압축 없이 한 줄 JSON 이며 개행이 없다.\n\n");

    out.push_str("## 5. meta.json\n\n");
    out.push_str("- `nodes` / `edges`: 그래프 노드 수와 CSR 타깃 길이.\n");
    out.push_str(
        "- `sections`: `[{\"name\",\"count\"}]` — 톱레벨 섹션 이름(오름차순)과 노트 수.\n",
    );
    out.push_str("- `built_at`: 빌드 시각(UTC, ISO-8601). **비결정 필드**다(아래 해시 게이트 대상에서 제외).\n");
    out.push_str("- `root_sha256`: 입력 트리 지문. 스캔한 모든 파일에 대해 `path\\\\0size\\\\0sha256(content)\\\\n` 를 이어 붙여 sha256 한 값.\n");
    out.push_str("- `layout`: `\"fa2\"` 고정. `seed`: 레이아웃 시드.\n\n");

    out.push_str("## 6. 노트 링크 정규화 (스테이지 트리에만 적용)\n\n");
    out.push_str(
        "원본 `.md` 는 절대 수정하지 않는다. 스테이지 사본에서만 링크 목적지를 다시 쓴다.\n\n",
    );
    out.push_str("| 입력 | 출력 |\n| --- | --- |\n");
    out.push_str("| `https://github.com/rlaope/estudy/blob/master/<p>.md` | 그 노트의 스테이지 페이지로 가는 **상대 URL** |\n");
    out.push_str("| 상대 `foo.md`, `../x/foo.md` | 스테이지 페이지 상대 URL |\n");
    out.push_str("| 디렉터리 타깃 (`MSA/pattern`) | 가장 가까운 상위 **섹션 페이지** |\n");
    out.push_str("| 대소문자 불일치 (`interpreter.md`) | 실제 파일명으로 교정 (규칙 d) |\n");
    out.push_str("| 그 밖의 저장소 파일 (`*.png`, `*.ipynb`) | 스테이지 안 같은 경로 상대 URL(사이트 밖으로 나가지 않게) |\n");
    out.push_str("| 외부 URL, `#anchor`, `mailto:` | 손대지 않음 |\n\n");
    out.push_str("해석 순서: (1) `.md` 정확 일치 → (2) `.md` 대소문자 무시 일치 → (3) 디렉터리면 섹션 페이지 → (4) 파일 정확/대소문자 무시 일치 → (5) 저장소 전체에서 basename 이 유일한 파일 → (6) 확장자 교정(`paxra.m` → `paxra.md`). (5)(6) 은 원문이 옮겨졌거나 오타인 경우를 복구한다.\n\n");
    out.push_str("링크 URL 은 원본 페이지 기준 **상대 경로**로 쓴다(`../JAVA/jvm.html`). base path 가 `/` 든 `/estudy/` 든, `hugo server` 든 GitHub Pages 든 동일하게 동작하기 때문이다. 공백/`%`/`#`/`?`/괄호는 퍼센트 인코딩하고 한글은 그대로 둔다(NFC).\n\n");
    out.push_str("스테이지 규칙:\n\n");
    out.push_str("- 트리 구조를 그대로 복사한다(이미지 포함). 제외: `.git`, `.omh`, `site`, `.cache`, `public`, `.github`, `tools`, `target`, `.cargo`, 루트의 `Cargo.toml`/`Cargo.lock`.\n");
    out.push_str("- `DataBase/index.md` → `DataBase/_index.md` 로 이름만 바꾼다. Hugo 가 `index.md` 를 leaf bundle 로 보고 형제 노트 75개를 흡수하기 때문이다(실측).\n");
    out.push_str("- 노트의 경로 자체는 바꾸지 않는다(URL 안정성).\n\n");

    out.push_str("## 7. 결정론\n\n");
    out.push_str("`pos.bin` / `graph.bin` / `search.json` 은 **연속 2회 실행에서 바이트 단위로 동일**해야 한다.\n");
    out.push_str("보장 방법: 입력 스캔을 경로 오름차순으로 고정, HashMap 대신 정렬 Vec/BTreeSet 사용, fa2 순차 경로만 사용, 초기 좌표는 고정 시드 PRNG, 반복 횟수 고정. 시각/난수는 `meta.json` 의 `built_at` 에만 들어간다.\n\n");
    out.push_str("```\n");
    out.push_str("shasum -a 256 site/generated/pos.bin   # 두 번 실행해 비교\n```\n\n");

    out.push_str("## 8. 이번 실행 실측\n\n");
    out.push_str(&format!(
        "- 노트 {} 개(파일 {} 개, 디렉터리 {} 개) 스캔, 노트 {} 개 스테이징, 그 외 파일 {} 개 복사.\n",
        r.notes_scanned, r.files_scanned, r.dirs_scanned, r.notes_staged, r.files_copied
    ));
    out.push_str(&format!(
        "- 링크: 인라인 {} 개 중 내부 해석 {} 개(노트 {} · 섹션 {} · 자산 {}), 재작성 {} 개, 대소문자 교정 {} 개, basename 복구 {} 개, 확장자 교정 {} 개, **미해석 {} 개**. 스테이지에 남은 `github.com/rlaope/estudy/blob/master/` URL: {} 개.\n",
        s.inline_total,
        s.internal,
        s.note_links,
        s.section_links,
        s.asset_links,
        s.rewritten,
        s.case_fixed,
        s.recovered_basename,
        s.ext_swap,
        s.unresolved.len(),
        r.gh_blob_left
    ));
    out.push_str(&format!(
        "- 그래프: 노드 {} / 무방향 엣지 {} / CSR 타깃 {}.\n",
        r.nodes, r.edges_unique, r.csr_targets
    ));
    out.push_str(&format!(
        "- 레이아웃: {} 초 (2회 대조 {}). 그래프 재직렬화 대조 {}. search.json 재직렬화 대조 {}.\n",
        format!("{:.2}", r.layout_seconds),
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
    ));
    out.push_str(&format!(
        "- 파일 크기: pos.bin {} B, graph.bin {} B, search.json {} B, meta.json {} B, CONTRACT.md {} B.\n",
        r.pos_bytes, r.graph_bytes, r.search_bytes, r.meta_bytes, r.contract_bytes
    ));
    out.push_str(&format!(
        "- 제목이 파일명으로 대체된 노트: {} 개 (첫 `# ` 제목이 없는 경우).\n",
        r.titles_from_filename
    ));
    out.push_str(&format!(
        "- root_sha256: `{}` (빌드 시각 {})\n",
        r.root_sha256, r.built_at
    ));
    if s.recovered_basename > 0 || s.ext_swap > 0 {
        out.push_str("\n### 자동 복구한 링크\n\n");
        out.push_str("원문이 가리키는 경로가 실제로 없어서, 저장소 전체에서 이름이 유일한 파일로 되돌린 링크들이다(원본 `.md` 는 그대로 두었다).\n\n");
        for line in s.recovered.iter().take(64) {
            out.push_str(&format!("- `{}`\n", line));
        }
    }

    out.push_str("\n## 9. 섹션 목록\n\n");
    for (name, count) in &r.sections {
        out.push_str(&format!("- `{}`: {} 개\n", name, count));
    }
    out.push('\n');
    out
}
