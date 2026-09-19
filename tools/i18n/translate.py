#!/usr/bin/env python3
"""estudy 노트 한→영 번역기.

- 키는 macOS 키체인(og-api-key)에서만 읽는다. 파일·로그에 절대 쓰지 않는다.
- 코드블록·링크·이미지·HTML·수식은 그대로 두고 산문만 번역한다.
- 결과는 brains-en/<같은 경로> 로 쓴다. KO 원본(brains/) 은 손대지 않는다.
- 캐시(.cache/i18n/index.json) 로 원본 해시가 같으면 다시 부르지 않는다.

사용:
  python3 tools/i18n/translate.py --dry 2      # 앞 2개만 번역해 품질 확인(리포트만)
  python3 tools/i18n/translate.py --all --workers 8
  python3 tools/i18n/translate.py --check      # 캐시 현황만
"""
from __future__ import annotations

import argparse
import concurrent.futures as cf
import hashlib
import json
import os
import pathlib
import re
import subprocess
import sys
import threading
import time
import urllib.error
import urllib.request

REPO = pathlib.Path(__file__).resolve().parents[2]
SRC = REPO / 'brains'
DST = REPO / 'brains-en'
CACHE = REPO / 'tools/i18n/cache'
INDEX = CACHE / 'index.json'
BASE_URL = 'https://apis.opengateway.ai/v1'
MODEL = 'google/gemini-2.5-flash'

SYSTEM = """You are a professional technical translator. Translate Korean (한국어) software-engineering blog notes into natural, idiomatic English for engineers.

HARD RULES — violating any of these makes the output unusable:
1. Translate PROSE ONLY. Never translate or alter: fenced code blocks (``` ... ```), inline code (`...`), URLs, file paths, HTML tags/attributes, CSS/class names, image paths, LaTeX math ($...$, $$...$$), front matter keys, or markdown link targets.
2. Preserve the markdown structure EXACTLY: same number of lines, same number of headings and their levels, same list nesting, same blank-line layout, same fenced-block count, same link/image count.
3. Keep technical proper nouns and API names as-is (Spring Boot, Kubernetes, Redis, Kafka, JVM, GC, DDD, ...). Do not expand abbreviations.
4. Keep the closing punctuation style of the source (a bullet stays a bullet; a heading has no trailing period).
5. Do NOT add commentary, notes, explanations, prefaces, or trailing remarks. Do NOT wrap the document in a code fence.
6. If a line is only Korean text that is a figure caption, translate it; if it is a path/filename, leave it.

Output ONLY the translated markdown document."""

_lock = threading.Lock()
FORCE_CHUNK = False


def api_key() -> str:
    """API 키: 로컬은 키체인(og-api-key), CI는 환경변수 OG_API_KEY 에서 읽는다."""
    key = ''
    try:
        out = subprocess.run(['security', 'find-generic-password', '-s', 'og-api-key', '-w'],
                             capture_output=True, text=True)
        key = out.stdout.strip()
    except OSError:
        key = ''  # macOS 가 아닌 환경(CI)에는 security 명령이 없다 -> 환경변수 사용
    if not key:
        key = os.environ.get('OG_API_KEY', '').strip()
    if not key:
        raise SystemExit('키 없음: 키체인 og-api-key 또는 환경변수 OG_API_KEY 가 필요합니다')
    return key


def sha(text: str) -> str:
    return hashlib.sha256(text.encode('utf-8')).hexdigest()[:16]


def load_index() -> dict:
    if INDEX.exists():
        try:
            return json.loads(INDEX.read_text(encoding='utf-8'))
        except Exception:
            return {}
    return {}


def save_index(idx: dict) -> None:
    CACHE.mkdir(parents=True, exist_ok=True)
    tmp = INDEX.with_suffix('.tmp')
    tmp.write_text(json.dumps(idx, ensure_ascii=False, indent=1, sort_keys=True), encoding='utf-8')
    tmp.replace(INDEX)


def structure(md: str) -> dict:
    """검증용 구조 지표."""
    fences = len(re.findall(r'^```', md, re.M))
    headings = len(re.findall(r'^#{1,6} ', md, re.M))
    links = len(re.findall(r'\]\(', md))
    images = len(re.findall(r'!\[', md))
    return {'fences': fences, 'headings': headings, 'links': links, 'images': images, 'lines': md.count('\n')}


