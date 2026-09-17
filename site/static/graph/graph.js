// estudy 지식 그래프 — canvas2D 렌더러 + DOM 셸.
// 질의(검색·ego·히트테스트·라벨 컬링)는 ./pkg/graph_wasm (Rust→WASM) 이 맡고,
// 이 파일은 데이터 적재·뷰 변환·그리기·입력·접근성 DOM 만 담당한다.
// 이 파일은 사이트에서 유일하게 JS 가 실행되는 페이지(graph.html) 에서만 로드된다.

// ---- 설정 ---------------------------------------------------------------
// 데이터 베이스 경로: 페이지(graph.html) 기준 상대 경로. 배포는 /estudy/generated/ 가 된다.
// 로컬 테스트: graph.html?data=http://127.0.0.1:8123/generated/
const DATA_BASE = 'generated/';
// 섹션 허브 링크 (graph.html 기준 상대). Hugo uglyURLs=true → <Section>.html
const HUB_URL = (section) => `${encodeURIComponent(section)}.html`;
const ROOT_URL = './';
const MAX_DPR = 2;
const LABEL_ZOOM = 1.2;
const LIST_PAGE = 200;
const VISITED_KEY = 'estudy.graph.visited';
const NONE = 0xffffffff;

// ---- 토큰 (graph.css 와 같은 값; canvas 는 CSS 변수를 직접 못 읽어 getComputedStyle 로 가져온다)
const cssVar = (name, fallback) => {
  const v = getComputedStyle(document.documentElement).getPropertyValue(name).trim();
  return v || fallback;
};
let TOKENS = {};
function readTokens() {
  TOKENS = {
    node: cssVar('--graph-node', '#737373'),
    current: cssVar('--graph-node-current', '#0052cc'),
    visited: cssVar('--graph-node-visited', '#6b5bd6'),
    neighbor: cssVar('--graph-node-neighbor', '#111111'),
    edge: cssVar('--graph-edge', '#bdbdbd'),
    edgeAlpha: parseFloat(cssVar('--graph-edge-alpha', '0.6')),
    dimAlpha: parseFloat(cssVar('--graph-dim-alpha', '0.15')),
    label: cssVar('--graph-label', '#525252'),
    bg: cssVar('--graph-bg', '#ffffff'),
    accent: cssVar('--color-accent', '#0052cc'),
    sans: cssVar('--font-sans', '"Pretendard Variable", Pretendard, -apple-system, sans-serif'),
    mono: cssVar('--font-mono', 'ui-monospace, monospace'),
  };
}

const params = new URLSearchParams(location.search);
const dataBase = (params.get('data') || DATA_BASE).replace(/\/?$/, '/');
const reducedMotion = matchMedia('(prefers-reduced-motion: reduce)').matches;

// ---- DOM ----------------------------------------------------------------
const $ = (id) => document.getElementById(id);
const canvas = $('graph');
const ctx = canvas.getContext('2d', { alpha: false });
const tooltip = $('tooltip');
const statsEl = $('stats');
const qEl = $('q');
const sectionEl = $('section');
const scopeBtn = $('toggle-scope');
const clearBtn = $('clear');
const fitBtn = $('fit');
const listEl = $('list');
const listTitle = $('list-title');
const listMeta = $('list-meta');
const focusBox = $('focus');
const focusTitle = $('focus-title');
const focusMeta = $('focus-meta');
const focusOpen = $('focus-open');
const focusEgoBtn = $('focus-ego');
const egoDepthEl = $('ego-depth');

// ---- 상태 ---------------------------------------------------------------
const S = {
  n: 0, pos: null, offsets: null, targets: null, radius: null,
  nodes: [], // {id,title,url,section,degree,hub}
  hubs: [],  // section hub ids in order, root last
  rootId: 0,
  view: { scale: 1, tx: 0, ty: 0 }, w: 0, h: 0, dpr: 1,
  visible: null,      // Uint8Array — 현재 범위(로컬/전체 ∩ ego)
  match: null,        // Uint8Array|null — 검색 일치 (null = 검색 없음)
  hover: NONE, focus: NONE, current: NONE, selected: NONE,
  ego: NONE, egoDepth: 2,
  scope: 'global', scopeHub: NONE,
  visited: new Set(),
  edgeLayer: null, edgeDirty: true, frame: 0,
  drawnNodes: 0, drawnTargets: 0, drawnLines: 0,
  listOffset: 0,
};
let wasm;

// ---- 적재 ---------------------------------------------------------------
async function fetchBuf(name) {
  const r = await fetch(dataBase + name);
  if (!r.ok) throw new Error(`${name}: HTTP ${r.status}`);
  return r.arrayBuffer();
}
async function fetchJson(name) {
  const r = await fetch(dataBase + name);
  if (!r.ok) throw new Error(`${name}: HTTP ${r.status}`);
  return r.json();
}

