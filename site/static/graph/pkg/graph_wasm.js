/* @ts-self-types="./graph_wasm.d.ts" */

/**
 * ego 네트워크: 중심에서 `depth` 홉 이내 노드 id, BFS 순서(중심이 첫 원소).
 * 루트 허브·README 색인처럼 차수가 `EXPAND_DEGREE_CUTOFF` 를 넘는 노드는 포함하되 그 너머로 확장하지 않는다
 * (모든 섹션이 루트/README 로 이어져 depth 2 면 전체가 되기 때문).
 * @param {number} node_id
 * @param {number} depth
 * @returns {Uint32Array}
 */
export function ego(node_id, depth) {
    try {
        const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
        wasm.ego(retptr, node_id, depth);
        var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
        var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
        var v1 = getArrayU32FromWasm0(r0, r1).slice();
        wasm.__wbindgen_export(r0, r1 * 4, 4);
        return v1;
    } finally {
        wasm.__wbindgen_add_to_stack_pointer(16);
    }
}

/**
 * 각 노드까지의 홉 수. 도달 불가 = 255. `ego()` 와 같은 규칙.
 * @param {number} node_id
 * @param {number} depth
 * @returns {Uint8Array}
 */
export function ego_depths(node_id, depth) {
    try {
        const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
        wasm.ego_depths(retptr, node_id, depth);
        var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
        var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
        var v1 = getArrayU8FromWasm0(r0, r1).slice();
        wasm.__wbindgen_export(r0, r1 * 1, 1);
        return v1;
    } finally {
        wasm.__wbindgen_add_to_stack_pointer(16);
    }
}

/**
 * @param {number} x
 * @param {number} y
 * @returns {number}
 */
export function hit_test(x, y) {
    const ret = wasm.hit_test(x, y);
    return ret >>> 0;
}

/**
 * 화면 좌표(CSS px)에서 가장 가까운 노드. 없으면 `u32::MAX`.
 * `mask` 가 비어 있지 않으면 `mask[id] != 0` 인 노드만 후보(현재 뷰에 보이는 노드).
 * @param {number} x
 * @param {number} y
 * @param {Uint8Array} mask
 * @returns {number}
 */
export function hit_test_masked(x, y, mask) {
    const ptr0 = passArray8ToWasm0(mask, wasm.__wbindgen_export2);
    const len0 = WASM_VECTOR_LEN;
    const ret = wasm.hit_test_masked(x, y, ptr0, len0);
    return ret >>> 0;
}

/**
 * @param {number} zoom
 * @returns {Uint8Array}
 */
export function label_visible(zoom) {
    try {
        const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
        wasm.label_visible(retptr, zoom);
        var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
        var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
        var v1 = getArrayU8FromWasm0(r0, r1).slice();
        wasm.__wbindgen_export(r0, r1 * 1, 1);
        return v1;
    } finally {
        wasm.__wbindgen_add_to_stack_pointer(16);
    }
}

/**
 * 라벨 표시 여부(노드별 0/1).
 * - zoom ≥ 1.2: 화면 안 노드 전부 후보, 차수 내림차순으로 겹치는 라벨은 걸러낸다(그리디 AABB).
 * - zoom < 1.2: 차수 상위 `LABEL_TOP_DEGREE` 만 후보(역시 겹침 컬링).
 * `mask` 가 비어 있지 않으면 `mask[id] != 0` 인 노드만 후보.
 * @param {number} zoom
 * @param {Uint8Array} mask
 * @returns {Uint8Array}
 */
export function label_visible_masked(zoom, mask) {
    try {
        const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
        const ptr0 = passArray8ToWasm0(mask, wasm.__wbindgen_export2);
        const len0 = WASM_VECTOR_LEN;
        wasm.label_visible_masked(retptr, zoom, ptr0, len0);
        var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
        var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
        var v2 = getArrayU8FromWasm0(r0, r1).slice();
        wasm.__wbindgen_export(r0, r1 * 1, 1);
        return v2;
    } finally {
        wasm.__wbindgen_add_to_stack_pointer(16);
    }
}

