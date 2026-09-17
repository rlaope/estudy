/* tslint:disable */
/* eslint-disable */

/**
 * ego 네트워크: 중심에서 `depth` 홉 이내 노드 id, BFS 순서(중심이 첫 원소).
 * 루트 허브·README 색인처럼 차수가 `EXPAND_DEGREE_CUTOFF` 를 넘는 노드는 포함하되 그 너머로 확장하지 않는다
 * (모든 섹션이 루트/README 로 이어져 depth 2 면 전체가 되기 때문).
 */
export function ego(node_id: number, depth: number): Uint32Array;

/**
 * 각 노드까지의 홉 수. 도달 불가 = 255. `ego()` 와 같은 규칙.
 */
export function ego_depths(node_id: number, depth: number): Uint8Array;

export function hit_test(x: number, y: number): number;

/**
 * 화면 좌표(CSS px)에서 가장 가까운 노드. 없으면 `u32::MAX`.
 * `mask` 가 비어 있지 않으면 `mask[id] != 0` 인 노드만 후보(현재 뷰에 보이는 노드).
 */
export function hit_test_masked(x: number, y: number, mask: Uint8Array): number;

export function label_visible(zoom: number): Uint8Array;

/**
 * 라벨 표시 여부(노드별 0/1).
 * - zoom ≥ 1.2: 화면 안 노드 전부 후보, 차수 내림차순으로 겹치는 라벨은 걸러낸다(그리디 AABB).
 * - zoom < 1.2: 차수 상위 `LABEL_TOP_DEGREE` 만 후보(역시 겹침 컬링).
 * `mask` 가 비어 있지 않으면 `mask[id] != 0` 인 노드만 후보.
 */
export function label_visible_masked(zoom: number, mask: Uint8Array): Uint8Array;

/**
 * 그래프 적재. `titles` 는 노드 순서대로 `'\n'` 로 이어 붙인 한 문자열(허브 포함, 정확히 N개).
 * `choseong` 도 같은 형식(허브는 빈 문자열이어도 된다 — 제목에서 다시 만든다).
 * 형식이 CONTRACT.md 와 어긋나면 false 를 돌려주고 아무것도 적재하지 않는다.
 */
export function load(pos: Float32Array, csr: Uint32Array, titles: string, choseong: string, root: number): boolean;

/**
 * 로컬 뷰: 섹션 허브 `hub_id` 의 노트들 + 그 노트들의 1홉 이웃(다른 섹션 노트/허브 포함), 루트 허브 제외.
 */
export function local_view(hub_id: number): Uint32Array;

/**
 * 직접 이웃(CSR 순서).
 */
export function neighbors(node_id: number): Uint32Array;

/**
 * 노드 반지름(CSS px, zoom 1 기준): `min(12, 3 + 2*sqrt(degree))`
 */
export function radii(): Float32Array;

/**
 * 검색. 점수 내림차순 노드 id. 빈 질의 → 빈 배열.
 *
 * - 제목 부분 문자열(대소문자 무시, NFC)
 * - 초성 질의(`ㅈㄱㅎ`): 초성 문자열(공백 제거)에 대한 부분 일치
 * - 음절 질의를 초성으로 바꿔 초성열과 대조 (띄어쓰기 차이 흡수)
 * - 공백으로 나뉜 토큰이 모두 제목/초성에 있으면 약한 일치
 */
export function search(query: string): Uint32Array;

/**
 * 월드→화면 변환: `sx = x*scale + tx`, `sy = y*scale + ty` (CSS px). 히트테스트/라벨 컬링이 쓴다.
 * `node_scale` 은 화면 반지름 배율(JS 렌더러와 같은 값을 넘긴다).
 */
export function set_view(scale: number, tx: number, ty: number, width: number, height: number, node_scale: number): void;

/**
 * `[nodes, csr_targets, undirected_edges]`
 */
export function stats(): Uint32Array;

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
    readonly memory: WebAssembly.Memory;
    readonly ego: (a: number, b: number, c: number) => void;
    readonly ego_depths: (a: number, b: number, c: number) => void;
    readonly hit_test: (a: number, b: number) => number;
    readonly hit_test_masked: (a: number, b: number, c: number, d: number) => number;
    readonly label_visible: (a: number, b: number) => void;
    readonly label_visible_masked: (a: number, b: number, c: number, d: number) => void;
    readonly load: (a: number, b: number, c: number, d: number, e: number, f: number, g: number, h: number, i: number) => number;
    readonly local_view: (a: number, b: number) => void;
    readonly neighbors: (a: number, b: number) => void;
    readonly radii: (a: number) => void;
    readonly search: (a: number, b: number, c: number) => void;
    readonly set_view: (a: number, b: number, c: number, d: number, e: number, f: number) => void;
    readonly stats: (a: number) => void;
    readonly __wbindgen_add_to_stack_pointer: (a: number) => number;
    readonly __wbindgen_export: (a: number, b: number, c: number) => void;
    readonly __wbindgen_export2: (a: number, b: number) => number;
    readonly __wbindgen_export3: (a: number, b: number, c: number, d: number) => number;
}

export type SyncInitInput = BufferSource | WebAssembly.Module;

/**
 * Instantiates the given `module`, which can either be bytes or
 * a precompiled `WebAssembly.Module`.
 *
 * @param {{ module: SyncInitInput }} module - Passing `SyncInitInput` directly is deprecated.
 *
 * @returns {InitOutput}
 */
export function initSync(module: { module: SyncInitInput } | SyncInitInput): InitOutput;

/**
 * If `module_or_path` is {RequestInfo} or {URL}, makes a request and
 * for everything else, calls `WebAssembly.instantiate` directly.
 *
 * @param {{ module_or_path: InitInput | Promise<InitInput> }} module_or_path - Passing `InitInput` directly is deprecated.
 *
 * @returns {Promise<InitOutput>}
 */
export default function __wbg_init (module_or_path?: { module_or_path: InitInput | Promise<InitInput> } | InitInput | Promise<InitInput>): Promise<InitOutput>;