async function main() {
  readTokens();
  const t0 = performance.now();
  const [mod, posBuf, csrBuf, search, meta] = await Promise.all([
    import('./pkg/graph_wasm.js').then(async (m) => { await m.default({ module_or_path: new URL('./pkg/graph_wasm_bg.wasm', import.meta.url) }); return m; }),
    fetchBuf('pos.bin'), fetchBuf('graph.bin'), fetchJson('search.json'), fetchJson('meta.json'),
  ]);
  wasm = mod;

  // --- CONTRACT.md 검증 (형식이 다르면 멈춘다)
  const pos = new Float32Array(posBuf);
  const csr = new Uint32Array(csrBuf);
  const n = pos.length / 2;
  const sections = meta.sections.map((s) => s.name);
  const errs = [];
  if (n !== meta.nodes) errs.push(`pos.bin 노드 ${n} ≠ meta.nodes ${meta.nodes}`);
  if (csr.length !== n + 1 + meta.edges) errs.push(`graph.bin u32 ${csr.length} ≠ N+1+E ${n + 1 + meta.edges}`);
  if (csr[n] !== meta.edges) errs.push(`offsets[N] ${csr[n]} ≠ meta.edges ${meta.edges}`);
  if (search.length + sections.length + 1 !== n) errs.push(`search ${search.length} + 섹션 ${sections.length} + 1 ≠ N ${n}`);
  search.forEach((e, i) => { if (e.id !== i) errs.push(`search.json id 순서 어긋남 at ${i}`); });
  if (errs.length) { fail('산출물이 CONTRACT.md 와 다릅니다:\n' + errs.join('\n')); return; }

  // --- 노드 테이블 (노트 890 + 섹션 허브 19 + 루트 1)
  S.n = n; S.pos = pos; S.offsets = csr.subarray(0, n + 1); S.targets = csr.subarray(n + 1);
  S.rootId = n - 1;
  const hubIndex = new Map();
  S.nodes = search.map((e) => ({ id: e.id, title: e.title, url: e.url, section: e.section, path: e.path, degree: e.degree, hub: false }));
  sections.forEach((name, k) => {
    const id = search.length + k;
    hubIndex.set(name, id);
    S.hubs.push(id);
    S.nodes.push({ id, title: name, url: HUB_URL(name), section: name, path: name + '/', degree: S.offsets[id + 1] - S.offsets[id], hub: true });
  });
  S.nodes.push({ id: S.rootId, title: 'estudy', url: ROOT_URL, section: 'root', path: '/', degree: S.offsets[n] - S.offsets[n - 1], hub: true });
  S.hubIndex = hubIndex;

  const ok = wasm.load(pos, csr, S.nodes.map((x) => x.title).join('\n'),
    S.nodes.map((x, i) => (i < search.length ? search[i].choseong : '')).join('\n'), S.rootId);
  if (!ok) { fail('graph_wasm.load 가 산출물을 거부했습니다 (CSR 불일치).'); return; }
  S.radius = wasm.radii();
  const st = wasm.stats();

  // --- 방문/현재 노드
  try { JSON.parse(localStorage.getItem(VISITED_KEY) || '[]').forEach((p) => S.visited.add(p)); } catch { /* ignore */ }
  S.current = resolveCurrent();
  if (S.current !== NONE) S.visited.add(S.nodes[S.current].path);

  // --- 섹션 select
  for (const name of sections) {
    const o = document.createElement('option'); o.value = name; o.textContent = name; sectionEl.append(o);
  }
  const wantSection = params.get('section') || (S.current !== NONE && S.current < search.length ? S.nodes[S.current].section : '');
  if (wantSection && hubIndex.has(wantSection)) { S.scope = 'local'; S.scopeHub = hubIndex.get(wantSection); sectionEl.value = wantSection; }
  else if (params.get('scope') === 'global' || !wantSection) { S.scope = 'global'; }

  // --- 폰트를 먼저 기다린다: 첫 프레임이 폴백 글꼴로 그려지지 않도록
  await Promise.all([
    document.fonts.load(`520 12px ${TOKENS.sans}`, '가나다'),
    document.fonts.load(`12px ${TOKENS.mono}`, '0123'),
  ]).catch(() => {});

  bindUI();
  resize();
  recomputeVisible();
  fitToVisible(false);
  draw();
  const firstPaint = Math.round(performance.now());
  statsEl.textContent = `노드 ${st[0]} · 링크 ${st[2]} (CSR ${st[1]}) · ${meta.layout} seed ${meta.seed} · wasm ${Math.round(performance.now() - t0)}ms`;
  for (const el of [qEl, sectionEl, scopeBtn, clearBtn, fitBtn]) el.disabled = false;
  updateScopeBtn();
  renderList();
  updateFocusBox();
  if (params.has('q')) { qEl.value = params.get('q'); onSearch(); }
  if (params.has('ego')) { const id = Number(params.get('ego')); if (id >= 0 && id < n) setEgo(id); }

  // 헤드리스 검증용 훅 (UI 동작에는 쓰이지 않는다)
  window.__graph = {
    firstPaintMs: firstPaint, stats: Array.from(st), meta,
    get drawn() { return { nodes: S.drawnNodes, csrTargets: S.drawnTargets, lines: S.drawnLines }; },
    search: (q) => Array.from(wasm.search(q)).map((i) => ({ id: i, title: S.nodes[i].title })),
    ego: (id, d) => Array.from(wasm.ego(id, d)),
    hubs: S.hubs.map((id) => ({ id, title: S.nodes[id].title })),
    setEgo, setScope, state: S,
  };
  performance.mark('graph-first-paint');
}

