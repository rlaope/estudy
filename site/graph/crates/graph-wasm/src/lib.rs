//! estudy 지식 그래프 — 쿼리 측 WASM 헬퍼.
//!
//! 레이아웃은 빌드타임(L1, `tools/graph`)에 끝났다. 이 크레이트는 브라우저에서
//! `site/generated/{pos.bin,graph.bin,search.json}` 을 받은 뒤의 **질의만** 담당한다:
//! 검색 점수화(초성 포함), ego 이웃 추출, 히트테스트, 라벨 컬링. 렌더링은 JS/canvas2D.
//!
//! 스레드/rayon/SharedArrayBuffer 는 쓰지 않는다(GitHub Pages 는 COOP/COEP 를 못 건다).

use std::cell::RefCell;
use wasm_bindgen::prelude::*;

const NONE: u32 = u32::MAX;
const MAX_RADIUS: f32 = 12.0;
const LABEL_ZOOM: f32 = 1.2;
const LABEL_TOP_DEGREE: usize = 24;
const LABEL_PX: f32 = 12.0;
/// ego/BFS 가 이 차수를 넘는 노드(루트 허브, README 색인)를 *거쳐* 확장하지 않는다 — 안 그러면 depth 2 가 전체 그래프가 된다.
const EXPAND_DEGREE_CUTOFF: u32 = 300;

/// 초성 19개 (호환 자모). `(음절 - 0xAC00) / 588` 이 인덱스.
const CHOSEONG: [char; 19] = [
    'ㄱ', 'ㄲ', 'ㄴ', 'ㄷ', 'ㄸ', 'ㄹ', 'ㅁ', 'ㅂ', 'ㅃ', 'ㅅ', 'ㅆ', 'ㅇ', 'ㅈ', 'ㅉ', 'ㅊ', 'ㅋ',
    'ㅌ', 'ㅍ', 'ㅎ',
];

struct Node {
    title: String,     // lowercase(ASCII); 입력은 L1 이 이미 NFC
    cho: String,       // choseong, lowercase, 공백 포함
    cho_tight: String, // choseong, 공백 제거
}

struct View {
    scale: f32,
    tx: f32,
    ty: f32,
    width: f32,
    height: f32,
    node_scale: f32,
}

#[derive(Default)]
struct Graph {
    pos: Vec<f32>,     // [x,y] * n
    offsets: Vec<u32>, // n+1
    targets: Vec<u32>, // offsets[n]
    radius: Vec<f32>,  // per node, CSS px at zoom 1
    nodes: Vec<Node>,
    by_degree: Vec<u32>, // node ids, degree desc
    view: Option<View>,
    root: u32,
}

thread_local! {
    static G: RefCell<Graph> = RefCell::new(Graph::default());
}

fn to_choseong(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    for ch in s.chars() {
        let cp = ch as u32;
        if (0xAC00..=0xD7A3).contains(&cp) {
            out.push(CHOSEONG[((cp - 0xAC00) / 588) as usize]);
        } else {
            out.push(ch);
        }
    }
    out
}

fn is_jamo(ch: char) -> bool {
    ('\u{3131}'..='\u{318E}').contains(&ch)
}

fn has_syllable(s: &str) -> bool {
    s.chars().any(|c| ('\u{AC00}'..='\u{D7A3}').contains(&c))
}

/// 그래프 적재. `titles` 는 노드 순서대로 `'\n'` 로 이어 붙인 한 문자열(허브 포함, 정확히 N개).
/// `choseong` 도 같은 형식(허브는 빈 문자열이어도 된다 — 제목에서 다시 만든다).
/// 형식이 CONTRACT.md 와 어긋나면 false 를 돌려주고 아무것도 적재하지 않는다.
#[wasm_bindgen]
pub fn load(pos: &[f32], csr: &[u32], titles: &str, choseong: &str, root: u32) -> bool {
    if pos.len() < 2 || pos.len() % 2 != 0 {
        return false;
    }
    let n = pos.len() / 2;
    if csr.len() < n + 1 {
        return false;
    }
    let offsets = &csr[..n + 1];
    let e = offsets[n] as usize;
    if csr.len() != n + 1 + e || offsets[0] != 0 {
        return false;
    }
    if offsets.windows(2).any(|w| w[0] > w[1]) {
        return false;
    }
    let targets = &csr[n + 1..];
    if targets.iter().any(|&t| t as usize >= n) {
        return false;
    }
    let title_list: Vec<&str> = titles.split('\n').collect();
    if title_list.len() != n {
        return false;
    }
    let cho_list: Vec<&str> = choseong.split('\n').collect();

    let mut nodes = Vec::with_capacity(n);
    let mut radius = Vec::with_capacity(n);
    for i in 0..n {
        let title: String = title_list[i].to_ascii_lowercase();
        let raw_cho = cho_list.get(i).copied().unwrap_or("");
        let cho = if raw_cho.is_empty() {
            to_choseong(&title)
        } else {
            raw_cho.to_ascii_lowercase()
        };
        let cho_tight: String = cho.chars().filter(|c| !c.is_whitespace()).collect();
        nodes.push(Node {
            title,
            cho,
            cho_tight,
        });
        let deg = (offsets[i + 1] - offsets[i]) as f32;
        radius.push((3.0 + 2.0 * deg.sqrt()).min(MAX_RADIUS));
    }
    let mut by_degree: Vec<u32> = (0..n as u32).collect();
    by_degree.sort_by(|&a, &b| {
        let da = offsets[a as usize + 1] - offsets[a as usize];
        let db = offsets[b as usize + 1] - offsets[b as usize];
        db.cmp(&da).then(a.cmp(&b))
    });

    G.with(|g| {
        let mut g = g.borrow_mut();
        g.pos = pos.to_vec();
        g.offsets = offsets.to_vec();
        g.targets = targets.to_vec();
        g.radius = radius;
        g.nodes = nodes;
        g.by_degree = by_degree;
        g.view = None;
        g.root = root;
    });
    true
}

