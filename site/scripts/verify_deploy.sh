#!/usr/bin/env bash
# 배포된 estudy 블로그를 실제 URL로 검증한다.
#
# 사용법:
#   site/scripts/verify_deploy.sh                       # 기본: https://rlaope.github.io/estudy
#   site/scripts/verify_deploy.sh https://example.com/  # 다른 base URL
#
# 검사 항목: 홈/섹션/노트/이미지/그래프 자산 응답 코드, 노트 본문 JS 0, wasm MIME,
#            README 색인의 github blob 링크 잔존 수, 한국어 파일명 200.
set -uo pipefail

BASE="${1:-https://rlaope.github.io/estudy}"
BASE="${BASE%/}"
REPO_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
SEARCH_JSON="$REPO_ROOT/site/generated/search.json"
FAILED=0

code() { curl -s -o /dev/null -w '%{http_code}' -L --max-time 20 "$1"; }
ctype() { curl -s -o /dev/null -w '%{content_type}' -L --max-time 20 "$1"; }

check() { # check <label> <url> <expected-code>
  local label="$1" url="$2" want="$3" got
  got="$(code "$url")"
  if [ "$got" = "$want" ]; then
    printf '  ok    %-56s %s\n' "$label" "$got"
  else
    printf '  FAIL  %-56s want %s got %s  %s\n' "$label" "$want" "$got" "$url"
    FAILED=1
  fi
}

check_any() { # check_any <label> <expected-code> <url...>  (첫 성공에서 통과)
  local label="$1" want="$2"; shift 2
  local got url
  for url in "$@"; do
    got="$(code "$url")"
    if [ "$got" = "$want" ]; then
      printf '  ok    %-56s %s  (%s)\n' "$label" "$got" "$url"
      return 0
    fi
  done
  printf '  FAIL  %-56s want %s got %s  tried: %s\n' "$label" "$want" "$got" "$*"
  FAILED=1
}

echo "== base: $BASE =="
echo "-- 정적 진입점 --"
check "home"                     "$BASE/"                                  200
check_any "section index (uglyURLs or dir)" 200 "$BASE/Back-End.html" "$BASE/Back-End/"
check "korean filename note"     "$BASE/%EC%A0%95%EB%B3%B4%EC%B2%98%EB%A6%AC/%EB%8F%99%EA%B8%B0%ED%99%94.html" 200
check "case-fixed note"          "$BASE/Design-Pattern/%ED%96%89%EB%8F%99/Interpreter.html"                    200

echo "-- 노트/이미지 샘플 (search.json 기반) --"
if [ -f "$SEARCH_JSON" ]; then
  python3 - "$SEARCH_JSON" "$BASE" "$REPO_ROOT" <<'PY' > /tmp/verify_urls.txt
import json, os, sys
from urllib.parse import urlsplit
search_json, base, repo = sys.argv[1], sys.argv[2].rstrip("/"), sys.argv[3]
sp = urlsplit(base)
origin = f"{sp.scheme}://{sp.netloc}"
prefix = sp.path.rstrip("/")            # 예: /estudy

data = json.load(open(search_json, encoding="utf-8"))

def absolute(u):
    """search.json 의 url 은 루트 기준 절대경로(/estudy/...)라 origin 만 붙인다."""
    if u.startswith("http"):
        return u
    return origin + (u if u.startswith("/") else "/" + u)

for d in [d for d in data if d.get("url", "").endswith(".html")][:20]:
    print("note", absolute(d["url"]))

# 이미지 샘플: 저장소에서 실제 이미지 파일 5개를 찾아 배포 경로로 변환
imgs = []
for dp, dn, fn in os.walk(repo):
    if any(part in dp for part in (os.sep + ".git", os.sep + ".cache", os.sep + "site" + os.sep + "static")):
        continue
    for f in fn:
        if f.rsplit(".", 1)[-1].lower() in ("png", "webp", "jpeg", "jpg", "gif"):
            imgs.append(os.path.relpath(os.path.join(dp, f), repo))
    if len(imgs) >= 5:
        break
for rel in sorted(set(imgs))[:5]:
    print("image", origin + prefix + "/" + rel)
PY
  while read -r kind url; do
    [ -z "${url:-}" ] && continue
    check "$kind $(basename "$url" | cut -c1-40)" "$url" 200
  done < /tmp/verify_urls.txt