function fail(msg) {
  statsEl.textContent = '오류';
  const p = document.createElement('p'); p.className = 'gh-noscript'; p.style.whiteSpace = 'pre-wrap'; p.textContent = msg;
  canvas.after(p);
  console.error(msg);
}

function resolveCurrent() {
  const byUrl = (u) => { const i = S.nodes.findIndex((x) => x.url === u); return i < 0 ? NONE : i; };
  if (params.has('node')) { const id = Number(params.get('node')); if (id >= 0 && id < S.n) return id; }
  if (params.has('from')) { const id = byUrl(params.get('from')); if (id !== NONE) return id; }
  if (document.referrer) {
    try { const u = new URL(document.referrer); if (u.origin === location.origin) { const id = byUrl(decodeURI(u.pathname)); if (id !== NONE) return id; } } catch { /* ignore */ }
  }
  return NONE;
}

// ---- 범위(visible) --------------------------------------------------------
function recomputeVisible() {
  const n = S.n;
  let vis = new Uint8Array(n);
  if (S.scope === 'local' && S.scopeHub !== NONE) { for (const id of wasm.local_view(S.scopeHub)) vis[id] = 1; }
  else vis.fill(1);
  if (S.ego !== NONE) {
    const d = wasm.ego_depths(S.ego, S.egoDepth);
    for (let i = 0; i < n; i++) if (d[i] === 255) vis[i] = 0;
    vis[S.ego] = 1;
  }
  if (S.current !== NONE) vis[S.current] = 1;
  S.visible = vis; S.edgeDirty = true;
}

function setScope(scope, hubId = S.scopeHub) {
  S.scope = scope; S.scopeHub = hubId;
  recomputeVisible(); fitToVisible(true); updateScopeBtn(); renderList(); requestDraw();
}
function updateScopeBtn() {
  const local = S.scope === 'local' && S.scopeHub !== NONE;
  scopeBtn.setAttribute('aria-pressed', String(!local));
  scopeBtn.textContent = local ? `전체 보기` : `섹션만 보기`;
  scopeBtn.disabled = !local && S.scopeHub === NONE;
}
function setEgo(id) {
  S.ego = id; S.selected = id;
  if (id !== NONE) S.egoDepth = Number(egoDepthEl.value) || 2;
  recomputeVisible(); fitToVisible(true); updateFocusBox(); renderList(); requestDraw();
}

// ---- 뷰 변환 --------------------------------------------------------------
function resize() {
  const wrap = canvas.parentElement;
  const rect = wrap.getBoundingClientRect();
  S.w = Math.max(1, Math.floor(rect.width)); S.h = Math.max(1, Math.floor(rect.height));
  S.dpr = Math.min(MAX_DPR, window.devicePixelRatio || 1);
  canvas.width = Math.round(S.w * S.dpr); canvas.height = Math.round(S.h * S.dpr);
  canvas.style.width = S.w + 'px'; canvas.style.height = S.h + 'px';
  S.edgeLayer = new OffscreenCanvasOrFallback(canvas.width, canvas.height);
  S.edgeDirty = true;
}
function OffscreenCanvasOrFallback(w, h) {
  if (typeof OffscreenCanvas === 'function') return new OffscreenCanvas(w, h);
  const c = document.createElement('canvas'); c.width = w; c.height = h; return c;
}
function nodeScale() { return Math.min(1.6, Math.max(0.7, Math.sqrt(S.view.scale))); }
function syncView() { wasm.set_view(S.view.scale, S.view.tx, S.view.ty, S.w, S.h, nodeScale()); }