/// `[nodes, csr_targets, undirected_edges]`
#[wasm_bindgen]
pub fn stats() -> Vec<u32> {
    G.with(|g| {
        let g = g.borrow();
        let n = g.offsets.len().saturating_sub(1) as u32;
        let e = g.targets.len() as u32;
        vec![n, e, e / 2]
    })
}

/// 노드 반지름(CSS px, zoom 1 기준): `min(12, 3 + 2*sqrt(degree))`
#[wasm_bindgen]
pub fn radii() -> Vec<f32> {
    G.with(|g| g.borrow().radius.clone())
}

/// 월드→화면 변환: `sx = x*scale + tx`, `sy = y*scale + ty` (CSS px). 히트테스트/라벨 컬링이 쓴다.
/// `node_scale` 은 화면 반지름 배율(JS 렌더러와 같은 값을 넘긴다).
#[wasm_bindgen]
pub fn set_view(scale: f32, tx: f32, ty: f32, width: f32, height: f32, node_scale: f32) {
    G.with(|g| {
        g.borrow_mut().view = Some(View {
            scale,
            tx,
            ty,
            width,
            height,
            node_scale,
        });
    });
}

/// 검색. 점수 내림차순 노드 id. 빈 질의 → 빈 배열.
///
/// - 제목 부분 문자열(대소문자 무시, NFC)
/// - 초성 질의(`ㅈㄱㅎ`): 초성 문자열(공백 제거)에 대한 부분 일치
/// - 음절 질의를 초성으로 바꿔 초성열과 대조 (띄어쓰기 차이 흡수)
/// - 공백으로 나뉜 토큰이 모두 제목/초성에 있으면 약한 일치
#[wasm_bindgen]
pub fn search(query: &str) -> Vec<u32> {
    let q: String = query.to_ascii_lowercase();
    let q = q.trim();
    if q.is_empty() {
        return Vec::new();
    }
    let q_tight: String = q.chars().filter(|c| !c.is_whitespace()).collect();
    let q_cho = to_choseong(&q_tight);
    let q_is_jamo = q_tight
        .chars()
        .all(|c| is_jamo(c) || c.is_ascii_alphanumeric());
    let q_has_syl = has_syllable(q);
    let tokens: Vec<&str> = q.split_whitespace().collect();

    G.with(|g| {
        let g = g.borrow();
        let mut hits: Vec<(u32, u32)> = Vec::new(); // (score, id)
        for (i, node) in g.nodes.iter().enumerate() {
            let mut score = 0u32;
            if let Some(p) = node.title.find(q) {
                score = if p == 0 {
                    400
                } else if word_start(&node.title, p) {
                    300
                } else {
                    200
                };
            } else if q_is_jamo {
                if let Some(p) = node.cho_tight.find(q_tight.as_str()) {
                    score = if p == 0 { 160 } else { 120 };
                } else if node.cho.find(q).is_some() {
                    score = 110;
                }
            } else if q_has_syl && node.cho_tight.find(q_cho.as_str()).is_some() {
                score = 80;
            }
            if score == 0 && tokens.len() > 1 {
                let all = tokens.iter().all(|t| {
                    node.title.contains(*t)
                        || node.cho_tight.contains(*t)
                        || node.cho_tight.contains(to_choseong(t).as_str())
                });
                if all {
                    score = 60;
                }
            }
            if score > 0 {
                let deg = g.offsets[i + 1] - g.offsets[i];
                hits.push((score * 4096 + deg.min(4095), i as u32));
            }
        }
        hits.sort_by(|a, b| b.0.cmp(&a.0).then(a.1.cmp(&b.1)));
        hits.into_iter().map(|(_, id)| id).collect()
    })
}

