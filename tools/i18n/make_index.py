#!/usr/bin/env python3
"""영어 홈 색인(README-en) 생성기.

KO README 의 그룹(`####`)·순서·링크를 그대로 두고, 링크 라벨만 해당 노트의 영어 제목
(brains-en/<경로> 의 첫 `# ` 줄)으로 바꿔 `.cache/content-en/README.md` 를 만든다.

사용:
  python3 tools/i18n/make_index.py            # 생성
  python3 tools/i18n/make_index.py --check    # 개수만 확인
"""
from __future__ import annotations

import argparse
import hashlib
import json
import os
import pathlib
import re
import sys

sys.path.insert(0, str(pathlib.Path(__file__).resolve().parent))
from translate import api_key, call_model, strip_wrapper  # noqa: E402 (같은 디렉터리 모듈)

REPO = pathlib.Path(__file__).resolve().parents[2]
KO_README = REPO / 'README.md'      # 한국어 원본(레포 기본 문서 + 사이트 KO 홈 색인의 출처)
EN_README = REPO / 'README.en.md'   # 영어판(이 스크립트가 생성)
EN_ROOT = REPO / 'brains-en'
# 기본은 파이프라인 EN 스테이지 위치. 로컬 실험/다른 스테이지에는 환경변수로 덮어쓴다.
OUT = pathlib.Path(os.environ.get('ESTUDY_EN_README_OUT', REPO / '.cache/content-en/README.md'))
INTRO_CACHE = REPO / '.cache/i18n/en-readme-intro.json'
LINK_RE = re.compile(r'https?://github\.com/rlaope/estudy/blob/master/([^)\s]+)')

INTRO_SYSTEM = """You translate the opening section of a Korean software-engineering notes repository README into natural English.

Rules:
- Translate prose only. Keep headings (#, ##), links, images, HTML, code fences and markdown structure exactly.
- Keep the repository name "estudy" and other proper nouns as-is.
- Keep line breaks and blank lines as in the source; do not add or remove sections.
- Output only the translated markdown, no commentary."""


def translate_intro(text: str, use_cache: bool = True) -> str:
    h = hashlib.sha256(text.encode('utf-8')).hexdigest()
    if use_cache and INTRO_CACHE.exists():
        try:
            c = json.loads(INTRO_CACHE.read_text(encoding='utf-8'))
            if c.get('src') == h and c.get('dst'):
                return c['dst']
        except Exception:
            pass
    out, _usage = call_model(api_key(), text, system=INTRO_SYSTEM)
    out = strip_wrapper(out)
    INTRO_CACHE.parent.mkdir(parents=True, exist_ok=True)
    INTRO_CACHE.write_text(json.dumps({'src': h, 'dst': out, 'model': 'google/gemini-2.5-flash'},
                                     ensure_ascii=False, indent=1), encoding='utf-8')
    return out


def en_title(rel: str) -> str | None:
    p = EN_ROOT / rel
    if p.is_dir():
        for cand in ('README.md', 'index.md'):
            if (p / cand).is_file():
                p = p / cand
                break
        else:
            return None
    if not p.is_file():
        return None
    for line in p.read_text(encoding='utf-8').splitlines():
        s = line.strip()
        if s.startswith('# '):
            return s[2:].strip()
    return None


def main() -> int:
    ap = argparse.ArgumentParser()
    ap.add_argument('--check', action='store_true')
    ap.add_argument('--no-intro', action='store_true', help='인트로 번역 없이(빠른 확인용)')
    args = ap.parse_args()

    ko = KO_README.read_text(encoding='utf-8')
    lines = ko.split('\n')
    # 인트로 = 첫 `####` 그룹 제목 앞부분(제목·소개 문단). 그 뒤는 색인.
    first_head = next((i for i, l in enumerate(lines) if re.match(r'^####\s+\S', l)), 0)
    intro_ko = '\n'.join(lines[:first_head]).rstrip()
    body = lines[first_head:]

    out_lines: list[str] = []
    total = 0
    missing = 0
    for line in body:
        m = LINK_RE.search(line)
        if not m:
            out_lines.append(line)
            continue
        path = m.group(1).split('#')[0].split('?')[0]
        if path.startswith('brains/'):
            path = path[len('brains/'):]
        total += 1
        title = en_title(path)
        if title is None:
            missing += 1
            out_lines.append(line)  # 번역이 아직 없으면 KO 라벨 유지
            continue
        # 라벨만 교체: [ ... ](<원래 url>)
        url = m.group(0)
        out_lines.append(re.sub(r'\[[^\]]*\]\(' + re.escape(url) + r'\)',
                                lambda _m, u=url, t=title: f'[{t}]({u})', line))

    if args.check:
        print(f'링크 {total} · 영어 제목 있음 {total - missing} · 아직 없음 {missing}')
        print(f'출력 예정: {OUT} + {EN_README}')
        return 0

    intro_en = intro_ko
    if not args.no_intro and intro_ko:
        try:
            intro_en = translate_intro(intro_ko)
        except Exception as e:
            print(f'인트로 번역 실패({str(e)[:80]}) — 한국어 유지')
    # 레포 기본 문서는 한국어 README.md 이고, 이 파일은 영어판이다(헤더 장식 없이 본문만).
    header = ''
    head_parts = [p for p in (header, intro_en) if p]
    doc = '\n'.join([*head_parts, '', *out_lines]).rstrip() + '\n'

    OUT.parent.mkdir(parents=True, exist_ok=True)
    OUT.write_text(doc, encoding='utf-8')
    EN_README.write_text(doc, encoding='utf-8')
    print(f'생성: {EN_README} + {OUT} ({len(out_lines)}줄) · 링크 {total} · 영어 제목 {total - missing} · 미번역 {missing} · 인트로 {"번역" if intro_en != intro_ko else "원문"}')
    return 0


if __name__ == '__main__':
    sys.exit(main())