function fitToVisible(animate) {
  let minX = Infinity, minY = Infinity, maxX = -Infinity, maxY = -Infinity, cnt = 0;
  for (let i = 0; i < S.n; i++) {
    if (!S.visible[i]) continue; cnt++;
    const x = S.pos[2 * i], y = S.pos[2 * i + 1];
    if (x < minX) minX = x; if (x > maxX) maxX = x; if (y < minY) minY = y; if (y > maxY) maxY = y;
  }
  if (!cnt) return;
  const pad = 48;
  const spanX = Math.max(1, maxX - minX), spanY = Math.max(1, maxY - minY);
  const scale = Math.min((S.w - 2 * pad) / spanX, (S.h - 2 * pad) / spanY, 6);
  const target = { scale, tx: S.w / 2 - (minX + maxX) / 2 * scale, ty: S.h / 2 - (minY + maxY) / 2 * scale };
  if (!animate || reducedMotion) { S.view = target; S.edgeDirty = true; return; }
  animateView(target);
}
let anim = 0;
function animateView(target) {
  cancelAnimationFrame(anim);
  const from = { ...S.view }, start = performance.now(), dur = 260;
  const step = (now) => {
    const t = Math.min(1, (now - start) / dur), e = 1 - Math.pow(1 - t, 3);
    S.view = { scale: from.scale + (target.scale - from.scale) * e, tx: from.tx + (target.tx - from.tx) * e, ty: from.ty + (target.ty - from.ty) * e };
    S.edgeDirty = true; draw();
    if (t < 1) anim = requestAnimationFrame(step);
  };
  anim = requestAnimationFrame(step);
}

// ---- 그리기 ---------------------------------------------------------------
let rafId = 0;
function requestDraw() { if (!rafId) rafId = requestAnimationFrame(() => { rafId = 0; draw(); }); }

function hoverSet() {
  // 강조 대상: hover 노드 + 이웃 (없으면 focus/selected)
  const centre = S.hover !== NONE ? S.hover : S.focus !== NONE ? S.focus : NONE;
  if (centre === NONE) return null;
  const set = new Uint8Array(S.n); set[centre] = 1;
  for (const t of wasm.neighbors(centre)) set[t] = 1;
  return { centre, set };
}

function drawEdges(target, hs) {
  const g = target.getContext('2d');
  const { scale, tx, ty } = S.view, dpr = S.dpr;
  g.setTransform(1, 0, 0, 1, 0, 0);
  g.clearRect(0, 0, target.width, target.height);
  g.setTransform(dpr, 0, 0, dpr, 0, 0);
  g.lineWidth = 1; // ×dpr 은 transform 이 처리
  g.strokeStyle = TOKENS.edge;
  g.lineCap = 'butt';
  const vis = S.visible, off = S.offsets, tg = S.targets, pos = S.pos;
  let lines = 0, csr = 0;
  // 한 패스 = 보이는 CSR 타깃을 전부 훑고, 무방향 엣지는 (u<v) 한 번만 긋는다.
  const pass = (alpha, pred, count) => {
    g.globalAlpha = alpha; g.beginPath();
    for (let u = 0; u < S.n; u++) {
      if (!vis[u]) continue;
      const ux = pos[2 * u] * scale + tx, uy = pos[2 * u + 1] * scale + ty;
      for (let k = off[u]; k < off[u + 1]; k++) {
        const v = tg[k]; if (!vis[v]) continue;
        if (count) csr++;
        if (v < u || !pred(u, v)) continue;
        lines++;
        g.moveTo(ux, uy); g.lineTo(pos[2 * v] * scale + tx, pos[2 * v + 1] * scale + ty);
      }
    }
    g.stroke();
  };
  if (!hs) { pass(TOKENS.edgeAlpha, () => true, true); }
  else {
    const hot = (u, v) => hs.set[u] && hs.set[v] && (u === hs.centre || v === hs.centre);
    pass(TOKENS.edgeAlpha * TOKENS.dimAlpha, (u, v) => !hot(u, v), true);
    pass(TOKENS.edgeAlpha, hot, false);
  }
  S.drawnLines = lines;      // 실제로 그은 선 (무방향 엣지 수)
  S.drawnTargets = csr;      // 방문한 CSR 타깃 수 (= 양방향 저장 기준, meta.edges 와 비교)
  g.globalAlpha = 1;
}