fn word_start(s: &str, byte_pos: usize) -> bool {
    s[..byte_pos]
        .chars()
        .next_back()
        .map(|c| !c.is_alphanumeric())
        .unwrap_or(true)
}

/// 직접 이웃(CSR 순서).
#[wasm_bindgen]
pub fn neighbors(node_id: u32) -> Vec<u32> {
    G.with(|g| {
        let g = g.borrow();
        let n = g.offsets.len().saturating_sub(1);
        let v = node_id as usize;
        if v >= n {
            return Vec::new();
        }
        g.targets[g.offsets[v] as usize..g.offsets[v + 1] as usize].to_vec()
    })
}

/// ego 네트워크: 중심에서 `depth` 홉 이내 노드 id, BFS 순서(중심이 첫 원소).
/// 루트 허브·README 색인처럼 차수가 `EXPAND_DEGREE_CUTOFF` 를 넘는 노드는 포함하되 그 너머로 확장하지 않는다
/// (모든 섹션이 루트/README 로 이어져 depth 2 면 전체가 되기 때문).
#[wasm_bindgen]
pub fn ego(node_id: u32, depth: u32) -> Vec<u32> {
    G.with(|g| {
        let g = g.borrow();
        bfs(&g, &[node_id], depth, true)
    })
}

/// 각 노드까지의 홉 수. 도달 불가 = 255. `ego()` 와 같은 규칙.
#[wasm_bindgen]
pub fn ego_depths(node_id: u32, depth: u32) -> Vec<u8> {
    G.with(|g| {
        let g = g.borrow();
        let n = g.offsets.len().saturating_sub(1);
        let mut dist = vec![255u8; n];
        if (node_id as usize) >= n {
            return dist;
        }
        dist[node_id as usize] = 0;
        let mut frontier = vec![node_id];
        for d in 1..=depth.min(254) {
            let mut next = Vec::new();
            for &u in &frontier {
                if u != node_id && !expandable(&g, u) {
                    continue;
                }
                let (a, b) = (
                    g.offsets[u as usize] as usize,
                    g.offsets[u as usize + 1] as usize,
                );
                for &w in &g.targets[a..b] {
                    if dist[w as usize] == 255 {
                        dist[w as usize] = d as u8;
                        next.push(w);
                    }
                }
            }
            if next.is_empty() {
                break;
            }
            frontier = next;
        }
        dist
    })
}

/// 로컬 뷰: 섹션 허브 `hub_id` 의 노트들 + 그 노트들의 1홉 이웃(다른 섹션 노트/허브 포함), 루트 허브 제외.
#[wasm_bindgen]
pub fn local_view(hub_id: u32) -> Vec<u32> {
    G.with(|g| {
        let g = g.borrow();
        let n = g.offsets.len().saturating_sub(1);
        if (hub_id as usize) >= n {
            return Vec::new();
        }
        let mut seen = vec![false; n];
        let mut out = Vec::new();
        seen[hub_id as usize] = true;
        out.push(hub_id);
        let members: Vec<u32> = neighbors_of(&g, hub_id)
            .iter()
            .copied()
            .filter(|&v| v != g.root)
            .collect();
        for &m in &members {
            if !seen[m as usize] {
                seen[m as usize] = true;
                out.push(m);
            }
        }
        for &m in &members {
            for &w in neighbors_of(&g, m) {
                if w != g.root && !seen[w as usize] {
                    seen[w as usize] = true;
                    out.push(w);
                }
            }
        }
        out
    })
}

fn expandable(g: &Graph, v: u32) -> bool {
    let deg = g.offsets[v as usize + 1] - g.offsets[v as usize];
    v != g.root && deg <= EXPAND_DEGREE_CUTOFF
}

fn neighbors_of(g: &Graph, v: u32) -> &[u32] {
    let (a, b) = (
        g.offsets[v as usize] as usize,
        g.offsets[v as usize + 1] as usize,
    );
    &g.targets[a..b]
}

fn bfs(g: &Graph, seeds: &[u32], depth: u32, skip_root: bool) -> Vec<u32> {
    let n = g.offsets.len().saturating_sub(1);
    let mut seen = vec![false; n];
    let mut out = Vec::new();
    for &s in seeds {
        if (s as usize) < n && !seen[s as usize] {
            seen[s as usize] = true;
            out.push(s);
        }
    }
    let mut start = 0;
    for _ in 0..depth {
        let end = out.len();
        if start == end {
            break;
        }
        for i in start..end {
            let u = out[i];
            if skip_root && !seeds.contains(&u) && !expandable(g, u) {
                continue;
            }
            for &w in neighbors_of(g, u) {
                if !seen[w as usize] {
                    seen[w as usize] = true;
                    out.push(w);
                }
            }
        }
        start = end;
    }
    out
}