def call_model(key: str, text: str, tries: int = 4, system: str = SYSTEM) -> tuple[str, dict]:
    body = json.dumps({
        'model': MODEL,
        'messages': [
            {'role': 'system', 'content': system},
            {'role': 'user', 'content': text},
        ],
        'temperature': 0.2,
        'max_tokens': 16384,
    }).encode('utf-8')
    delay = 3.0
    last = None
    for attempt in range(tries):
        req = urllib.request.Request(
            f'{BASE_URL}/chat/completions', data=body,
            headers={'Authorization': f'Bearer {key}', 'Content-Type': 'application/json'},
            method='POST')
        try:
            with urllib.request.urlopen(req, timeout=300) as r:
                data = json.load(r)
            out = data['choices'][0]['message']['content']
            return out, data.get('usage', {})
        except urllib.error.HTTPError as e:
            last = f'HTTP {e.code}: {e.read()[:200]!r}'
            if e.code in (429, 500, 502, 503, 504):
                time.sleep(delay)
                delay *= 2
                continue
            raise RuntimeError(last)
        except Exception as e:  # 네트워크
            last = f'{type(e).__name__}: {e}'
            time.sleep(delay)
            delay *= 2
    raise RuntimeError(last or 'unknown error')


def strip_wrapper(md: str) -> str:
    s = md.strip()
    if s.startswith('```') and s.endswith('```'):
        first = s.split('\n', 1)
        if len(first) == 2 and first[0].strip() in ('```', '```markdown', '```md'):
            s = first[1].rsplit('```', 1)[0]
    return s.rstrip() + '\n'


def looks_collapsed(a: dict, b: dict) -> bool:
    """문서가 잘리거나 요약된 심각한 구조 붕괴 — 청크 번역으로 다시 시도할 대상."""
    if a['lines'] > 60 and b['lines'] < a['lines'] * 0.6:
        return True
    if a['fences'] >= 4 and b['fences'] * 3 < a['fences']:
        return True
    return a['headings'] >= 6 and b['headings'] * 2 < a['headings']


def translate_chunked(key: str, src: str) -> tuple[str, int]:
    """긴 문서 대비 — `## ` 경계로 나눠 각각 번역하고 그대로 이어붙인다(구분자는 보존)."""
    parts = re.split(r'(?m)^(?=## )', src)
    out: list[str] = []
    tokens = 0
    for part in parts:
        piece, usage = call_model(key, part)
        out.append(strip_wrapper(piece))
        tokens += usage.get('total_tokens') or 0
    return '\n'.join(p.rstrip() + '\n' for p in out), tokens


def translate_one(key: str, rel: str, src: str, idx: dict) -> dict:
    h = sha(src)
    cached = idx.get(rel)
    dst_path = DST / rel
    if cached and cached.get('src') == h and dst_path.exists():
        return {'rel': rel, 'status': 'cached'}
    # 내용이 사실상 없는 노트는 번역기를 부르지 않는다 — 모델이 없는 내용을 지어낸다.
    if len(src.strip()) < 40:
        dst_path.parent.mkdir(parents=True, exist_ok=True)
        dst_path.write_text(src, encoding='utf-8')
        with _lock:
            idx[rel] = {'src': h, 'dst': sha(src), 'model': '(skip: 빈 노트)',
                        'at': time.strftime('%Y-%m-%dT%H:%M:%SZ', time.gmtime()),
                        'usage': 0, 'bad': []}
        return {'rel': rel, 'status': 'skip', 'bad': []}
    t0 = time.time()
    out, usage = call_model(key, src)
    out = strip_wrapper(out)
    a, b = structure(src), structure(out)
    used_chunks = False
    if FORCE_CHUNK or looks_collapsed(a, b):
        # 문서가 뭉개졌다 → 청크 번역으로 재시도(성공하면 이 결과를 쓴다).
        try:
            out2, tok2 = translate_chunked(key, src)
            b2 = structure(out2)
            if b2['lines'] > b['lines']:
                out, b, used_chunks = out2, b2, True
                usage = {'total_tokens': (usage.get('total_tokens') or 0) + tok2}
        except Exception:
            pass
    bad = []
    if a['fences'] != b['fences']:
        bad.append(f"코드펜스 {a['fences']}→{b['fences']}")
    if a['links'] != b['links']:
        bad.append(f"링크 {a['links']}→{b['links']}")
    if a['images'] != b['images']:
        bad.append(f"이미지 {a['images']}→{b['images']}")
    if abs(a['headings'] - b['headings']) > 0:
        bad.append(f"제목 {a['headings']}→{b['headings']}")
    dst_path.parent.mkdir(parents=True, exist_ok=True)
    dst_path.write_text(out, encoding='utf-8')
    with _lock:
        idx[rel] = {'src': h, 'dst': sha(out), 'model': MODEL,
                    'at': time.strftime('%Y-%m-%dT%H:%M:%SZ', time.gmtime()),
                    'usage': usage.get('total_tokens'), 'bad': bad}
    return {'rel': rel, 'status': 'ok' if not bad else 'warn', 'sec': round(time.time() - t0, 1),
            'tokens': usage.get('total_tokens'), 'bad': bad, 'chunked': used_chunks}