else
  echo "  skip  site/generated/search.json 없음 (샘플 대신 고정 URL 사용)"
  check "note sample"  "$BASE/Math/math5.html" 200
  check "image sample" "$BASE/Back-End/spring/image/aop.png" 200
fi

echo "-- README 색인 링크 재작성 --"
home_html="$(curl -sL --max-time 30 "$BASE/" || true)"
blob_links="$(printf '%s' "$home_html" | grep -o 'github\.com/rlaope/estudy/blob/master' | wc -l | tr -d ' ')"
# Hugo --minify 는 속성 따옴표를 제거하므로 href= 뒤 따옴표 유무를 모두 받는다
internal_links="$(printf '%s' "$home_html" | grep -oE 'href=[^ >]+\.html' | wc -l | tr -d ' ')"
echo "  github blob 잔존: $blob_links / 내부 .html 링크: $internal_links"
if [ "$blob_links" != "0" ]; then
  echo "  FAIL  README 색인에 github blob 링크가 남아 있다 (전처리 미적용)"
  FAILED=1
fi
if [ "$internal_links" = "0" ]; then
  echo "  FAIL  홈에서 내부 노트 링크를 찾지 못했다 (홈이 README 색인이 아니다)"
  FAILED=1
fi

echo "-- 노트 본문 클라이언트 JS / 수식 --"
# 샘플 노트를 실제로 받아 상태 코드를 먼저 확인한다 (404 HTML로 통과하는 것을 막는다)
note_url="$BASE/Math/math5.html"
note_body="$(curl -sL --max-time 30 "$note_url" || true)"
note_status="$(curl -s -o /dev/null -w '%{http_code}' -L --max-time 30 "$note_url" || echo 000)"
if [ "$note_status" != "200" ]; then
  echo "  FAIL  샘플 노트 응답 $note_status ($note_url) — 본문 검사를 할 수 없다"
  FAILED=1
else
  scripts="$(printf '%s' "$note_body" | grep -o '<script' | wc -l | tr -d ' ')"
  echo "  <script> 태그 수: $scripts"
  if [ "$scripts" != "0" ]; then
    echo "  FAIL  노트 본문에 클라이언트 JS가 있다"
    FAILED=1
  fi
  if printf '%s' "$note_body" | grep -Eq 'katex-html|katex-display|class=katex|class="katex"'; then
    echo "  ok    수식 서버사이드 렌더(KaTeX) 확인"
  else
    echo "  FAIL  수식이 KaTeX로 렌더되지 않았다"
    FAILED=1
  fi
fi

echo "-- 그래프 자산 --"
wasm_url="$BASE/graph.html"
graph_code="$(code "$wasm_url")"
if [ "$graph_code" = "200" ]; then
  # wasm-pack --target web 글루는 new URL('graph_wasm_bg.wasm', import.meta.url) 형태라
  # HTML에 직접 나타나지 않는다. 글루 JS에서 실제 wasm 파일명을 읽는다.
  glue="$(curl -sL --max-time 30 "$BASE/graph/pkg/graph_wasm.js" || true)"
  wasm_file="$(printf '%s' "$glue" | grep -oE '[A-Za-z0-9_.-]+\.wasm' | head -1)"
  [ -z "${wasm_file:-}" ] && wasm_file="graph_wasm_bg.wasm"
  wasm_full="$BASE/graph/pkg/$wasm_file"
  ct="$(ctype "$wasm_full")"
  size="$(curl -sL --max-time 60 "$wasm_full" | wc -c | tr -d ' ')"
  echo "  wasm: graph/pkg/$wasm_file  content-type=$ct  bytes=$size"
  case "$ct" in
    *application/wasm*) ;;
    *) echo "  WARN  wasm content-type이 application/wasm이 아니다" ;;
  esac
  if [ "$size" -gt 81920 ]; then
    echo "  FAIL  wasm이 80KB 예산을 초과했다 ($size bytes)"
    FAILED=1
  fi
  check "graph data search.json" "$BASE/generated/search.json" 200
  check "graph data pos.bin"     "$BASE/generated/pos.bin"     200
  check "graph data graph.bin"   "$BASE/generated/graph.bin"   200
else
  echo "  FAIL  graph.html 응답 $graph_code ($wasm_url) — 그래프 페이지가 배포되지 않았다"
  FAILED=1
fi

echo
if [ "$FAILED" = "0" ]; then
  echo "RESULT: pass"
else
  echo "RESULT: fail"
fi
exit "$FAILED"
