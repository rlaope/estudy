//! ForceAtlas2 레이아웃 (네이티브 전용).
//!
//! - `fa2` 크레이트(MIT)를 사용한다. AGPL 인 `forceatlas2` 크레이트는 쓰지 않는다.
//! - 초기 좌표는 고정 시드 splitmix64 로 만들고, 반복 횟수를 고정한다.
//! - fa2 의 순차 경로만 사용한다(`parallel=false`). 병렬 경로는 사용하지 않는다.
//!   → 같은 입력에 대해 바이트 단위로 같은 결과가 나온다.

pub const DEFAULT_SEED: u64 = 20_260_917;
pub const DEFAULT_ITERATIONS: usize = 300;

/// splitmix64 — 결정론적 PRNG. 표준 라이브러리 밖 의존성 없이 초기 좌표를 만든다.
pub struct SplitMix64(u64);

impl SplitMix64 {
    pub fn new(seed: u64) -> Self {
        Self(seed)
    }
    pub fn next_u64(&mut self) -> u64 {
        self.0 = self.0.wrapping_add(0x9E37_79B9_7F4A_7C15);
        let mut z = self.0;
        z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
        z ^ (z >> 31)
    }
    /// [0, 1) 구간의 f32
    pub fn next_f32(&mut self) -> f32 {
        ((self.next_u64() >> 40) as f32) / ((1u32 << 24) as f32)
    }
}

/// 섹션 순서로 정렬한 뒤 골든앵글 나선에 배치한다.
///
/// - 밀도가 면적 균등(디스크 전체에 고르게) → 사용자가 보는 "골고루 퍼진" 분포
/// - 같은 섹션 노트가 나선에서 연속으로 놓여 서로 가깝다 → 링크가 길게 흩어지지 않는다
/// - 좌표는 결정론적(시드는 회전 위상만 바꾼다)
pub fn spiral_by_section(node_count: usize, section_of: &[u32], seed: u64) -> Vec<(f32, f32)> {
    const GOLDEN_ANGLE: f32 = 2.399_963_2;
    let n = node_count.max(1) as f32;
    let phase = ((seed % 9973) as f32) / 9973.0 * std::f32::consts::TAU;

    let mut order: Vec<usize> = (0..node_count).collect();
    order.sort_by_key(|i| (section_of.get(*i).copied().unwrap_or(0), *i));

    let mut out = vec![(0.0f32, 0.0f32); node_count];
    for (rank, node) in order.iter().enumerate() {
        let t = (rank as f32 + 0.5) / n;
        let r = (t.sqrt() * 0.96 + 0.04).min(1.0); // 안쪽 0.04 ~ 바깥 1.0
        let a = rank as f32 * GOLDEN_ANGLE + phase;
        out[*node] = (r * a.cos(), r * a.sin());
    }
    out
}

/// 초기 좌표 — 골든앵글 나선으로 디스크에 고르게 깔아 둔다.
///
/// 난수 초기값은 FA2 가 국소 최적에 갇혀 덩어리(허브 주변 뭉침)를 만들었다.
/// 면적 균등한 나선 배치는 시작부터 고르게 퍼져 있고, 시드에 따라 회전만 달라진다(결정론 유지).
pub fn initial_positions(node_count: usize, seed: u64) -> Vec<(f32, f32)> {
    const GOLDEN_ANGLE: f32 = 2.399_963_2; // 황금각(라디안)
    let n = node_count.max(1) as f32;
    let phase = ((seed % 9973) as f32) / 9973.0 * std::f32::consts::TAU;
    (0..node_count)
        .map(|i| {
            let t = (i as f32 + 0.5) / n;
            let r = t.sqrt(); // 면적 균등
            let a = i as f32 * GOLDEN_ANGLE + phase;
            (r * a.cos(), r * a.sin())
        })
        .collect()
}

/// 평균 중심 이동 + 98퍼센타일 반경을 1.0 으로 정규화 → 화면을 고르게 채운다(이상치에 끌려가지 않게).
pub fn normalize(pts: &mut [(f32, f32)]) {
    let n = pts.len() as f32;
    if n == 0.0 {
        return;
    }
    let (mut cx, mut cy) = (0.0f32, 0.0f32);
    for (x, y) in pts.iter() {
        cx += x;
        cy += y;
    }
    cx /= n;
    cy /= n;
    let mut radii: Vec<f32> = pts
        .iter()
        .map(|(x, y)| ((x - cx).powi(2) + (y - cy).powi(2)).sqrt())
        .collect();
    radii.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
    let idx = ((radii.len() as f32 * 0.98) as usize).min(radii.len() - 1);
    let scale = if radii[idx] > 1e-6 { 1.0 / radii[idx] } else { 1.0 };
    for p in pts.iter_mut() {
        p.0 = (p.0 - cx) * scale;
        p.1 = (p.1 - cy) * scale;
    }
}

/// 레이아웃 결과 좌표 (노드 id 순서).
#[cfg(feature = "native-layout")]
pub fn force_atlas2(
    node_count: usize,
    edges: &[(u32, u32)],
    seed: u64,
    iterations: usize,
) -> Vec<(f32, f32)> {
    use fa2::{FA2Data, FA2Settings};

    let init = initial_positions(node_count, seed);

    let mut data = FA2Data::<f32>::with_capacity(node_count, edges.len());
    for (x, y) in &init {
        data.add_node_with_position(*x, *y);
    }
    for (u, v) in edges {
        data.add_edge(*u as usize, *v as usize);
    }

    // 고른 분포를 위해: 약한 중력(0.02) + strong_gravity 끔 + pairwise repulsion.
    // strong_gravity 는 모든 노드를 중심으로 끌어당겨 덩어리를 만든다(예전 기본값 0.05/on).
    let settings = FA2Settings::<f32>::from_graph_order(node_count)
        .with_pairwise_repulsion()
        .gravity(0.02)
        .strong_gravity_mode(false)
        .scaling_ratio(10.0)
        .parallel(false);

    let mut layout = settings.build(data);
    layout.run(iterations);
    let mut positions: Vec<(f32, f32)> = layout.into_data().positions().collect();
    normalize(&mut positions);
    positions
}

#[cfg(not(feature = "native-layout"))]
pub fn force_atlas2(
    _node_count: usize,
    _edges: &[(u32, u32)],
    _seed: u64,
    _iterations: usize,
) -> Vec<(f32, f32)> {
    panic!("native-layout feature 가 꺼져 있다: 레이아웃은 네이티브 빌드에서만 계산한다");
}

/// f32 슬라이스를 little-endian 바이트로 만든다.
pub fn positions_to_le_bytes(positions: &[(f32, f32)]) -> Vec<u8> {
    let mut out = Vec::with_capacity(positions.len() * 8);
    for (x, y) in positions {
        out.extend_from_slice(&x.to_le_bytes());
        out.extend_from_slice(&y.to_le_bytes());
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn prng_is_stable() {
        let mut a = SplitMix64::new(20_260_917);
        let mut b = SplitMix64::new(20_260_917);
        for _ in 0..8 {
            assert_eq!(a.next_u64(), b.next_u64());
        }
    }
}