def main() -> int:
    global MODEL, FORCE_CHUNK
    ap = argparse.ArgumentParser()
    ap.add_argument('--dry', type=int, default=0, help='앞 N개만 번역(리포트만, 저장은 함)')
    ap.add_argument('--all', action='store_true')
    ap.add_argument('--check', action='store_true')
    ap.add_argument('--workers', type=int, default=8)
    ap.add_argument('--filter', default='', help='경로 부분 문자열')
    ap.add_argument('--retry-bad', action='store_true', help='구조 검증 실패분만 다시')
    ap.add_argument('--model', default=MODEL, help='번역 모델(기본 %(default)s)')
    ap.add_argument('--chunk-force', action='store_true', help='대상 파일을 청크 번역으로 강제(긴 문서 보존)')
    args = ap.parse_args()

    MODEL = args.model
    FORCE_CHUNK = args.chunk_force

    files = sorted(p for p in SRC.rglob('*.md') if 'image' not in p.parts)
    rels = [str(p.relative_to(SRC)) for p in files]
    idx = load_index()
    if args.filter:
        rels = [r for r in rels if args.filter in r]
    if args.check:
        done = sum(1 for r in rels if idx.get(r) and (DST / r).exists() and idx[r]['src'] == sha((SRC / r).read_text(encoding='utf-8')))
        bad = [r for r in rels if idx.get(r, {}).get('bad')]
        print(f'노트 {len(rels)} · 최신 번역 {done} · 남음 {len(rels) - done} · 구조경고 {len(bad)}')
        for r in bad[:10]:
            print('   경고:', r, idx[r]['bad'])
        return 0
    if args.retry_bad:
        rels = [r for r in rels if idx.get(r, {}).get('bad')]
        for r in rels:
            idx.pop(r, None)  # 캐시를 지워 실제로 다시 부르게 한다
        print(f'재번역 대상 {len(rels)}')
    elif args.dry:
        rels = rels[:args.dry]
    elif not args.all:
        ap.error('--all | --dry N | --check | --retry-bad 중 하나')

    key = api_key()
    print(f'모델 {MODEL} · 대상 {len(rels)}개 · 워커 {args.workers}')
    t0 = time.time()
    done = warn = fail = 0
    with cf.ThreadPoolExecutor(max_workers=args.workers) as ex:
        futs = {}
        for r in rels:
            src = (SRC / r).read_text(encoding='utf-8')
            futs[ex.submit(translate_one, key, r, src, idx)] = r
        for i, fut in enumerate(cf.as_completed(futs), 1):
            r = futs[fut]
            try:
                res = fut.result()
                if res['status'] == 'warn':
                    warn += 1
                    print(f"  [{i}/{len(rels)}] 경고 {r} {res['bad']}")
                elif res['status'] == 'ok':
                    done += 1
                if i % 25 == 0:
                    print(f"  [{i}/{len(rels)}] 정상 {done} 경고 {warn} 실패 {fail} · {round(time.time()-t0)}s", flush=True)
                    save_index(idx)
            except Exception as e:
                fail += 1
                print(f"  [{i}/{len(rels)}] 실패 {r}: {str(e)[:160]}")
    save_index(idx)
    print(f'끝: 정상 {done} · 경고 {warn} · 실패 {fail} · {round(time.time()-t0)}초')
    return 0


if __name__ == '__main__':
    sys.exit(main())