function draw() {
  if (!S.visible) return;
  S.frame++;
  syncView();
  const { scale, tx, ty } = S.view, dpr = S.dpr;
  const hs = hoverSet();
  // 엣지 레이어: 뷰가 바뀌었거나 강조 집합이 바뀌었을 때만 다시 그린다
  const hsKey = hs ? hs.centre : -1;
  if (S.edgeDirty || S.edgeKey !== hsKey) { drawEdges(S.edgeLayer, hs); S.edgeDirty = false; S.edgeKey = hsKey; }

  ctx.setTransform(1, 0, 0, 1, 0, 0);
  ctx.fillStyle = TOKENS.bg; ctx.fillRect(0, 0, canvas.width, canvas.height);
  ctx.drawImage(S.edgeLayer, 0, 0);
  ctx.setTransform(dpr, 0, 0, dpr, 0, 0);

  // 노드: 색 상태 기계 → 배치(색별 한 path)
  const ns = nodeScale();
  const groups = { dim: [], def: [], visited: [], neighbor: [], current: [] };
  const vis = S.visible, match = S.match;
  let drawn = 0;
  for (let i = 0; i < S.n; i++) {
    if (!vis[i]) continue;
    const sx = S.pos[2 * i] * scale + tx, sy = S.pos[2 * i + 1] * scale + ty;
    const r = S.radius[i] * ns;
    if (sx < -r || sy < -r || sx > S.w + r || sy > S.h + r) continue;
    drawn++;
    let key;
    if (i === S.current) key = 'current';
    else if (hs && hs.set[i]) key = 'neighbor';
    else if (hs && !hs.set[i]) key = 'dim';
    else if (match && !match[i]) key = 'dim';
    else if (S.visited.has(S.nodes[i].path)) key = 'visited';
    else key = 'def';
    groups[key].push(sx, sy, r);
  }
  S.drawnNodes = drawn;
  const fillGroup = (arr, color, alpha) => {
    if (!arr.length) return;
    ctx.globalAlpha = alpha; ctx.fillStyle = color; ctx.beginPath();
    for (let k = 0; k < arr.length; k += 3) { ctx.moveTo(arr[k] + arr[k + 2], arr[k + 1]); ctx.arc(arr[k], arr[k + 1], arr[k + 2], 0, Math.PI * 2); }
    ctx.fill();
  };
  fillGroup(groups.dim, TOKENS.node, TOKENS.dimAlpha);
  fillGroup(groups.def, TOKENS.node, 1);
  fillGroup(groups.visited, TOKENS.visited, 1);
  fillGroup(groups.neighbor, TOKENS.neighbor, 1);
  fillGroup(groups.current, TOKENS.current, 1);
  ctx.globalAlpha = 1;

  // 허브 테두리(빈 원)로 섹션 허브를 구분 — 색은 추가하지 않는다
  ctx.strokeStyle = TOKENS.bg; ctx.lineWidth = 1.5; ctx.beginPath();
  for (const id of S.hubs) {
    if (!vis[id]) continue;
    const sx = S.pos[2 * id] * scale + tx, sy = S.pos[2 * id + 1] * scale + ty, r = S.radius[id] * ns * 0.45;
    ctx.moveTo(sx + r, sy); ctx.arc(sx, sy, r, 0, Math.PI * 2);
  }
  ctx.stroke();

  // 포커스 링 (키보드 포커스 / ego 중심 / 선택)
  const ring = S.focus !== NONE ? S.focus : S.selected;
  if (ring !== NONE && vis[ring]) {
    const sx = S.pos[2 * ring] * scale + tx, sy = S.pos[2 * ring + 1] * scale + ty, r = S.radius[ring] * ns + 4;
    ctx.strokeStyle = TOKENS.accent; ctx.lineWidth = 2; ctx.beginPath(); ctx.arc(sx, sy, r, 0, Math.PI * 2); ctx.stroke();
  }

  // 라벨: zoom ≥ 1.2 이거나 차수 상위 노드만 (겹침 컬링은 wasm)
  const labelMask = hs ? andMask(vis, hs.set) : (match ? andMask(vis, match) : vis);
  const lv = wasm.label_visible_masked(scale, labelMask);
  ctx.font = `520 12px ${TOKENS.sans}`; ctx.textAlign = 'center'; ctx.textBaseline = 'top';
  ctx.fillStyle = TOKENS.label;
  for (let i = 0; i < S.n; i++) {
    if (!lv[i]) continue;
    const sx = S.pos[2 * i] * scale + tx, sy = S.pos[2 * i + 1] * scale + ty + S.radius[i] * ns + 2;
    ctx.fillText(ellipsis(S.nodes[i].title), sx, sy);
  }
  // 강조 중심 라벨은 항상
  if (hs && vis[hs.centre] && !lv[hs.centre]) {
    const i = hs.centre, sx = S.pos[2 * i] * scale + tx, sy = S.pos[2 * i + 1] * scale + ty + S.radius[i] * ns + 2;
    ctx.fillText(ellipsis(S.nodes[i].title), sx, sy);
  }
}
function andMask(a, b) { const o = new Uint8Array(a.length); for (let i = 0; i < a.length; i++) o[i] = a[i] & b[i]; return o; }
function ellipsis(s) { return s.length > 28 ? s.slice(0, 28) + '…' : s; }