/**
 * 그래프 적재. `titles` 는 노드 순서대로 `'\n'` 로 이어 붙인 한 문자열(허브 포함, 정확히 N개).
 * `choseong` 도 같은 형식(허브는 빈 문자열이어도 된다 — 제목에서 다시 만든다).
 * 형식이 CONTRACT.md 와 어긋나면 false 를 돌려주고 아무것도 적재하지 않는다.
 * @param {Float32Array} pos
 * @param {Uint32Array} csr
 * @param {string} titles
 * @param {string} choseong
 * @param {number} root
 * @returns {boolean}
 */
export function load(pos, csr, titles, choseong, root) {
    const ptr0 = passArrayF32ToWasm0(pos, wasm.__wbindgen_export2);
    const len0 = WASM_VECTOR_LEN;
    const ptr1 = passArray32ToWasm0(csr, wasm.__wbindgen_export2);
    const len1 = WASM_VECTOR_LEN;
    const ptr2 = passStringToWasm0(titles, wasm.__wbindgen_export2, wasm.__wbindgen_export3);
    const len2 = WASM_VECTOR_LEN;
    const ptr3 = passStringToWasm0(choseong, wasm.__wbindgen_export2, wasm.__wbindgen_export3);
    const len3 = WASM_VECTOR_LEN;
    const ret = wasm.load(ptr0, len0, ptr1, len1, ptr2, len2, ptr3, len3, root);
    return ret !== 0;
}

/**
 * 로컬 뷰: 섹션 허브 `hub_id` 의 노트들 + 그 노트들의 1홉 이웃(다른 섹션 노트/허브 포함), 루트 허브 제외.
 * @param {number} hub_id
 * @returns {Uint32Array}
 */
export function local_view(hub_id) {
    try {
        const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
        wasm.local_view(retptr, hub_id);
        var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
        var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
        var v1 = getArrayU32FromWasm0(r0, r1).slice();
        wasm.__wbindgen_export(r0, r1 * 4, 4);
        return v1;
    } finally {
        wasm.__wbindgen_add_to_stack_pointer(16);
    }
}

/**
 * 직접 이웃(CSR 순서).
 * @param {number} node_id
 * @returns {Uint32Array}
 */
export function neighbors(node_id) {
    try {
        const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
        wasm.neighbors(retptr, node_id);
        var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
        var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
        var v1 = getArrayU32FromWasm0(r0, r1).slice();
        wasm.__wbindgen_export(r0, r1 * 4, 4);
        return v1;
    } finally {
        wasm.__wbindgen_add_to_stack_pointer(16);
    }
}

/**
 * 노드 반지름(CSS px, zoom 1 기준): `min(12, 3 + 2*sqrt(degree))`
 * @returns {Float32Array}
 */
export function radii() {
    try {
        const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
        wasm.radii(retptr);
        var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
        var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
        var v1 = getArrayF32FromWasm0(r0, r1).slice();
        wasm.__wbindgen_export(r0, r1 * 4, 4);
        return v1;
    } finally {
        wasm.__wbindgen_add_to_stack_pointer(16);
    }
}

/**
 * 검색. 점수 내림차순 노드 id. 빈 질의 → 빈 배열.
 *
 * - 제목 부분 문자열(대소문자 무시, NFC)
 * - 초성 질의(`ㅈㄱㅎ`): 초성 문자열(공백 제거)에 대한 부분 일치
 * - 음절 질의를 초성으로 바꿔 초성열과 대조 (띄어쓰기 차이 흡수)
 * - 공백으로 나뉜 토큰이 모두 제목/초성에 있으면 약한 일치
 * @param {string} query
 * @returns {Uint32Array}
 */
export function search(query) {
    try {
        const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
        const ptr0 = passStringToWasm0(query, wasm.__wbindgen_export2, wasm.__wbindgen_export3);
        const len0 = WASM_VECTOR_LEN;
        wasm.search(retptr, ptr0, len0);
        var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
        var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
        var v2 = getArrayU32FromWasm0(r0, r1).slice();
        wasm.__wbindgen_export(r0, r1 * 4, 4);
        return v2;
    } finally {
        wasm.__wbindgen_add_to_stack_pointer(16);
    }
}

/**
 * 월드→화면 변환: `sx = x*scale + tx`, `sy = y*scale + ty` (CSS px). 히트테스트/라벨 컬링이 쓴다.
 * `node_scale` 은 화면 반지름 배율(JS 렌더러와 같은 값을 넘긴다).
 * @param {number} scale
 * @param {number} tx
 * @param {number} ty
 * @param {number} width
 * @param {number} height
 * @param {number} node_scale
 */