/// 화면 좌표(CSS px)에서 가장 가까운 노드. 없으면 `u32::MAX`.
/// `mask` 가 비어 있지 않으면 `mask[id] != 0` 인 노드만 후보(현재 뷰에 보이는 노드).
#[wasm_bindgen]
pub fn hit_test_masked(x: f32, y: f32, mask: &[u8]) -> u32 {
    G.with(|g| {
        let g = g.borrow();
        let view = match &g.view {
            Some(v) => v,
            None => return NONE,
        };
        let n = g.radius.len();
        let mut best = NONE;
        let mut best_d = f32::INFINITY;
        for i in 0..n {
            if !mask.is_empty() && mask[i] == 0 {
                continue;
            }
            let sx = g.pos[2 * i] * view.scale + view.tx;
            let sy = g.pos[2 * i + 1] * view.scale + view.ty;
            let r = g.radius[i] * view.node_scale + 3.0; // 여유 3px
            let dx = sx - x;
            let dy = sy - y;
            let d = dx * dx + dy * dy;
            if d <= r * r && d < best_d {
                best_d = d;
                best = i as u32;
            }
        }
        best
    })
}

#[wasm_bindgen]
pub fn hit_test(x: f32, y: f32) -> u32 {
    hit_test_masked(x, y, &[])
}

/// 라벨 표시 여부(노드별 0/1).
/// - zoom ≥ 1.2: 화면 안 노드 전부 후보, 차수 내림차순으로 겹치는 라벨은 걸러낸다(그리디 AABB).
/// - zoom < 1.2: 차수 상위 `LABEL_TOP_DEGREE` 만 후보(역시 겹침 컬링).
/// `mask` 가 비어 있지 않으면 `mask[id] != 0` 인 노드만 후보.
#[wasm_bindgen]
pub fn label_visible_masked(zoom: f32, mask: &[u8]) -> Vec<u8> {
    G.with(|g| {
        let g = g.borrow();
        let n = g.radius.len();
        let mut out = vec![0u8; n];
        let view = match &g.view {
            Some(v) => v,
            None => return out,
        };
        let candidates: &[u32] = if zoom >= LABEL_ZOOM {
            &g.by_degree[..]
        } else {
            &g.by_degree[..g.by_degree.len().min(LABEL_TOP_DEGREE)]
        };
        let mut placed: Vec<[f32; 4]> = Vec::with_capacity(64);
        for &id in candidates {
            let i = id as usize;
            if !mask.is_empty() && mask[i] == 0 {
                continue;
            }
            let sx = g.pos[2 * i] * view.scale + view.tx;
            let sy = g.pos[2 * i + 1] * view.scale + view.ty;
            if sx < -40.0 || sy < -40.0 || sx > view.width + 40.0 || sy > view.height + 40.0 {
                continue;
            }
            let w = label_width(&g.nodes[i].title);
            let r = g.radius[i] * view.node_scale;
            // 라벨은 노드 아래 중앙
            let x0 = sx - w * 0.5;
            let y0 = sy + r + 2.0;
            let rect = [x0 - 2.0, y0 - 1.0, x0 + w + 2.0, y0 + LABEL_PX + 3.0];
            if placed.iter().any(|p| overlaps(p, &rect)) {
                continue;
            }
            placed.push(rect);
            out[i] = 1;
        }
        out
    })
}

#[wasm_bindgen]
pub fn label_visible(zoom: f32) -> Vec<u8> {
    label_visible_masked(zoom, &[])
}

fn label_width(title: &str) -> f32 {
    // 12px Pretendard 기준 근사: 한글/CJK ≈ 12px, 그 외 ≈ 6.6px. 28자 넘으면 말줄임(JS 와 동일 규칙).
    let mut w = 0.0f32;
    for (k, ch) in title.chars().enumerate() {
        if k >= 28 {
            w += 8.0;
            break;
        }
        let cp = ch as u32;
        w += if cp >= 0x1100 { 12.0 } else { 6.6 };
    }
    w
}

fn overlaps(a: &[f32; 4], b: &[f32; 4]) -> bool {
    a[0] < b[2] && b[0] < a[2] && a[1] < b[3] && b[1] < a[3]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn choseong_of_syllables() {
        assert_eq!(to_choseong("정규화"), "ㅈㄱㅎ");
        assert_eq!(to_choseong("ORM의 개념"), "ORMㅇ ㄱㄴ");
    }
}