// ---- 입력 -----------------------------------------------------------------
function bindUI() {
  new ResizeObserver(() => { resize(); requestDraw(); }).observe(canvas.parentElement);
  matchMedia('(prefers-color-scheme: dark)').addEventListener('change', () => { readTokens(); S.edgeDirty = true; requestDraw(); });

  // 포인터: 드래그 이동 / 핀치 확대 / 클릭 / 호버
  const pointers = new Map();
  let drag = null, pinch = null, moved = false;
  canvas.addEventListener('pointerdown', (e) => {
    canvas.setPointerCapture(e.pointerId);
    pointers.set(e.pointerId, { x: e.offsetX, y: e.offsetY });
    if (pointers.size === 1) { drag = { x: e.offsetX, y: e.offsetY, tx: S.view.tx, ty: S.view.ty }; moved = false; canvas.classList.add('dragging'); }
    else if (pointers.size === 2) { const [a, b] = [...pointers.values()]; pinch = { d: Math.hypot(a.x - b.x, a.y - b.y), scale: S.view.scale, cx: (a.x + b.x) / 2, cy: (a.y + b.y) / 2, tx: S.view.tx, ty: S.view.ty }; drag = null; }
  });
  canvas.addEventListener('pointermove', (e) => {
    if (pointers.has(e.pointerId)) pointers.set(e.pointerId, { x: e.offsetX, y: e.offsetY });
    if (pinch && pointers.size === 2) {
      const [a, b] = [...pointers.values()];
      const d = Math.hypot(a.x - b.x, a.y - b.y), k = d / pinch.d;
      const cx = (a.x + b.x) / 2, cy = (a.y + b.y) / 2;
      S.view.scale = clampScale(pinch.scale * k);
      const kk = S.view.scale / pinch.scale;
      S.view.tx = cx - (pinch.cx - pinch.tx) * kk; S.view.ty = cy - (pinch.cy - pinch.ty) * kk;
      S.edgeDirty = true; moved = true; requestDraw(); return;
    }
    if (drag) {
      const dx = e.offsetX - drag.x, dy = e.offsetY - drag.y;
      if (Math.abs(dx) + Math.abs(dy) > 3) moved = true;
      if (moved) { S.view.tx = drag.tx + dx; S.view.ty = drag.ty + dy; S.edgeDirty = true; hideTooltip(); requestDraw(); }
      return;
    }
    onHover(e.offsetX, e.offsetY, e.clientX, e.clientY);
  });
  const up = (e) => {
    pointers.delete(e.pointerId);
    if (pointers.size < 2) pinch = null;
    if (drag && !moved && e.type === 'pointerup') {
      const id = wasm.hit_test_masked(e.offsetX, e.offsetY, S.visible);
      if (id !== NONE) {
        if (e.shiftKey || e.altKey) { setEgo(id); }
        else { markVisited(id); location.href = S.nodes[id].url; }
      } else if (S.selected !== NONE && S.ego === NONE) { S.selected = NONE; updateFocusBox(); requestDraw(); }
    }
    if (pointers.size === 0) { drag = null; canvas.classList.remove('dragging'); }
  };
  canvas.addEventListener('pointerup', up); canvas.addEventListener('pointercancel', up);
  canvas.addEventListener('pointerleave', () => { if (!drag) { S.hover = NONE; hideTooltip(); canvas.classList.remove('over'); requestDraw(); } });
  canvas.addEventListener('wheel', (e) => {
    e.preventDefault();
    const k = Math.exp(-e.deltaY * (e.deltaMode === 1 ? 0.05 : 0.0015));
    zoomAt(e.offsetX, e.offsetY, k);
  }, { passive: false });
  canvas.addEventListener('dblclick', (e) => { zoomAt(e.offsetX, e.offsetY, 1.8); });

  // 툴바
  qEl.addEventListener('input', onSearch);
  qEl.addEventListener('keydown', (e) => {
    if (e.key === 'Enter') { e.preventDefault(); const first = listEl.querySelector('a'); if (first) first.focus(); }
    if (e.key === 'Escape') { qEl.value = ''; onSearch(); }
  });
  sectionEl.addEventListener('change', () => {
    const hub = S.hubIndex.get(sectionEl.value);
    if (hub === undefined) setScope('global', NONE); else setScope('local', hub);
  });
  scopeBtn.addEventListener('click', () => {
    if (S.scope === 'local') setScope('global', S.scopeHub); else setScope('local', S.scopeHub);
  });
  clearBtn.addEventListener('click', () => {
    qEl.value = ''; S.match = null; S.searchHits = null; S.ego = NONE; S.selected = NONE; S.focus = NONE;
    recomputeVisible(); fitToVisible(true); updateFocusBox(); renderList(); requestDraw();
  });
  fitBtn.addEventListener('click', () => { fitToVisible(true); requestDraw(); });
  focusEgoBtn.addEventListener('click', () => {
    const id = S.selected !== NONE ? S.selected : S.focus;
    if (id === NONE) return;
    if (S.ego === id) setEgo(NONE); else setEgo(id);
  });
  egoDepthEl.addEventListener('change', () => { if (S.ego !== NONE) setEgo(S.ego); });
  focusOpen.addEventListener('click', () => { if (S.selected !== NONE) markVisited(S.selected); });

  // 키보드: Tab 은 목록 링크를 순회(포커스 링은 캔버스에), Esc 해제, 0 맞춤
  document.addEventListener('keydown', (e) => {
    if (e.target === qEl || e.target.tagName === 'SELECT') return;
    if (e.key === 'Escape') { S.focus = NONE; S.hover = NONE; if (S.ego !== NONE) setEgo(NONE); hideTooltip(); requestDraw(); }
    else if (e.key === '0') { fitToVisible(true); requestDraw(); }
    else if (e.key === '/' ) { e.preventDefault(); qEl.focus(); }
    else if ((e.key === 'e' || e.key === 'E') && S.focus !== NONE) { setEgo(S.focus); }
  });
  listEl.addEventListener('focusin', (e) => {
    const li = e.target.closest('li[data-id]'); if (!li) return;
    S.focus = Number(li.dataset.id); S.selected = S.focus;
    for (const x of listEl.querySelectorAll('.is-focus')) x.classList.remove('is-focus');
    li.classList.add('is-focus');
    ensureOnScreen(S.focus); updateFocusBox(); requestDraw();
  });
  listEl.addEventListener('click', (e) => {
    const a = e.target.closest('a[data-id]'); if (a) markVisited(Number(a.dataset.id));
    const btn = e.target.closest('button[data-more]'); if (btn) { S.listOffset += LIST_PAGE; renderList(true); }
    const eb = e.target.closest('button[data-ego]'); if (eb) { e.preventDefault(); setEgo(Number(eb.dataset.ego)); }
  });
}
function clampScale(s) { return Math.min(12, Math.max(0.05, s)); }
function zoomAt(x, y, k) {
  const s0 = S.view.scale, s1 = clampScale(s0 * k), kk = s1 / s0;
  S.view.scale = s1; S.view.tx = x - (x - S.view.tx) * kk; S.view.ty = y - (y - S.view.ty) * kk;
  S.edgeDirty = true; requestDraw();
}
function ensureOnScreen(id) {
  const sx = S.pos[2 * id] * S.view.scale + S.view.tx, sy = S.pos[2 * id + 1] * S.view.scale + S.view.ty;
  if (sx < 20 || sy < 20 || sx > S.w - 20 || sy > S.h - 20) {
    const target = { scale: S.view.scale, tx: S.w / 2 - S.pos[2 * id] * S.view.scale, ty: S.h / 2 - S.pos[2 * id + 1] * S.view.scale };
    if (reducedMotion) { S.view = target; S.edgeDirty = true; } else animateView(target);
  }
}