export function set_view(scale, tx, ty, width, height, node_scale) {
    wasm.set_view(scale, tx, ty, width, height, node_scale);
}

/**
 * `[nodes, csr_targets, undirected_edges]`
 * @returns {Uint32Array}
 */
export function stats() {
    try {
        const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
        wasm.stats(retptr);
        var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
        var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
        var v1 = getArrayU32FromWasm0(r0, r1).slice();
        wasm.__wbindgen_export(r0, r1 * 4, 4);
        return v1;
    } finally {
        wasm.__wbindgen_add_to_stack_pointer(16);
    }
}
function __wbg_get_imports() {
    const import0 = {
        __proto__: null,
    };
    return {
        __proto__: null,
        "./graph_wasm_bg.js": import0,
    };
}

function getArrayF32FromWasm0(ptr, len) {
    ptr = ptr >>> 0;
    return getFloat32ArrayMemory0().subarray(ptr / 4, ptr / 4 + len);
}

function getArrayU32FromWasm0(ptr, len) {
    ptr = ptr >>> 0;
    return getUint32ArrayMemory0().subarray(ptr / 4, ptr / 4 + len);
}

function getArrayU8FromWasm0(ptr, len) {
    ptr = ptr >>> 0;
    return getUint8ArrayMemory0().subarray(ptr / 1, ptr / 1 + len);
}

let cachedDataViewMemory0 = null;
function getDataViewMemory0() {
    if (cachedDataViewMemory0 === null || cachedDataViewMemory0.buffer.detached === true || (cachedDataViewMemory0.buffer.detached === undefined && cachedDataViewMemory0.buffer !== wasm.memory.buffer)) {
        cachedDataViewMemory0 = new DataView(wasm.memory.buffer);
    }
    return cachedDataViewMemory0;
}

let cachedFloat32ArrayMemory0 = null;
function getFloat32ArrayMemory0() {
    if (cachedFloat32ArrayMemory0 === null || cachedFloat32ArrayMemory0.byteLength === 0) {
        cachedFloat32ArrayMemory0 = new Float32Array(wasm.memory.buffer);
    }
    return cachedFloat32ArrayMemory0;
}

let cachedUint32ArrayMemory0 = null;
function getUint32ArrayMemory0() {
    if (cachedUint32ArrayMemory0 === null || cachedUint32ArrayMemory0.byteLength === 0) {
        cachedUint32ArrayMemory0 = new Uint32Array(wasm.memory.buffer);
    }
    return cachedUint32ArrayMemory0;
}

let cachedUint8ArrayMemory0 = null;
function getUint8ArrayMemory0() {
    if (cachedUint8ArrayMemory0 === null || cachedUint8ArrayMemory0.byteLength === 0) {
        cachedUint8ArrayMemory0 = new Uint8Array(wasm.memory.buffer);
    }
    return cachedUint8ArrayMemory0;
}

function passArray32ToWasm0(arg, malloc) {
    const ptr = malloc(arg.length * 4, 4) >>> 0;
    getUint32ArrayMemory0().set(arg, ptr / 4);
    WASM_VECTOR_LEN = arg.length;
    return ptr;
}

function passArray8ToWasm0(arg, malloc) {
    const ptr = malloc(arg.length * 1, 1) >>> 0;
    getUint8ArrayMemory0().set(arg, ptr / 1);
    WASM_VECTOR_LEN = arg.length;
    return ptr;
}

function passArrayF32ToWasm0(arg, malloc) {
    const ptr = malloc(arg.length * 4, 4) >>> 0;
    getFloat32ArrayMemory0().set(arg, ptr / 4);
    WASM_VECTOR_LEN = arg.length;
    return ptr;
}

