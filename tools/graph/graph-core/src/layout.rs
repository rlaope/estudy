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

/// 시드 기반 초기 좌표 ([-1, 1] 범위).
pub fn initial_positions(node_count: usize, seed: u64) -> Vec<(f32, f32)> {
    let mut rng = SplitMix64::new(seed);
    (0..node_count)
        .map(|_| (rng.next_f32() * 2.0 - 1.0, rng.next_f32() * 2.0 - 1.0))
        .collect()
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

    // from_graph_order(910): pairwise repulsion + strong gravity(0.05) + scaling_ratio 10
    // + slow_down = 1 + ln(N). 반복 횟수는 고정, 병렬 경로는 끈다.
    let settings = FA2Settings::<f32>::from_graph_order(node_count)
        .with_pairwise_repulsion()
        .parallel(false);

    let mut layout = settings.build(data);
    layout.run(iterations);
    layout.into_data().positions().collect()
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