function onHover(x, y, cx, cy) {
  const id = wasm.hit_test_masked(x, y, S.visible);
  if (id !== S.hover) { S.hover = id; canvas.classList.toggle('over', id !== NONE); requestDraw(); }
  if (id === NONE) { hideTooltip(); return; }
  const nd = S.nodes[id];
  tooltip.innerHTML = '';
  const strong = document.createElement('strong'); strong.textContent = nd.title;
  const small = document.createElement('small'); small.textContent = `${nd.hub ? '섹션 허브' : nd.section} · 링크 ${nd.degree}${nd.hub ? '' : ' · ' + nd.path}`;
  tooltip.append(strong, small); tooltip.hidden = false;
  const rect = canvas.getBoundingClientRect();
  const px = cx - rect.left + 14, py = cy - rect.top + 14;
  tooltip.style.left = Math.min(px, S.w - tooltip.offsetWidth - 8) + 'px';
  tooltip.style.top = Math.min(py, S.h - tooltip.offsetHeight - 8) + 'px';
}
function hideTooltip() { tooltip.hidden = true; }
function markVisited(id) {
  S.visited.add(S.nodes[id].path);
  try { localStorage.setItem(VISITED_KEY, JSON.stringify([...S.visited].slice(-500))); } catch { /* ignore */ }
}

// ---- 검색 -----------------------------------------------------------------
let searchTimer = 0;
function onSearch() {
  clearTimeout(searchTimer);
  searchTimer = setTimeout(() => {
    const q = qEl.value.trim();
    if (!q) { S.match = null; S.searchHits = null; }
    else {
      const hits = wasm.search(q);
      const m = new Uint8Array(S.n); for (const id of hits) m[id] = 1;
      S.match = m; S.searchHits = hits;
    }
    S.listOffset = 0; renderList(); requestDraw();
  }, 60);
}