function passStringToWasm0(arg, malloc, realloc) {
    if (realloc === undefined) {
        const buf = cachedTextEncoder.encode(arg);
        const ptr = malloc(buf.length, 1) >>> 0;
        getUint8ArrayMemory0().subarray(ptr, ptr + buf.length).set(buf);
        WASM_VECTOR_LEN = buf.length;
        return ptr;
    }

    let len = arg.length;
    let ptr = malloc(len, 1) >>> 0;

    const mem = getUint8ArrayMemory0();

    let offset = 0;

    for (; offset < len; offset++) {
        const code = arg.charCodeAt(offset);
        if (code > 0x7F) break;
        mem[ptr + offset] = code;
    }
    if (offset !== len) {
        if (offset !== 0) {
            arg = arg.slice(offset);
        }
        ptr = realloc(ptr, len, len = offset + arg.length * 3, 1) >>> 0;
        const view = getUint8ArrayMemory0().subarray(ptr + offset, ptr + len);
        const ret = cachedTextEncoder.encodeInto(arg, view);

        offset += ret.written;
        ptr = realloc(ptr, len, offset, 1) >>> 0;
    }

    WASM_VECTOR_LEN = offset;
    return ptr;
}

const cachedTextEncoder = new TextEncoder();

if (!('encodeInto' in cachedTextEncoder)) {
    cachedTextEncoder.encodeInto = function (arg, view) {
        const buf = cachedTextEncoder.encode(arg);
        view.set(buf);
        return {
            read: arg.length,
            written: buf.length
        };
    };
}

let WASM_VECTOR_LEN = 0;

let wasmModule, wasmInstance, wasm;
function __wbg_finalize_init(instance, module) {
    wasmInstance = instance;
    wasm = instance.exports;
    wasmModule = module;
    cachedDataViewMemory0 = null;
    cachedFloat32ArrayMemory0 = null;
    cachedUint32ArrayMemory0 = null;
    cachedUint8ArrayMemory0 = null;
    return wasm;
}

async function __wbg_load(module, imports) {
    if (typeof Response === 'function' && module instanceof Response) {
        if (!module.ok) {
            throw new Error(`failed to fetch Wasm: ${module.status} ${module.statusText} fetching '${module.url}'`);
        }

        if (typeof WebAssembly.instantiateStreaming === 'function') {
            try {
                return await WebAssembly.instantiateStreaming(module, imports);
            } catch (e) {
                const validResponse = expectedResponseType(module.type);

                if (validResponse && module.headers.get('Content-Type') !== 'application/wasm') {
                    console.warn("`WebAssembly.instantiateStreaming` failed because your server does not serve Wasm with `application/wasm` MIME type. Falling back to `WebAssembly.instantiate` which is slower. Original error:\n", e);

                } else { throw e; }
            }
        }

        const bytes = await module.arrayBuffer();
        return await WebAssembly.instantiate(bytes, imports);
    } else {
        const instance = await WebAssembly.instantiate(module, imports);

        if (instance instanceof WebAssembly.Instance) {
            return { instance, module };
        } else {
            return instance;
        }
    }

    function expectedResponseType(type) {
        switch (type) {
            case 'basic': case 'cors': case 'default': return true;
        }
        return false;
    }
}

function initSync(module) {
    if (wasm !== undefined) return wasm;


    if (module !== undefined) {
        if (Object.getPrototypeOf(module) === Object.prototype) {
            ({module} = module)
        } else {
            console.warn('using deprecated parameters for `initSync()`; pass a single object instead')
        }
    }

    const imports = __wbg_get_imports();
    if (!(module instanceof WebAssembly.Module)) {
        module = new WebAssembly.Module(module);
    }
    const instance = new WebAssembly.Instance(module, imports);
    return __wbg_finalize_init(instance, module);
}

async function __wbg_init(module_or_path) {
    if (wasm !== undefined) return wasm;


    if (module_or_path !== undefined) {
        if (Object.getPrototypeOf(module_or_path) === Object.prototype) {
            ({module_or_path} = module_or_path)
        } else {
            console.warn('using deprecated parameters for the initialization function; pass a single object instead')
        }
    }

    if (module_or_path === undefined) {
        module_or_path = new URL('graph_wasm_bg.wasm', import.meta.url);
    }
    const imports = __wbg_get_imports();

    if (typeof module_or_path === 'string' || (typeof Request === 'function' && module_or_path instanceof Request) || (typeof URL === 'function' && module_or_path instanceof URL)) {
        module_or_path = fetch(module_or_path);
    }

    const { instance, module } = await __wbg_load(await module_or_path, imports);

    return __wbg_finalize_init(instance, module);
}

export { initSync, __wbg_init as default };
