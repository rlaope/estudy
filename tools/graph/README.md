# tools/graph — estudy 빌드타임 콘텐츠 파이프라인 (L1)

890개 한국어 Markdown 노트를 **Hugo 가 그대로 먹을 수 있는 스테이지 트리**로 만들고,
사이트가 코드로 소비하는 **동결 산출물**(그래프·검색·레이아웃 좌표)을 생성한다.
원본 노트/이미지는 절대 수정하지 않는다 — 모든 재작성은 스테이지 사본에서만 일어난다.

## 구성

| 경로 | 역할 |
| --- | --- |
| `graph-core/` | 라이브러리: 스캔, 링크 정규화, 그래프 모델, `fa2` 레이아웃, 산출물 직렬화, `CONTRACT.md` 생성 |
| `graph-build/` | CLI 바이너리 (`--root`, `--out`, `--stage`, `--base-path`, `--seed`, `--iterations`) |

의존성은 `fa2 0.3.0`(MIT, ForceAtlas2), `sha2`, `unicode-normalization` 뿐이다.
`fa2` 는 rayon 을 끌어오므로 **네이티브 전용**이며 `native-layout` feature 뒤에 숨어 있다.
WASM 크레이트에서 `graph-core` 를 쓸 경우 `default-features = false` 로 끌 것.
AGPL 인 `forceatlas2` 크레이트는 쓰지 않는다.

## 실행

리포 루트에서:

```sh
cargo run -p graph-build --release -- --root . --out site/generated --stage .cache/content
```

- 워크스페이스 매니페스트는 리포 루트의 `Cargo.toml` 에 있다(멤버만 선언).
  그래야 리포 루트에서 `cargo run -p graph-build` 가 그대로 동작한다.
- 빌드 산출물은 `.cargo/config.toml` 의 `target-dir` 설정으로 `.cache/target` 에 격리된다
  (리포에 `target/` 이 생기지 않는다).

## 산출물

- `.cache/content/**` — Hugo `contentDir` 로 쓰는 스테이지 트리
  (링크 재작성 적용, `DataBase/index.md` → `DataBase/_index.md`)
- `site/generated/pos.bin`, `graph.bin`, `search.json`, `meta.json` — 동결 계약 산출물
- `site/generated/CONTRACT.md` — 위 파일들의 바이트 레이아웃과 규칙을 적은 계약 문서

## 검증

```sh
python3 -c "import json;print(len(json.load(open('site/generated/search.json'))))"   # 890
shasum -a 256 site/generated/pos.bin                                                  # 2회 실행 비교
test -f .cache/content/DataBase/_index.md && echo staged-index-ok
```

Hugo 스모크 테스트(스테이지 트리 기준, 레이아웃은 아직 L3 소유):

```sh
/tmp/hugotest/expanded/Payload/hugo --config /tmp/hugotest/hugo_full.toml \
  --contentDir .cache/content --destination /tmp/l1-out --layoutDir /tmp/hugotest/layouts
find /tmp/l1-out -name '*.html' | wc -l    # 911
```

> 주의: Hugo 0.166.0 은 설정 파일의 `layoutsDir` 를 해석하지 않는다(`hugo config` 로 확인).
> 그래서 스모크 테스트에서는 `--layoutDir` 를 명시하거나 cwd 에 `layouts/` 가 있어야 한다.