// ---- 텍스트 목록 (스크린리더/JS 없음 폴백의 실체) -----------------------------
function listIds() {
  if (S.searchHits) return { ids: Array.from(S.searchHits), title: `검색 결과`, meta: `"${qEl.value.trim()}" — ${S.searchHits.length} 건 (초성 일치 포함)` };
  if (S.ego !== NONE) {
    const ids = Array.from(wasm.ego(S.ego, S.egoDepth)).filter((i) => S.visible[i]);
    return { ids, title: `이웃 (깊이 ${S.egoDepth})`, meta: `${S.nodes[S.ego].title} 에서 ${S.egoDepth} 홉 이내 ${ids.length} 개` };
  }
  if (S.selected !== NONE) {
    const ids = [S.selected, ...Array.from(wasm.neighbors(S.selected))];
    return { ids, title: '백링크 / 이웃', meta: `${S.nodes[S.selected].title} — 직접 링크 ${ids.length - 1} 개` };
  }
  const ids = []; for (let i = 0; i < S.n; i++) if (S.visible[i]) ids.push(i);
  ids.sort((a, b) => (S.nodes[b].hub - S.nodes[a].hub) || (S.nodes[b].degree - S.nodes[a].degree) || a - b);
  const scope = S.scope === 'local' && S.scopeHub !== NONE ? `${S.nodes[S.scopeHub].title} ±1 홉` : '전체';
  return { ids, title: `노트 목록 — ${scope}`, meta: `${ids.length} 개 (허브 · 링크 많은 순)` };
}
function renderList(append = false) {
  const { ids, title, meta } = listIds();
  listTitle.textContent = title; listMeta.textContent = meta;
  if (!append) { listEl.innerHTML = ''; S.listOffset = 0; }
  else listEl.querySelector('.gh-list-more')?.remove();
  const frag = document.createDocumentFragment();
  const end = Math.min(ids.length, S.listOffset + LIST_PAGE);
  for (let k = S.listOffset; k < end; k++) {
    const id = ids[k], nd = S.nodes[id];
    const li = document.createElement('li'); li.dataset.id = id;
    if (nd.hub) li.classList.add('is-hub'); if (id === S.current) li.classList.add('is-current'); if (id === S.focus) li.classList.add('is-focus');
    const a = document.createElement('a'); a.href = nd.url; a.dataset.id = id; a.textContent = nd.title;
    a.title = nd.hub ? `${nd.title} 섹션` : nd.path;
    if (id === S.current) a.setAttribute('aria-current', 'page');
    const sec = document.createElement('span'); sec.className = 'sec'; sec.textContent = nd.hub ? (id === S.rootId ? 'root' : 'hub') : nd.section;
    const deg = document.createElement('span'); deg.className = 'deg'; deg.textContent = String(nd.degree);
    const eb = document.createElement('button'); eb.type = 'button'; eb.className = 'gh-btn'; eb.dataset.ego = id; eb.textContent = '이웃'; eb.setAttribute('aria-label', `${nd.title} 의 이웃만 보기`); eb.style.padding = '0 6px'; eb.style.fontSize = '11px';
    li.append(a, sec, deg, eb); frag.append(li);
  }
  if (end < ids.length) {
    const li = document.createElement('li'); li.className = 'gh-list-more';
    const b = document.createElement('button'); b.type = 'button'; b.dataset.more = '1'; b.textContent = `더 보기 (${ids.length - end} 개 남음)`;
    li.append(b); frag.append(li);
  }
  if (!ids.length) { const li = document.createElement('li'); li.className = 'gh-list-empty'; li.textContent = '일치하는 노트가 없습니다.'; frag.append(li); }
  listEl.append(frag);
}
function updateFocusBox() {
  const id = S.selected !== NONE ? S.selected : S.focus;
  if (id === NONE) { focusBox.hidden = true; return; }
  const nd = S.nodes[id];
  focusBox.hidden = false; focusTitle.textContent = nd.title;
  focusMeta.textContent = nd.hub ? `섹션 허브 · 노트 ${nd.degree - (id === S.rootId ? 0 : 1)} 개` : `${nd.section} · 링크 ${nd.degree} · ${nd.path}`;
  focusOpen.href = nd.url;
  focusEgoBtn.setAttribute('aria-pressed', String(S.ego === id));
  focusEgoBtn.textContent = S.ego === id ? '이웃 보기 해제' : '이웃만 보기';
}

main().catch((e) => fail(String(e && e.stack || e)));
