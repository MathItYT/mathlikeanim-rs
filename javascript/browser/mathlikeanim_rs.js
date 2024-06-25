let wasm;

const heap = new Array(128).fill(undefined);

heap.push(undefined, null, true, false);

function getObject(idx) { return heap[idx]; }

let heap_next = heap.length;

function dropObject(idx) {
    if (idx < 132) return;
    heap[idx] = heap_next;
    heap_next = idx;
}

function takeObject(idx) {
    const ret = getObject(idx);
    dropObject(idx);
    return ret;
}

function isLikeNone(x) {
    return x === undefined || x === null;
}

let cachedFloat64Memory0 = null;

function getFloat64Memory0() {
    if (cachedFloat64Memory0 === null || cachedFloat64Memory0.byteLength === 0) {
        cachedFloat64Memory0 = new Float64Array(wasm.memory.buffer);
    }
    return cachedFloat64Memory0;
}

let cachedInt32Memory0 = null;

function getInt32Memory0() {
    if (cachedInt32Memory0 === null || cachedInt32Memory0.byteLength === 0) {
        cachedInt32Memory0 = new Int32Array(wasm.memory.buffer);
    }
    return cachedInt32Memory0;
}

function addHeapObject(obj) {
    if (heap_next === heap.length) heap.push(heap.length + 1);
    const idx = heap_next;
    heap_next = heap[idx];

    heap[idx] = obj;
    return idx;
}

const cachedTextDecoder = (typeof TextDecoder !== 'undefined' ? new TextDecoder('utf-8', { ignoreBOM: true, fatal: true }) : { decode: () => { throw Error('TextDecoder not available') } } );

if (typeof TextDecoder !== 'undefined') { cachedTextDecoder.decode(); };

let cachedUint8Memory0 = null;

function getUint8Memory0() {
    if (cachedUint8Memory0 === null || cachedUint8Memory0.byteLength === 0) {
        cachedUint8Memory0 = new Uint8Array(wasm.memory.buffer);
    }
    return cachedUint8Memory0;
}

function getStringFromWasm0(ptr, len) {
    ptr = ptr >>> 0;
    return cachedTextDecoder.decode(getUint8Memory0().subarray(ptr, ptr + len));
}

let WASM_VECTOR_LEN = 0;

const cachedTextEncoder = (typeof TextEncoder !== 'undefined' ? new TextEncoder('utf-8') : { encode: () => { throw Error('TextEncoder not available') } } );

const encodeString = (typeof cachedTextEncoder.encodeInto === 'function'
    ? function (arg, view) {
    return cachedTextEncoder.encodeInto(arg, view);
}
    : function (arg, view) {
    const buf = cachedTextEncoder.encode(arg);
    view.set(buf);
    return {
        read: arg.length,
        written: buf.length
    };
});

function passStringToWasm0(arg, malloc, realloc) {

    if (realloc === undefined) {
        const buf = cachedTextEncoder.encode(arg);
        const ptr = malloc(buf.length, 1) >>> 0;
        getUint8Memory0().subarray(ptr, ptr + buf.length).set(buf);
        WASM_VECTOR_LEN = buf.length;
        return ptr;
    }

    let len = arg.length;
    let ptr = malloc(len, 1) >>> 0;

    const mem = getUint8Memory0();

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
        const view = getUint8Memory0().subarray(ptr + offset, ptr + len);
        const ret = encodeString(arg, view);

        offset += ret.written;
        ptr = realloc(ptr, len, offset, 1) >>> 0;
    }

    WASM_VECTOR_LEN = offset;
    return ptr;
}

function debugString(val) {
    // primitive types
    const type = typeof val;
    if (type == 'number' || type == 'boolean' || val == null) {
        return  `${val}`;
    }
    if (type == 'string') {
        return `"${val}"`;
    }
    if (type == 'symbol') {
        const description = val.description;
        if (description == null) {
            return 'Symbol';
        } else {
            return `Symbol(${description})`;
        }
    }
    if (type == 'function') {
        const name = val.name;
        if (typeof name == 'string' && name.length > 0) {
            return `Function(${name})`;
        } else {
            return 'Function';
        }
    }
    // objects
    if (Array.isArray(val)) {
        const length = val.length;
        let debug = '[';
        if (length > 0) {
            debug += debugString(val[0]);
        }
        for(let i = 1; i < length; i++) {
            debug += ', ' + debugString(val[i]);
        }
        debug += ']';
        return debug;
    }
    // Test for built-in
    const builtInMatches = /\[object ([^\]]+)\]/.exec(toString.call(val));
    let className;
    if (builtInMatches.length > 1) {
        className = builtInMatches[1];
    } else {
        // Failed to match the standard '[object ClassName]'
        return toString.call(val);
    }
    if (className == 'Object') {
        // we're a user defined class or Object
        // JSON.stringify avoids problems with cycles, and is generally much
        // easier than looping through ownProperties of `val`.
        try {
            return 'Object(' + JSON.stringify(val) + ')';
        } catch (_) {
            return 'Object';
        }
    }
    // errors
    if (val instanceof Error) {
        return `${val.name}: ${val.message}\n${val.stack}`;
    }
    // TODO we could test for more things here, like `Set`s and `Map`s.
    return className;
}

const CLOSURE_DTORS = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(state => {
    wasm.__wbindgen_export_2.get(state.dtor)(state.a, state.b)
});

function makeClosure(arg0, arg1, dtor, f) {
    const state = { a: arg0, b: arg1, cnt: 1, dtor };
    const real = (...args) => {
        // First up with a closure we increment the internal reference
        // count. This ensures that the Rust closure environment won't
        // be deallocated while we're invoking it.
        state.cnt++;
        try {
            return f(state.a, state.b, ...args);
        } finally {
            if (--state.cnt === 0) {
                wasm.__wbindgen_export_2.get(state.dtor)(state.a, state.b);
                state.a = 0;
                CLOSURE_DTORS.unregister(state);
            }
        }
    };
    real.original = state;
    CLOSURE_DTORS.register(real, state, state);
    return real;
}
function __wbg_adapter_34(arg0, arg1, arg2, arg3) {
    const ret = wasm._dyn_core__ops__function__Fn__A_B___Output___R_as_wasm_bindgen__closure__WasmClosure___describe__invoke__h6d64b0991b389f85(arg0, arg1, addHeapObject(arg2), arg3);
    return takeObject(ret);
}

function __wbg_adapter_37(arg0, arg1) {
    const ret = wasm._dyn_core__ops__function__Fn_____Output___R_as_wasm_bindgen__closure__WasmClosure___describe__invoke__ha1cf8fb80f0e40d3(arg0, arg1);
    return takeObject(ret);
}

function makeMutClosure(arg0, arg1, dtor, f) {
    const state = { a: arg0, b: arg1, cnt: 1, dtor };
    const real = (...args) => {
        // First up with a closure we increment the internal reference
        // count. This ensures that the Rust closure environment won't
        // be deallocated while we're invoking it.
        state.cnt++;
        const a = state.a;
        state.a = 0;
        try {
            return f(a, state.b, ...args);
        } finally {
            if (--state.cnt === 0) {
                wasm.__wbindgen_export_2.get(state.dtor)(a, state.b);
                CLOSURE_DTORS.unregister(state);
            } else {
                state.a = a;
            }
        }
    };
    real.original = state;
    CLOSURE_DTORS.register(real, state, state);
    return real;
}
function __wbg_adapter_40(arg0, arg1, arg2) {
    wasm._dyn_core__ops__function__FnMut__A____Output___R_as_wasm_bindgen__closure__WasmClosure___describe__invoke__h36f54c9e7475dd01(arg0, arg1, addHeapObject(arg2));
}

function _assertClass(instance, klass) {
    if (!(instance instanceof klass)) {
        throw new Error(`expected instance of ${klass.name}`);
    }
    return instance.ptr;
}

let cachedUint32Memory0 = null;

function getUint32Memory0() {
    if (cachedUint32Memory0 === null || cachedUint32Memory0.byteLength === 0) {
        cachedUint32Memory0 = new Uint32Array(wasm.memory.buffer);
    }
    return cachedUint32Memory0;
}

function passArrayJsValueToWasm0(array, malloc) {
    const ptr = malloc(array.length * 4, 4) >>> 0;
    const mem = getUint32Memory0();
    for (let i = 0; i < array.length; i++) {
        mem[ptr / 4 + i] = addHeapObject(array[i]);
    }
    WASM_VECTOR_LEN = array.length;
    return ptr;
}

function getArrayJsValueFromWasm0(ptr, len) {
    ptr = ptr >>> 0;
    const mem = getUint32Memory0();
    const slice = mem.subarray(ptr / 4, ptr / 4 + len);
    const result = [];
    for (let i = 0; i < slice.length; i++) {
        result.push(takeObject(slice[i]));
    }
    return result;
}
/**
* @param {WasmVectorObject} shape
* @param {number} tip_side_length
* @param {WasmColor} tip_color
* @returns {WasmVectorObject}
*/
export function addFinalTip(shape, tip_side_length, tip_color) {
    _assertClass(shape, WasmVectorObject);
    var ptr0 = shape.__destroy_into_raw();
    _assertClass(tip_color, WasmColor);
    var ptr1 = tip_color.__destroy_into_raw();
    const ret = wasm.addFinalTip(ptr0, tip_side_length, ptr1);
    return WasmVectorObject.__wrap(ret);
}

/**
* @param {WasmVectorObject} shape
* @param {number} tip_side_length
* @param {WasmColor} tip_color
* @returns {WasmVectorObject}
*/
export function addInitialTip(shape, tip_side_length, tip_color) {
    _assertClass(shape, WasmVectorObject);
    var ptr0 = shape.__destroy_into_raw();
    _assertClass(tip_color, WasmColor);
    var ptr1 = tip_color.__destroy_into_raw();
    const ret = wasm.addInitialTip(ptr0, tip_side_length, ptr1);
    return WasmVectorObject.__wrap(ret);
}

/**
* @param {WasmVectorObject} shape
* @param {number} tip_side_length
* @param {WasmColor} tip_color
* @returns {WasmVectorObject}
*/
export function addBothSidesTips(shape, tip_side_length, tip_color) {
    _assertClass(shape, WasmVectorObject);
    var ptr0 = shape.__destroy_into_raw();
    _assertClass(tip_color, WasmColor);
    var ptr1 = tip_color.__destroy_into_raw();
    const ret = wasm.addBothSidesTips(ptr0, tip_side_length, ptr1);
    return WasmVectorObject.__wrap(ret);
}

/**
* @param {Array<any>} center
* @param {number} radius
* @param {number} start_angle
* @param {number} end_angle
* @param {number | undefined} [num_points]
* @param {WasmColor | undefined} [stroke_color]
* @param {WasmColor | undefined} [fill_color]
* @param {number | undefined} [stroke_width]
* @param {string | undefined} [line_cap]
* @param {string | undefined} [line_join]
* @param {number | undefined} [index]
* @returns {WasmVectorObject}
*/
export function arc(center, radius, start_angle, end_angle, num_points, stroke_color, fill_color, stroke_width, line_cap, line_join, index) {
    let ptr0 = 0;
    if (!isLikeNone(stroke_color)) {
        _assertClass(stroke_color, WasmColor);
        ptr0 = stroke_color.__destroy_into_raw();
    }
    let ptr1 = 0;
    if (!isLikeNone(fill_color)) {
        _assertClass(fill_color, WasmColor);
        ptr1 = fill_color.__destroy_into_raw();
    }
    var ptr2 = isLikeNone(line_cap) ? 0 : passStringToWasm0(line_cap, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    var len2 = WASM_VECTOR_LEN;
    var ptr3 = isLikeNone(line_join) ? 0 : passStringToWasm0(line_join, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    var len3 = WASM_VECTOR_LEN;
    const ret = wasm.arc(addHeapObject(center), radius, start_angle, end_angle, !isLikeNone(num_points), isLikeNone(num_points) ? 0 : num_points, ptr0, ptr1, !isLikeNone(stroke_width), isLikeNone(stroke_width) ? 0 : stroke_width, ptr2, len2, ptr3, len3, !isLikeNone(index), isLikeNone(index) ? 0 : index);
    return WasmVectorObject.__wrap(ret);
}

/**
* @param {Array<any>} center
* @param {number} radius
* @param {number | undefined} [num_points]
* @param {WasmColor | undefined} [stroke_color]
* @param {WasmColor | undefined} [fill_color]
* @param {number | undefined} [stroke_width]
* @param {string | undefined} [line_cap]
* @param {string | undefined} [line_join]
* @param {number | undefined} [index]
* @returns {WasmVectorObject}
*/
export function circle(center, radius, num_points, stroke_color, fill_color, stroke_width, line_cap, line_join, index) {
    let ptr0 = 0;
    if (!isLikeNone(stroke_color)) {
        _assertClass(stroke_color, WasmColor);
        ptr0 = stroke_color.__destroy_into_raw();
    }
    let ptr1 = 0;
    if (!isLikeNone(fill_color)) {
        _assertClass(fill_color, WasmColor);
        ptr1 = fill_color.__destroy_into_raw();
    }
    var ptr2 = isLikeNone(line_cap) ? 0 : passStringToWasm0(line_cap, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    var len2 = WASM_VECTOR_LEN;
    var ptr3 = isLikeNone(line_join) ? 0 : passStringToWasm0(line_join, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    var len3 = WASM_VECTOR_LEN;
    const ret = wasm.circle(addHeapObject(center), radius, !isLikeNone(num_points), isLikeNone(num_points) ? 0 : num_points, ptr0, ptr1, !isLikeNone(stroke_width), isLikeNone(stroke_width) ? 0 : stroke_width, ptr2, len2, ptr3, len3, !isLikeNone(index), isLikeNone(index) ? 0 : index);
    return WasmVectorObject.__wrap(ret);
}

/**
* @param {Array<any>} center
* @param {number} x_radius
* @param {number} y_radius
* @param {number} start_angle
* @param {number} end_angle
* @param {number | undefined} [num_points]
* @param {WasmColor | undefined} [stroke_color]
* @param {WasmColor | undefined} [fill_color]
* @param {number | undefined} [stroke_width]
* @param {string | undefined} [line_cap]
* @param {string | undefined} [line_join]
* @param {number | undefined} [index]
* @returns {WasmVectorObject}
*/
export function ellipticalArc(center, x_radius, y_radius, start_angle, end_angle, num_points, stroke_color, fill_color, stroke_width, line_cap, line_join, index) {
    let ptr0 = 0;
    if (!isLikeNone(stroke_color)) {
        _assertClass(stroke_color, WasmColor);
        ptr0 = stroke_color.__destroy_into_raw();
    }
    let ptr1 = 0;
    if (!isLikeNone(fill_color)) {
        _assertClass(fill_color, WasmColor);
        ptr1 = fill_color.__destroy_into_raw();
    }
    var ptr2 = isLikeNone(line_cap) ? 0 : passStringToWasm0(line_cap, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    var len2 = WASM_VECTOR_LEN;
    var ptr3 = isLikeNone(line_join) ? 0 : passStringToWasm0(line_join, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    var len3 = WASM_VECTOR_LEN;
    const ret = wasm.ellipticalArc(addHeapObject(center), x_radius, y_radius, start_angle, end_angle, !isLikeNone(num_points), isLikeNone(num_points) ? 0 : num_points, ptr0, ptr1, !isLikeNone(stroke_width), isLikeNone(stroke_width) ? 0 : stroke_width, ptr2, len2, ptr3, len3, !isLikeNone(index), isLikeNone(index) ? 0 : index);
    return WasmVectorObject.__wrap(ret);
}

/**
* @param {Array<any>} center
* @param {number} x_radius
* @param {number} y_radius
* @param {number | undefined} [num_points]
* @param {WasmColor | undefined} [stroke_color]
* @param {WasmColor | undefined} [fill_color]
* @param {number | undefined} [stroke_width]
* @param {string | undefined} [line_cap]
* @param {string | undefined} [line_join]
* @param {number | undefined} [index]
* @returns {WasmVectorObject}
*/
export function ellipse(center, x_radius, y_radius, num_points, stroke_color, fill_color, stroke_width, line_cap, line_join, index) {
    let ptr0 = 0;
    if (!isLikeNone(stroke_color)) {
        _assertClass(stroke_color, WasmColor);
        ptr0 = stroke_color.__destroy_into_raw();
    }
    let ptr1 = 0;
    if (!isLikeNone(fill_color)) {
        _assertClass(fill_color, WasmColor);
        ptr1 = fill_color.__destroy_into_raw();
    }
    var ptr2 = isLikeNone(line_cap) ? 0 : passStringToWasm0(line_cap, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    var len2 = WASM_VECTOR_LEN;
    var ptr3 = isLikeNone(line_join) ? 0 : passStringToWasm0(line_join, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    var len3 = WASM_VECTOR_LEN;
    const ret = wasm.ellipse(addHeapObject(center), x_radius, y_radius, !isLikeNone(num_points), isLikeNone(num_points) ? 0 : num_points, ptr0, ptr1, !isLikeNone(stroke_width), isLikeNone(stroke_width) ? 0 : stroke_width, ptr2, len2, ptr3, len3, !isLikeNone(index), isLikeNone(index) ? 0 : index);
    return WasmVectorObject.__wrap(ret);
}

/**
* @param {Array<any>} center
* @param {number} inner_radius
* @param {number} outer_radius
* @param {number} start_angle
* @param {number} end_angle
* @param {number | undefined} [num_points]
* @param {WasmColor | undefined} [stroke_color]
* @param {WasmColor | undefined} [fill_color]
* @param {number | undefined} [stroke_width]
* @param {string | undefined} [line_cap]
* @param {string | undefined} [line_join]
* @param {number | undefined} [index]
* @returns {WasmVectorObject}
*/
export function annularSector(center, inner_radius, outer_radius, start_angle, end_angle, num_points, stroke_color, fill_color, stroke_width, line_cap, line_join, index) {
    let ptr0 = 0;
    if (!isLikeNone(stroke_color)) {
        _assertClass(stroke_color, WasmColor);
        ptr0 = stroke_color.__destroy_into_raw();
    }
    let ptr1 = 0;
    if (!isLikeNone(fill_color)) {
        _assertClass(fill_color, WasmColor);
        ptr1 = fill_color.__destroy_into_raw();
    }
    var ptr2 = isLikeNone(line_cap) ? 0 : passStringToWasm0(line_cap, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    var len2 = WASM_VECTOR_LEN;
    var ptr3 = isLikeNone(line_join) ? 0 : passStringToWasm0(line_join, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    var len3 = WASM_VECTOR_LEN;
    const ret = wasm.annularSector(addHeapObject(center), inner_radius, outer_radius, start_angle, end_angle, !isLikeNone(num_points), isLikeNone(num_points) ? 0 : num_points, ptr0, ptr1, !isLikeNone(stroke_width), isLikeNone(stroke_width) ? 0 : stroke_width, ptr2, len2, ptr3, len3, !isLikeNone(index), isLikeNone(index) ? 0 : index);
    return WasmVectorObject.__wrap(ret);
}

/**
* @param {Array<any>} start_point
* @param {Array<any>} end_point
* @param {WasmColor | undefined} [stroke_color]
* @param {number | undefined} [stroke_width]
* @param {string | undefined} [line_cap]
* @param {string | undefined} [line_join]
* @param {number | undefined} [index]
* @returns {WasmVectorObject}
*/
export function line(start_point, end_point, stroke_color, stroke_width, line_cap, line_join, index) {
    let ptr0 = 0;
    if (!isLikeNone(stroke_color)) {
        _assertClass(stroke_color, WasmColor);
        ptr0 = stroke_color.__destroy_into_raw();
    }
    var ptr1 = isLikeNone(line_cap) ? 0 : passStringToWasm0(line_cap, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    var len1 = WASM_VECTOR_LEN;
    var ptr2 = isLikeNone(line_join) ? 0 : passStringToWasm0(line_join, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    var len2 = WASM_VECTOR_LEN;
    const ret = wasm.line(addHeapObject(start_point), addHeapObject(end_point), ptr0, !isLikeNone(stroke_width), isLikeNone(stroke_width) ? 0 : stroke_width, ptr1, len1, ptr2, len2, !isLikeNone(index), isLikeNone(index) ? 0 : index);
    return WasmVectorObject.__wrap(ret);
}

/**
* @param {Array<any>} points
* @param {WasmColor | undefined} [stroke_color]
* @param {WasmColor | undefined} [fill_color]
* @param {number | undefined} [stroke_width]
* @param {string | undefined} [line_cap]
* @param {string | undefined} [line_join]
* @param {number | undefined} [index]
* @returns {WasmVectorObject}
*/
export function polygon(points, stroke_color, fill_color, stroke_width, line_cap, line_join, index) {
    let ptr0 = 0;
    if (!isLikeNone(stroke_color)) {
        _assertClass(stroke_color, WasmColor);
        ptr0 = stroke_color.__destroy_into_raw();
    }
    let ptr1 = 0;
    if (!isLikeNone(fill_color)) {
        _assertClass(fill_color, WasmColor);
        ptr1 = fill_color.__destroy_into_raw();
    }
    var ptr2 = isLikeNone(line_cap) ? 0 : passStringToWasm0(line_cap, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    var len2 = WASM_VECTOR_LEN;
    var ptr3 = isLikeNone(line_join) ? 0 : passStringToWasm0(line_join, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    var len3 = WASM_VECTOR_LEN;
    const ret = wasm.polygon(addHeapObject(points), ptr0, ptr1, !isLikeNone(stroke_width), isLikeNone(stroke_width) ? 0 : stroke_width, ptr2, len2, ptr3, len3, !isLikeNone(index), isLikeNone(index) ? 0 : index);
    return WasmVectorObject.__wrap(ret);
}

/**
* @param {Array<any>} center
* @param {number} side_length
* @param {number} num_sides
* @param {WasmColor | undefined} [stroke_color]
* @param {WasmColor | undefined} [fill_color]
* @param {number | undefined} [stroke_width]
* @param {string | undefined} [line_cap]
* @param {string | undefined} [line_join]
* @param {number | undefined} [index]
* @returns {WasmVectorObject}
*/
export function regularPolygon(center, side_length, num_sides, stroke_color, fill_color, stroke_width, line_cap, line_join, index) {
    let ptr0 = 0;
    if (!isLikeNone(stroke_color)) {
        _assertClass(stroke_color, WasmColor);
        ptr0 = stroke_color.__destroy_into_raw();
    }
    let ptr1 = 0;
    if (!isLikeNone(fill_color)) {
        _assertClass(fill_color, WasmColor);
        ptr1 = fill_color.__destroy_into_raw();
    }
    var ptr2 = isLikeNone(line_cap) ? 0 : passStringToWasm0(line_cap, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    var len2 = WASM_VECTOR_LEN;
    var ptr3 = isLikeNone(line_join) ? 0 : passStringToWasm0(line_join, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    var len3 = WASM_VECTOR_LEN;
    const ret = wasm.regularPolygon(addHeapObject(center), side_length, num_sides, ptr0, ptr1, !isLikeNone(stroke_width), isLikeNone(stroke_width) ? 0 : stroke_width, ptr2, len2, ptr3, len3, !isLikeNone(index), isLikeNone(index) ? 0 : index);
    return WasmVectorObject.__wrap(ret);
}

/**
* @param {Array<any>} center
* @param {number} side_length
* @param {WasmColor | undefined} [stroke_color]
* @param {WasmColor | undefined} [fill_color]
* @param {number | undefined} [stroke_width]
* @param {string | undefined} [line_cap]
* @param {string | undefined} [line_join]
* @param {number | undefined} [index]
* @returns {WasmVectorObject}
*/
export function square(center, side_length, stroke_color, fill_color, stroke_width, line_cap, line_join, index) {
    let ptr0 = 0;
    if (!isLikeNone(stroke_color)) {
        _assertClass(stroke_color, WasmColor);
        ptr0 = stroke_color.__destroy_into_raw();
    }
    let ptr1 = 0;
    if (!isLikeNone(fill_color)) {
        _assertClass(fill_color, WasmColor);
        ptr1 = fill_color.__destroy_into_raw();
    }
    var ptr2 = isLikeNone(line_cap) ? 0 : passStringToWasm0(line_cap, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    var len2 = WASM_VECTOR_LEN;
    var ptr3 = isLikeNone(line_join) ? 0 : passStringToWasm0(line_join, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    var len3 = WASM_VECTOR_LEN;
    const ret = wasm.square(addHeapObject(center), side_length, ptr0, ptr1, !isLikeNone(stroke_width), isLikeNone(stroke_width) ? 0 : stroke_width, ptr2, len2, ptr3, len3, !isLikeNone(index), isLikeNone(index) ? 0 : index);
    return WasmVectorObject.__wrap(ret);
}

/**
* @param {Array<any>} center
* @param {number} width
* @param {number} height
* @param {WasmColor | undefined} [stroke_color]
* @param {WasmColor | undefined} [fill_color]
* @param {number | undefined} [stroke_width]
* @param {string | undefined} [line_cap]
* @param {string | undefined} [line_join]
* @param {number | undefined} [index]
* @returns {WasmVectorObject}
*/
export function rectangle(center, width, height, stroke_color, fill_color, stroke_width, line_cap, line_join, index) {
    let ptr0 = 0;
    if (!isLikeNone(stroke_color)) {
        _assertClass(stroke_color, WasmColor);
        ptr0 = stroke_color.__destroy_into_raw();
    }
    let ptr1 = 0;
    if (!isLikeNone(fill_color)) {
        _assertClass(fill_color, WasmColor);
        ptr1 = fill_color.__destroy_into_raw();
    }
    var ptr2 = isLikeNone(line_cap) ? 0 : passStringToWasm0(line_cap, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    var len2 = WASM_VECTOR_LEN;
    var ptr3 = isLikeNone(line_join) ? 0 : passStringToWasm0(line_join, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    var len3 = WASM_VECTOR_LEN;
    const ret = wasm.rectangle(addHeapObject(center), width, height, ptr0, ptr1, !isLikeNone(stroke_width), isLikeNone(stroke_width) ? 0 : stroke_width, ptr2, len2, ptr3, len3, !isLikeNone(index), isLikeNone(index) ? 0 : index);
    return WasmVectorObject.__wrap(ret);
}

/**
* @param {Array<any>} center
* @param {number} side_length
* @param {WasmColor | undefined} [stroke_color]
* @param {WasmColor | undefined} [fill_color]
* @param {number | undefined} [stroke_width]
* @param {string | undefined} [line_cap]
* @param {string | undefined} [line_join]
* @param {number | undefined} [index]
* @returns {WasmVectorObject}
*/
export function equilateralTriangle(center, side_length, stroke_color, fill_color, stroke_width, line_cap, line_join, index) {
    let ptr0 = 0;
    if (!isLikeNone(stroke_color)) {
        _assertClass(stroke_color, WasmColor);
        ptr0 = stroke_color.__destroy_into_raw();
    }
    let ptr1 = 0;
    if (!isLikeNone(fill_color)) {
        _assertClass(fill_color, WasmColor);
        ptr1 = fill_color.__destroy_into_raw();
    }
    var ptr2 = isLikeNone(line_cap) ? 0 : passStringToWasm0(line_cap, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    var len2 = WASM_VECTOR_LEN;
    var ptr3 = isLikeNone(line_join) ? 0 : passStringToWasm0(line_join, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    var len3 = WASM_VECTOR_LEN;
    const ret = wasm.equilateralTriangle(addHeapObject(center), side_length, ptr0, ptr1, !isLikeNone(stroke_width), isLikeNone(stroke_width) ? 0 : stroke_width, ptr2, len2, ptr3, len3, !isLikeNone(index), isLikeNone(index) ? 0 : index);
    return WasmVectorObject.__wrap(ret);
}

/**
* @param {Array<any>} point1
* @param {Array<any>} point2
* @param {Array<any>} point3
* @param {WasmColor | undefined} [stroke_color]
* @param {WasmColor | undefined} [fill_color]
* @param {number | undefined} [stroke_width]
* @param {string | undefined} [line_cap]
* @param {string | undefined} [line_join]
* @param {number | undefined} [index]
* @returns {WasmVectorObject}
*/
export function triangle(point1, point2, point3, stroke_color, fill_color, stroke_width, line_cap, line_join, index) {
    let ptr0 = 0;
    if (!isLikeNone(stroke_color)) {
        _assertClass(stroke_color, WasmColor);
        ptr0 = stroke_color.__destroy_into_raw();
    }
    let ptr1 = 0;
    if (!isLikeNone(fill_color)) {
        _assertClass(fill_color, WasmColor);
        ptr1 = fill_color.__destroy_into_raw();
    }
    var ptr2 = isLikeNone(line_cap) ? 0 : passStringToWasm0(line_cap, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    var len2 = WASM_VECTOR_LEN;
    var ptr3 = isLikeNone(line_join) ? 0 : passStringToWasm0(line_join, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    var len3 = WASM_VECTOR_LEN;
    const ret = wasm.triangle(addHeapObject(point1), addHeapObject(point2), addHeapObject(point3), ptr0, ptr1, !isLikeNone(stroke_width), isLikeNone(stroke_width) ? 0 : stroke_width, ptr2, len2, ptr3, len3, !isLikeNone(index), isLikeNone(index) ? 0 : index);
    return WasmVectorObject.__wrap(ret);
}

/**
* @param {Array<any>} point1
* @param {Array<any>} point2
* @param {WasmColor | undefined} [stroke_color]
* @param {WasmColor | undefined} [fill_color]
* @param {number | undefined} [stroke_width]
* @param {string | undefined} [line_cap]
* @param {string | undefined} [line_join]
* @param {number | undefined} [index]
* @returns {WasmVectorObject}
*/
export function rightTriangle(point1, point2, stroke_color, fill_color, stroke_width, line_cap, line_join, index) {
    let ptr0 = 0;
    if (!isLikeNone(stroke_color)) {
        _assertClass(stroke_color, WasmColor);
        ptr0 = stroke_color.__destroy_into_raw();
    }
    let ptr1 = 0;
    if (!isLikeNone(fill_color)) {
        _assertClass(fill_color, WasmColor);
        ptr1 = fill_color.__destroy_into_raw();
    }
    var ptr2 = isLikeNone(line_cap) ? 0 : passStringToWasm0(line_cap, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    var len2 = WASM_VECTOR_LEN;
    var ptr3 = isLikeNone(line_join) ? 0 : passStringToWasm0(line_join, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    var len3 = WASM_VECTOR_LEN;
    const ret = wasm.rightTriangle(addHeapObject(point1), addHeapObject(point2), ptr0, ptr1, !isLikeNone(stroke_width), isLikeNone(stroke_width) ? 0 : stroke_width, ptr2, len2, ptr3, len3, !isLikeNone(index), isLikeNone(index) ? 0 : index);
    return WasmVectorObject.__wrap(ret);
}

/**
* @param {number} x_min
* @param {number} x_max
* @param {number} x_step
* @param {number} y_min
* @param {number} y_max
* @param {number} y_step
* @param {Array<any>} center
* @param {number | undefined} [x_length]
* @param {number | undefined} [y_length]
* @param {WasmColor | undefined} [color]
* @param {number | undefined} [stroke_width]
* @param {string | undefined} [line_cap]
* @param {string | undefined} [line_join]
* @param {number | undefined} [index]
* @param {boolean | undefined} [add_x_ticks]
* @param {boolean | undefined} [add_y_ticks]
* @param {number | undefined} [x_tick_size]
* @param {number | undefined} [y_tick_size]
* @param {boolean | undefined} [add_x_tip]
* @param {boolean | undefined} [add_y_tip]
* @returns {WasmVectorObject}
*/
export function axes(x_min, x_max, x_step, y_min, y_max, y_step, center, x_length, y_length, color, stroke_width, line_cap, line_join, index, add_x_ticks, add_y_ticks, x_tick_size, y_tick_size, add_x_tip, add_y_tip) {
    let ptr0 = 0;
    if (!isLikeNone(color)) {
        _assertClass(color, WasmColor);
        ptr0 = color.__destroy_into_raw();
    }
    var ptr1 = isLikeNone(line_cap) ? 0 : passStringToWasm0(line_cap, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    var len1 = WASM_VECTOR_LEN;
    var ptr2 = isLikeNone(line_join) ? 0 : passStringToWasm0(line_join, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    var len2 = WASM_VECTOR_LEN;
    const ret = wasm.axes(x_min, x_max, x_step, y_min, y_max, y_step, addHeapObject(center), !isLikeNone(x_length), isLikeNone(x_length) ? 0 : x_length, !isLikeNone(y_length), isLikeNone(y_length) ? 0 : y_length, ptr0, !isLikeNone(stroke_width), isLikeNone(stroke_width) ? 0 : stroke_width, ptr1, len1, ptr2, len2, !isLikeNone(index), isLikeNone(index) ? 0 : index, isLikeNone(add_x_ticks) ? 0xFFFFFF : add_x_ticks ? 1 : 0, isLikeNone(add_y_ticks) ? 0xFFFFFF : add_y_ticks ? 1 : 0, !isLikeNone(x_tick_size), isLikeNone(x_tick_size) ? 0 : x_tick_size, !isLikeNone(y_tick_size), isLikeNone(y_tick_size) ? 0 : y_tick_size, isLikeNone(add_x_tip) ? 0xFFFFFF : add_x_tip ? 1 : 0, isLikeNone(add_y_tip) ? 0xFFFFFF : add_y_tip ? 1 : 0);
    return WasmVectorObject.__wrap(ret);
}

/**
* @param {WasmVectorObject} axes
* @param {number} x
* @param {number} y
* @param {number} x_min
* @param {number} x_max
* @param {number} y_min
* @param {number} y_max
* @returns {Array<any>}
*/
export function coordsToPoint(axes, x, y, x_min, x_max, y_min, y_max) {
    _assertClass(axes, WasmVectorObject);
    const ret = wasm.coordsToPoint(axes.__wbg_ptr, x, y, x_min, x_max, y_min, y_max);
    return takeObject(ret);
}

/**
* @param {WasmVectorObject} axes
* @param {Array<any>} point
* @param {number} x_min
* @param {number} x_max
* @param {number} y_min
* @param {number} y_max
* @returns {Array<any>}
*/
export function pointToCoords(axes, point, x_min, x_max, y_min, y_max) {
    _assertClass(axes, WasmVectorObject);
    const ret = wasm.pointToCoords(axes.__wbg_ptr, addHeapObject(point), x_min, x_max, y_min, y_max);
    return takeObject(ret);
}

let stack_pointer = 128;

function addBorrowedObject(obj) {
    if (stack_pointer == 1) throw new Error('out of js stack');
    heap[--stack_pointer] = obj;
    return stack_pointer;
}
/**
* @param {Function} f
* @param {number} t_min
* @param {number} t_max
* @param {number} t_step
* @param {WasmVectorObject} axes
* @param {number} x_min
* @param {number} x_max
* @param {number} y_min
* @param {number} y_max
* @param {WasmColor | undefined} [color]
* @param {number | undefined} [stroke_width]
* @param {string | undefined} [line_cap]
* @param {string | undefined} [line_join]
* @param {number | undefined} [index]
* @returns {WasmVectorObject}
*/
export function parametricPlotInAxes(f, t_min, t_max, t_step, axes, x_min, x_max, y_min, y_max, color, stroke_width, line_cap, line_join, index) {
    try {
        _assertClass(axes, WasmVectorObject);
        let ptr0 = 0;
        if (!isLikeNone(color)) {
            _assertClass(color, WasmColor);
            ptr0 = color.__destroy_into_raw();
        }
        var ptr1 = isLikeNone(line_cap) ? 0 : passStringToWasm0(line_cap, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        var len1 = WASM_VECTOR_LEN;
        var ptr2 = isLikeNone(line_join) ? 0 : passStringToWasm0(line_join, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        var len2 = WASM_VECTOR_LEN;
        const ret = wasm.parametricPlotInAxes(addBorrowedObject(f), t_min, t_max, t_step, axes.__wbg_ptr, x_min, x_max, y_min, y_max, ptr0, !isLikeNone(stroke_width), isLikeNone(stroke_width) ? 0 : stroke_width, ptr1, len1, ptr2, len2, !isLikeNone(index), isLikeNone(index) ? 0 : index);
        return WasmVectorObject.__wrap(ret);
    } finally {
        heap[stack_pointer++] = undefined;
    }
}

/**
* @param {Function} f
* @param {number} x_min
* @param {number} x_max
* @param {number} y_min
* @param {number} y_max
* @param {number} x1
* @param {number} x2
* @param {number} x_step
* @param {WasmVectorObject} axes
* @param {WasmColor | undefined} [color]
* @param {number | undefined} [stroke_width]
* @param {string | undefined} [line_cap]
* @param {string | undefined} [line_join]
* @param {number | undefined} [index]
* @returns {WasmVectorObject}
*/
export function plotInAxes(f, x_min, x_max, y_min, y_max, x1, x2, x_step, axes, color, stroke_width, line_cap, line_join, index) {
    try {
        _assertClass(axes, WasmVectorObject);
        let ptr0 = 0;
        if (!isLikeNone(color)) {
            _assertClass(color, WasmColor);
            ptr0 = color.__destroy_into_raw();
        }
        var ptr1 = isLikeNone(line_cap) ? 0 : passStringToWasm0(line_cap, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        var len1 = WASM_VECTOR_LEN;
        var ptr2 = isLikeNone(line_join) ? 0 : passStringToWasm0(line_join, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        var len2 = WASM_VECTOR_LEN;
        const ret = wasm.plotInAxes(addBorrowedObject(f), x_min, x_max, y_min, y_max, x1, x2, x_step, axes.__wbg_ptr, ptr0, !isLikeNone(stroke_width), isLikeNone(stroke_width) ? 0 : stroke_width, ptr1, len1, ptr2, len2, !isLikeNone(index), isLikeNone(index) ? 0 : index);
        return WasmVectorObject.__wrap(ret);
    } finally {
        heap[stack_pointer++] = undefined;
    }
}

function passArrayF64ToWasm0(arg, malloc) {
    const ptr = malloc(arg.length * 8, 8) >>> 0;
    getFloat64Memory0().set(arg, ptr / 8);
    WASM_VECTOR_LEN = arg.length;
    return ptr;
}
/**
* @param {Function} f
* @param {number} x_min
* @param {number} x_max
* @param {number} y_min
* @param {number} y_max
* @param {number} x_1
* @param {number} x_2
* @param {number} x_step
* @param {number} y_1
* @param {number} y_2
* @param {number} y_step
* @param {WasmVectorObject} axes
* @param {WasmColor | undefined} color
* @param {number | undefined} stroke_width
* @param {string | undefined} line_cap
* @param {string | undefined} line_join
* @param {number | undefined} index
* @param {Float64Array} intervals
* @returns {WasmVectorObject}
*/
export function contourPlotInAxes(f, x_min, x_max, y_min, y_max, x_1, x_2, x_step, y_1, y_2, y_step, axes, color, stroke_width, line_cap, line_join, index, intervals) {
    try {
        _assertClass(axes, WasmVectorObject);
        let ptr0 = 0;
        if (!isLikeNone(color)) {
            _assertClass(color, WasmColor);
            ptr0 = color.__destroy_into_raw();
        }
        var ptr1 = isLikeNone(line_cap) ? 0 : passStringToWasm0(line_cap, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        var len1 = WASM_VECTOR_LEN;
        var ptr2 = isLikeNone(line_join) ? 0 : passStringToWasm0(line_join, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        var len2 = WASM_VECTOR_LEN;
        const ptr3 = passArrayF64ToWasm0(intervals, wasm.__wbindgen_malloc);
        const len3 = WASM_VECTOR_LEN;
        const ret = wasm.contourPlotInAxes(addBorrowedObject(f), x_min, x_max, y_min, y_max, x_1, x_2, x_step, y_1, y_2, y_step, axes.__wbg_ptr, ptr0, !isLikeNone(stroke_width), isLikeNone(stroke_width) ? 0 : stroke_width, ptr1, len1, ptr2, len2, !isLikeNone(index), isLikeNone(index) ? 0 : index, ptr3, len3);
        return WasmVectorObject.__wrap(ret);
    } finally {
        heap[stack_pointer++] = undefined;
    }
}

/**
* @param {WasmVectorObject} axes
* @param {WasmVectorObject} plot
* @param {number} x_min
* @param {number} x_max
* @param {number} y_min
* @param {number} y_max
* @param {number} x1
* @param {number} x2
* @param {WasmColor | undefined} [color]
* @param {number | undefined} [index]
* @returns {WasmVectorObject}
*/
export function areaUnderCurve(axes, plot, x_min, x_max, y_min, y_max, x1, x2, color, index) {
    _assertClass(axes, WasmVectorObject);
    _assertClass(plot, WasmVectorObject);
    let ptr0 = 0;
    if (!isLikeNone(color)) {
        _assertClass(color, WasmColor);
        ptr0 = color.__destroy_into_raw();
    }
    const ret = wasm.areaUnderCurve(axes.__wbg_ptr, plot.__wbg_ptr, x_min, x_max, y_min, y_max, x1, x2, ptr0, !isLikeNone(index), isLikeNone(index) ? 0 : index);
    return WasmVectorObject.__wrap(ret);
}

/**
* @param {Function} f
* @param {number} x_min
* @param {number} x_max
* @param {number} y_min
* @param {number} y_max
* @param {number} direction
* @param {number} x_1
* @param {number} x_2
* @param {number} n_rects
* @param {WasmVectorObject} axes
* @param {WasmColor | undefined} [stroke_color]
* @param {WasmColor | undefined} [fill_color]
* @param {number | undefined} [stroke_width]
* @param {string | undefined} [line_cap]
* @param {string | undefined} [line_join]
* @param {number | undefined} [index]
* @returns {WasmVectorObject}
*/
export function riemannRectanglesForPlot(f, x_min, x_max, y_min, y_max, direction, x_1, x_2, n_rects, axes, stroke_color, fill_color, stroke_width, line_cap, line_join, index) {
    try {
        _assertClass(axes, WasmVectorObject);
        let ptr0 = 0;
        if (!isLikeNone(stroke_color)) {
            _assertClass(stroke_color, WasmColor);
            ptr0 = stroke_color.__destroy_into_raw();
        }
        let ptr1 = 0;
        if (!isLikeNone(fill_color)) {
            _assertClass(fill_color, WasmColor);
            ptr1 = fill_color.__destroy_into_raw();
        }
        var ptr2 = isLikeNone(line_cap) ? 0 : passStringToWasm0(line_cap, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        var len2 = WASM_VECTOR_LEN;
        var ptr3 = isLikeNone(line_join) ? 0 : passStringToWasm0(line_join, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        var len3 = WASM_VECTOR_LEN;
        const ret = wasm.riemannRectanglesForPlot(addBorrowedObject(f), x_min, x_max, y_min, y_max, direction, x_1, x_2, n_rects, axes.__wbg_ptr, ptr0, ptr1, !isLikeNone(stroke_width), isLikeNone(stroke_width) ? 0 : stroke_width, ptr2, len2, ptr3, len3, !isLikeNone(index), isLikeNone(index) ? 0 : index);
        return WasmVectorObject.__wrap(ret);
    } finally {
        heap[stack_pointer++] = undefined;
    }
}

/**
* @param {Function} f
* @param {number} x_1
* @param {number} x_2
* @param {number} length
* @param {WasmVectorObject} axes
* @param {number} x_min
* @param {number} x_max
* @param {number} y_min
* @param {number} y_max
* @param {WasmColor | undefined} [color]
* @param {number | undefined} [stroke_width]
* @param {string | undefined} [line_cap]
* @param {string | undefined} [line_join]
* @param {number | undefined} [index]
* @returns {WasmVectorObject}
*/
export function secantLineForPlot(f, x_1, x_2, length, axes, x_min, x_max, y_min, y_max, color, stroke_width, line_cap, line_join, index) {
    try {
        _assertClass(axes, WasmVectorObject);
        let ptr0 = 0;
        if (!isLikeNone(color)) {
            _assertClass(color, WasmColor);
            ptr0 = color.__destroy_into_raw();
        }
        var ptr1 = isLikeNone(line_cap) ? 0 : passStringToWasm0(line_cap, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        var len1 = WASM_VECTOR_LEN;
        var ptr2 = isLikeNone(line_join) ? 0 : passStringToWasm0(line_join, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        var len2 = WASM_VECTOR_LEN;
        const ret = wasm.secantLineForPlot(addBorrowedObject(f), x_1, x_2, length, axes.__wbg_ptr, x_min, x_max, y_min, y_max, ptr0, !isLikeNone(stroke_width), isLikeNone(stroke_width) ? 0 : stroke_width, ptr1, len1, ptr2, len2, !isLikeNone(index), isLikeNone(index) ? 0 : index);
        return WasmVectorObject.__wrap(ret);
    } finally {
        heap[stack_pointer++] = undefined;
    }
}

/**
* @param {Function} f
* @param {number} t_min
* @param {number} t_max
* @param {number} t_step
* @param {WasmColor | undefined} [color]
* @param {number | undefined} [stroke_width]
* @param {string | undefined} [line_cap]
* @param {string | undefined} [line_join]
* @param {number | undefined} [index]
* @returns {WasmVectorObject}
*/
export function parametricFunction(f, t_min, t_max, t_step, color, stroke_width, line_cap, line_join, index) {
    try {
        let ptr0 = 0;
        if (!isLikeNone(color)) {
            _assertClass(color, WasmColor);
            ptr0 = color.__destroy_into_raw();
        }
        var ptr1 = isLikeNone(line_cap) ? 0 : passStringToWasm0(line_cap, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        var len1 = WASM_VECTOR_LEN;
        var ptr2 = isLikeNone(line_join) ? 0 : passStringToWasm0(line_join, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        var len2 = WASM_VECTOR_LEN;
        const ret = wasm.parametricFunction(addBorrowedObject(f), t_min, t_max, t_step, ptr0, !isLikeNone(stroke_width), isLikeNone(stroke_width) ? 0 : stroke_width, ptr1, len1, ptr2, len2, !isLikeNone(index), isLikeNone(index) ? 0 : index);
        return WasmVectorObject.__wrap(ret);
    } finally {
        heap[stack_pointer++] = undefined;
    }
}

/**
* @param {Function} f
* @param {number} x_min
* @param {number} x_max
* @param {number} y_min
* @param {number} y_max
* @param {number} x_step
* @param {number} y_step
* @param {WasmColor | undefined} color
* @param {number | undefined} stroke_width
* @param {string | undefined} line_cap
* @param {string | undefined} line_join
* @param {number | undefined} index
* @param {Float64Array} intervals
* @returns {WasmVectorObject}
*/
export function contourPlot(f, x_min, x_max, y_min, y_max, x_step, y_step, color, stroke_width, line_cap, line_join, index, intervals) {
    try {
        let ptr0 = 0;
        if (!isLikeNone(color)) {
            _assertClass(color, WasmColor);
            ptr0 = color.__destroy_into_raw();
        }
        var ptr1 = isLikeNone(line_cap) ? 0 : passStringToWasm0(line_cap, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        var len1 = WASM_VECTOR_LEN;
        var ptr2 = isLikeNone(line_join) ? 0 : passStringToWasm0(line_join, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        var len2 = WASM_VECTOR_LEN;
        const ptr3 = passArrayF64ToWasm0(intervals, wasm.__wbindgen_malloc);
        const len3 = WASM_VECTOR_LEN;
        const ret = wasm.contourPlot(addBorrowedObject(f), x_min, x_max, y_min, y_max, x_step, y_step, ptr0, !isLikeNone(stroke_width), isLikeNone(stroke_width) ? 0 : stroke_width, ptr1, len1, ptr2, len2, !isLikeNone(index), isLikeNone(index) ? 0 : index, ptr3, len3);
        return WasmVectorObject.__wrap(ret);
    } finally {
        heap[stack_pointer++] = undefined;
    }
}

/**
* @param {Function} f
* @param {number} x_min
* @param {number} x_max
* @param {number} x_step
* @param {WasmColor | undefined} [color]
* @param {number | undefined} [stroke_width]
* @param {string | undefined} [line_cap]
* @param {string | undefined} [line_join]
* @param {number | undefined} [index]
* @returns {WasmVectorObject}
*/
export function realFunction(f, x_min, x_max, x_step, color, stroke_width, line_cap, line_join, index) {
    try {
        let ptr0 = 0;
        if (!isLikeNone(color)) {
            _assertClass(color, WasmColor);
            ptr0 = color.__destroy_into_raw();
        }
        var ptr1 = isLikeNone(line_cap) ? 0 : passStringToWasm0(line_cap, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        var len1 = WASM_VECTOR_LEN;
        var ptr2 = isLikeNone(line_join) ? 0 : passStringToWasm0(line_join, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        var len2 = WASM_VECTOR_LEN;
        const ret = wasm.realFunction(addBorrowedObject(f), x_min, x_max, x_step, ptr0, !isLikeNone(stroke_width), isLikeNone(stroke_width) ? 0 : stroke_width, ptr1, len1, ptr2, len2, !isLikeNone(index), isLikeNone(index) ? 0 : index);
        return WasmVectorObject.__wrap(ret);
    } finally {
        heap[stack_pointer++] = undefined;
    }
}

/**
* @param {number} x_min
* @param {number} x_max
* @param {number} x_step
* @param {WasmColor | undefined} [color]
* @param {number | undefined} [stroke_width]
* @param {string | undefined} [line_cap]
* @param {string | undefined} [line_join]
* @param {number | undefined} [index]
* @param {Array<any> | undefined} [center]
* @param {number | undefined} [length]
* @param {boolean | undefined} [add_tip]
* @param {boolean | undefined} [add_ticks]
* @param {number | undefined} [tick_size]
* @param {number | undefined} [angle]
* @returns {WasmVectorObject}
*/
export function numberLine(x_min, x_max, x_step, color, stroke_width, line_cap, line_join, index, center, length, add_tip, add_ticks, tick_size, angle) {
    let ptr0 = 0;
    if (!isLikeNone(color)) {
        _assertClass(color, WasmColor);
        ptr0 = color.__destroy_into_raw();
    }
    var ptr1 = isLikeNone(line_cap) ? 0 : passStringToWasm0(line_cap, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    var len1 = WASM_VECTOR_LEN;
    var ptr2 = isLikeNone(line_join) ? 0 : passStringToWasm0(line_join, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    var len2 = WASM_VECTOR_LEN;
    const ret = wasm.numberLine(x_min, x_max, x_step, ptr0, !isLikeNone(stroke_width), isLikeNone(stroke_width) ? 0 : stroke_width, ptr1, len1, ptr2, len2, !isLikeNone(index), isLikeNone(index) ? 0 : index, isLikeNone(center) ? 0 : addHeapObject(center), !isLikeNone(length), isLikeNone(length) ? 0 : length, isLikeNone(add_tip) ? 0xFFFFFF : add_tip ? 1 : 0, isLikeNone(add_ticks) ? 0xFFFFFF : add_ticks ? 1 : 0, !isLikeNone(tick_size), isLikeNone(tick_size) ? 0 : tick_size, !isLikeNone(angle), isLikeNone(angle) ? 0 : angle);
    return WasmVectorObject.__wrap(ret);
}

/**
* @param {WasmVectorObject} number_line
* @param {number} number
* @param {number} x_min
* @param {number} x_max
* @returns {Array<any>}
*/
export function numberToPoint(number_line, number, x_min, x_max) {
    _assertClass(number_line, WasmVectorObject);
    const ret = wasm.numberToPoint(number_line.__wbg_ptr, number, x_min, x_max);
    return takeObject(ret);
}

/**
* @param {WasmVectorObject} number_line
* @param {Array<any>} point
* @param {number} x_min
* @param {number} x_max
* @returns {number}
*/
export function pointToNumber(number_line, point, x_min, x_max) {
    _assertClass(number_line, WasmVectorObject);
    const ret = wasm.pointToNumber(number_line.__wbg_ptr, addHeapObject(point), x_min, x_max);
    return ret;
}

/**
* @param {WasmVectorObject} number_line
* @param {Array<any>} numbers
* @param {Function} number_to_vector
* @param {number} x_min
* @param {number} x_max
* @param {number} height
* @param {Array<any> | undefined} [direction]
* @param {number | undefined} [buff]
* @param {number | undefined} [index]
* @returns {Promise<WasmVectorObject>}
*/
export function getNumbersTex(number_line, numbers, number_to_vector, x_min, x_max, height, direction, buff, index) {
    _assertClass(number_line, WasmVectorObject);
    var ptr0 = number_line.__destroy_into_raw();
    const ret = wasm.getNumbersTex(ptr0, addHeapObject(numbers), addHeapObject(number_to_vector), x_min, x_max, height, isLikeNone(direction) ? 0 : addHeapObject(direction), !isLikeNone(buff), isLikeNone(buff) ? 0 : buff, !isLikeNone(index), isLikeNone(index) ? 0 : index);
    return takeObject(ret);
}

/**
* @param {string} svg
* @returns {WasmVectorObject}
*/
export function svgToVector(svg) {
    const ptr0 = passStringToWasm0(svg, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    const len0 = WASM_VECTOR_LEN;
    const ret = wasm.svgToVector(ptr0, len0);
    return WasmVectorObject.__wrap(ret);
}

/**
* @param {number} angle
* @param {number} axis
* @returns {Array<any>}
*/
export function rotMatrix(angle, axis) {
    const ret = wasm.rotMatrix(angle, axis);
    return takeObject(ret);
}

/**
* @param {Array<any>} a
* @param {Array<any>} b
* @returns {Array<any>}
*/
export function matrixProduct(a, b) {
    const ret = wasm.matrixProduct(addHeapObject(a), addHeapObject(b));
    return takeObject(ret);
}

/**
* @param {number} phi
* @param {number} theta
* @param {number} gamma
* @returns {Array<any>}
*/
export function rotMatrixEuler(phi, theta, gamma) {
    const ret = wasm.rotMatrixEuler(phi, theta, gamma);
    return takeObject(ret);
}

/**
* @param {Array<any>} a
* @returns {Array<any>}
*/
export function transposeMatrix(a) {
    const ret = wasm.transposeMatrix(addHeapObject(a));
    return takeObject(ret);
}

/**
* @param {Array<any>} matrix
* @param {Array<any>} points
* @returns {Array<any>}
*/
export function applyMatrix(matrix, points) {
    const ret = wasm.applyMatrix(addHeapObject(matrix), addHeapObject(points));
    return takeObject(ret);
}

/**
* @param {Array<any>} points
* @param {Array<any>} shift
* @returns {Array<any>}
*/
export function shiftPoints3D(points, shift) {
    const ret = wasm.shiftPoints3D(addHeapObject(points), addHeapObject(shift));
    return takeObject(ret);
}

/**
* @param {WasmGradientImageOrColor} color
* @returns {WasmGradientImageOrColor}
*/
export function ensureValidThreeDColor(color) {
    _assertClass(color, WasmGradientImageOrColor);
    var ptr0 = color.__destroy_into_raw();
    const ret = wasm.ensureValidThreeDColor(ptr0);
    return WasmGradientImageOrColor.__wrap(ret);
}

/**
* @param {WasmColor} color
* @param {Array<any>} point
* @param {Array<any>} unit_normal
* @param {WasmLightSource} light_source
* @returns {WasmColor}
*/
export function getShadedRgb(color, point, unit_normal, light_source) {
    _assertClass(color, WasmColor);
    var ptr0 = color.__destroy_into_raw();
    _assertClass(light_source, WasmLightSource);
    const ret = wasm.getShadedRgb(ptr0, addHeapObject(point), addHeapObject(unit_normal), light_source.__wbg_ptr);
    return WasmColor.__wrap(ret);
}

/**
* @param {Array<any>} points
* @returns {Array<any>}
*/
export function getStartCorner(points) {
    const ret = wasm.getStartCorner(addHeapObject(points));
    return takeObject(ret);
}

/**
* @param {Array<any>} points
* @returns {Array<any>}
*/
export function getEndCorner(points) {
    const ret = wasm.getEndCorner(addHeapObject(points));
    return takeObject(ret);
}

/**
* @param {Array<any>} a
* @param {Array<any>} b
* @returns {Array<any>}
*/
export function crossProduct(a, b) {
    const ret = wasm.crossProduct(addHeapObject(a), addHeapObject(b));
    return takeObject(ret);
}

/**
* @param {Array<any>} v1
* @param {Array<any>} v2
* @returns {Array<any>}
*/
export function getUnitNormal(v1, v2) {
    const ret = wasm.getUnitNormal(addHeapObject(v1), addHeapObject(v2));
    return takeObject(ret);
}

/**
* @param {Array<any>} points
* @returns {Array<any>}
*/
export function getStartAnchors(points) {
    const ret = wasm.getStartAnchors(addHeapObject(points));
    return takeObject(ret);
}

/**
* @param {Array<any>} points
* @returns {Array<any>}
*/
export function getEndAnchors(points) {
    const ret = wasm.getEndAnchors(addHeapObject(points));
    return takeObject(ret);
}

/**
* @param {Array<any>} points
* @returns {Array<any>}
*/
export function getAnchors(points) {
    const ret = wasm.getAnchors(addHeapObject(points));
    return takeObject(ret);
}

/**
* @param {Array<any>} points
* @param {number} index
* @returns {Array<any>}
*/
export function getCornerUnitNormal(points, index) {
    const ret = wasm.getCornerUnitNormal(addHeapObject(points), index);
    return takeObject(ret);
}

/**
* @param {Array<any>} points
* @returns {Array<any>}
*/
export function getStartCornerUnitNormal(points) {
    const ret = wasm.getStartCornerUnitNormal(addHeapObject(points));
    return takeObject(ret);
}

/**
* @param {Array<any>} points
* @returns {Array<any>}
*/
export function getEndCornerUnitNormal(points) {
    const ret = wasm.getEndCornerUnitNormal(addHeapObject(points));
    return takeObject(ret);
}

/**
* @param {WasmGradientImageOrColor} color
* @param {Array<any>} points
* @param {WasmLightSource} light_source
* @param {WasmCamera} camera
* @returns {WasmGradientImageOrColor}
*/
export function getShadedColor(color, points, light_source, camera) {
    _assertClass(color, WasmGradientImageOrColor);
    _assertClass(light_source, WasmLightSource);
    _assertClass(camera, WasmCamera);
    const ret = wasm.getShadedColor(color.__wbg_ptr, addHeapObject(points), light_source.__wbg_ptr, camera.__wbg_ptr);
    return WasmGradientImageOrColor.__wrap(ret);
}

/**
* @param {Array<any>} points
* @param {WasmCamera} camera
* @returns {Array<any>}
*/
export function projectPoints(points, camera) {
    _assertClass(camera, WasmCamera);
    const ret = wasm.projectPoints(addHeapObject(points), camera.__wbg_ptr);
    return takeObject(ret);
}

/**
* @param {Array<any>} point1
* @param {Array<any>} point2
* @returns {Array<any>}
*/
export function lineAsCubicBezier3D(point1, point2) {
    const ret = wasm.lineAsCubicBezier3D(addHeapObject(point1), addHeapObject(point2));
    return takeObject(ret);
}

function passArray32ToWasm0(arg, malloc) {
    const ptr = malloc(arg.length * 4, 4) >>> 0;
    getUint32Memory0().set(arg, ptr / 4);
    WASM_VECTOR_LEN = arg.length;
    return ptr;
}
/**
* @param {string} expression
* @returns {Promise<WasmVectorObject>}
*/
export function mathjax(expression) {
    const ptr0 = passStringToWasm0(expression, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    const len0 = WASM_VECTOR_LEN;
    const ret = wasm.mathjax(ptr0, len0);
    return takeObject(ret);
}

/**
* @param {string} text
* @param {string} font_family
* @returns {Promise<WasmVectorObject>}
*/
export function textToVector(text, font_family) {
    const ptr0 = passStringToWasm0(text, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    const len0 = WASM_VECTOR_LEN;
    const ptr1 = passStringToWasm0(font_family, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    const len1 = WASM_VECTOR_LEN;
    const ret = wasm.textToVector(ptr0, len0, ptr1, len1);
    return takeObject(ret);
}

/**
* @param {number} ux
* @param {number} uy
* @param {number} vx
* @param {number} vy
* @returns {number}
*/
export function radian(ux, uy, vx, vy) {
    const ret = wasm.radian(ux, uy, vx, vy);
    return ret;
}

/**
* @param {Array<any>} last_move
* @param {number} rx
* @param {number} ry
* @param {number} rotation
* @param {boolean} large_arc
* @param {boolean} sweep
* @param {number} x
* @param {number} y
* @returns {Array<any>}
*/
export function ellipticalArcPath(last_move, rx, ry, rotation, large_arc, sweep, x, y) {
    const ret = wasm.ellipticalArcPath(addHeapObject(last_move), rx, ry, rotation, large_arc, sweep, x, y);
    return takeObject(ret);
}

/**
* @param {Array<any>} points
* @returns {Array<any>}
*/
export function getBbox(points) {
    const ret = wasm.getBbox(addHeapObject(points));
    return takeObject(ret);
}

/**
* @param {Array<any>} points
* @param {Array<any>} center_if_no_points
* @returns {Array<any>}
*/
export function center(points, center_if_no_points) {
    const ret = wasm.center(addHeapObject(points), addHeapObject(center_if_no_points));
    return takeObject(ret);
}

/**
* @param {bigint} n
* @returns {bigint}
*/
export function factorial(n) {
    const ret = wasm.factorial(n);
    return BigInt.asUintN(64, ret);
}

/**
* @param {string} hex
* @param {number} a
* @returns {WasmColor}
*/
export function hexToColor(hex, a) {
    const ptr0 = passStringToWasm0(hex, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    const len0 = WASM_VECTOR_LEN;
    const ret = wasm.hexToColor(ptr0, len0, a);
    return WasmColor.__wrap(ret);
}

/**
* @param {Array<any>} points
* @param {number} t
* @returns {Array<any>}
*/
export function bezier(points, t) {
    const ret = wasm.bezier(addHeapObject(points), t);
    return takeObject(ret);
}

/**
* @param {Array<any>} numbers
* @param {number} t
* @returns {number}
*/
export function bezierNumber(numbers, t) {
    const ret = wasm.bezierNumber(addHeapObject(numbers), t);
    return ret;
}

/**
* @param {bigint} n
* @param {bigint} r
* @returns {bigint}
*/
export function permutation(n, r) {
    const ret = wasm.permutation(n, r);
    return BigInt.asUintN(64, ret);
}

/**
* @param {bigint} n
* @param {bigint} r
* @returns {bigint}
*/
export function choose(n, r) {
    const ret = wasm.choose(n, r);
    return BigInt.asUintN(64, ret);
}

/**
* @param {number} x1
* @param {number} y1
* @param {number} x2
* @param {number} y2
* @returns {number}
*/
export function distanceSquared(x1, y1, x2, y2) {
    const ret = wasm.distanceSquared(x1, y1, x2, y2);
    return ret;
}

/**
* @param {number} x
* @param {number} y
* @param {number} t
* @returns {number}
*/
export function interpolate(x, y, t) {
    const ret = wasm.interpolate(x, y, t);
    return ret;
}

/**
* @param {Array<any>} x
* @param {Array<any>} y
* @param {number} t
* @returns {Array<any>}
*/
export function interpolateTuple(x, y, t) {
    const ret = wasm.interpolateTuple(addHeapObject(x), addHeapObject(y), t);
    return takeObject(ret);
}

/**
* @param {Array<any>} x
* @param {Array<any>} y
* @param {number} t
* @returns {Array<any>}
*/
export function interpolateTuple3D(x, y, t) {
    const ret = wasm.interpolateTuple3D(addHeapObject(x), addHeapObject(y), t);
    return takeObject(ret);
}

/**
* @param {WasmColor} x
* @param {WasmColor} y
* @param {number} t
* @returns {WasmColor}
*/
export function interpolateColor(x, y, t) {
    _assertClass(x, WasmColor);
    var ptr0 = x.__destroy_into_raw();
    _assertClass(y, WasmColor);
    var ptr1 = y.__destroy_into_raw();
    const ret = wasm.interpolateColor(ptr0, ptr1, t);
    return WasmColor.__wrap(ret);
}

/**
* @param {Array<any>} anchors1
* @param {Array<any>} handles1
* @param {Array<any>} handles2
* @param {Array<any>} anchors2
* @returns {Array<any>}
*/
export function pointsFromAnchorsAndHandles(anchors1, handles1, handles2, anchors2) {
    const ret = wasm.pointsFromAnchorsAndHandles(addHeapObject(anchors1), addHeapObject(handles1), addHeapObject(handles2), addHeapObject(anchors2));
    return takeObject(ret);
}

/**
* @param {Array<any>} points
* @param {Array<any>} point
* @returns {Array<any>}
*/
export function startNewPath(points, point) {
    const ret = wasm.startNewPath(addHeapObject(points), addHeapObject(point));
    return takeObject(ret);
}

/**
* @param {Array<any>} points
* @returns {boolean}
*/
export function hasNewPathBegun(points) {
    const ret = wasm.hasNewPathBegun(addHeapObject(points));
    return ret !== 0;
}

/**
* @param {Array<any>} points
* @param {number} n
* @returns {Array<any>}
*/
export function getNthSubpath(points, n) {
    const ret = wasm.getNthSubpath(addHeapObject(points), n);
    return takeObject(ret);
}

/**
* @param {number} n
* @param {Array<any>} points
* @returns {Array<any>}
*/
export function insertNCurvesToPointList(n, points) {
    const ret = wasm.insertNCurvesToPointList(n, addHeapObject(points));
    return takeObject(ret);
}

/**
* @param {WasmVectorObject} vec_obj1
* @param {WasmVectorObject} vec_obj2
* @returns {Array<any>}
*/
export function nullPointAlign(vec_obj1, vec_obj2) {
    _assertClass(vec_obj1, WasmVectorObject);
    var ptr0 = vec_obj1.__destroy_into_raw();
    _assertClass(vec_obj2, WasmVectorObject);
    var ptr1 = vec_obj2.__destroy_into_raw();
    const ret = wasm.nullPointAlign(ptr0, ptr1);
    return takeObject(ret);
}

/**
* @param {Array<any>} points1
* @param {Array<any>} points2
* @param {Array<any>} center_if_no_points
* @returns {Array<any>}
*/
export function alignPoints(points1, points2, center_if_no_points) {
    const ret = wasm.alignPoints(addHeapObject(points1), addHeapObject(points2), addHeapObject(center_if_no_points));
    return takeObject(ret);
}

/**
* @param {WasmVectorObject} vec_obj
* @param {number} n
* @param {Array<any>} center_if_no_points
* @returns {WasmVectorObject}
*/
export function addNMoreSubobjects(vec_obj, n, center_if_no_points) {
    _assertClass(vec_obj, WasmVectorObject);
    var ptr0 = vec_obj.__destroy_into_raw();
    const ret = wasm.addNMoreSubobjects(ptr0, n, addHeapObject(center_if_no_points));
    return WasmVectorObject.__wrap(ret);
}

/**
* @param {WasmVectorObject} vec_obj1
* @param {WasmVectorObject} vec_obj2
* @param {Array<any>} center_if_no_points
* @returns {(WasmVectorObject)[]}
*/
export function alignSubobjects(vec_obj1, vec_obj2, center_if_no_points) {
    try {
        const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
        _assertClass(vec_obj1, WasmVectorObject);
        var ptr0 = vec_obj1.__destroy_into_raw();
        _assertClass(vec_obj2, WasmVectorObject);
        var ptr1 = vec_obj2.__destroy_into_raw();
        wasm.alignSubobjects(retptr, ptr0, ptr1, addHeapObject(center_if_no_points));
        var r0 = getInt32Memory0()[retptr / 4 + 0];
        var r1 = getInt32Memory0()[retptr / 4 + 1];
        var v3 = getArrayJsValueFromWasm0(r0, r1).slice();
        wasm.__wbindgen_free(r0, r1 * 4, 4);
        return v3;
    } finally {
        wasm.__wbindgen_add_to_stack_pointer(16);
    }
}

/**
* @param {WasmVectorObject} vec_obj1
* @param {WasmVectorObject} vec_obj2
* @param {boolean} skip_point_align
* @param {Array<any>} center_if_no_points
* @returns {Array<any>}
*/
export function alignData(vec_obj1, vec_obj2, skip_point_align, center_if_no_points) {
    _assertClass(vec_obj1, WasmVectorObject);
    var ptr0 = vec_obj1.__destroy_into_raw();
    _assertClass(vec_obj2, WasmVectorObject);
    var ptr1 = vec_obj2.__destroy_into_raw();
    const ret = wasm.alignData(ptr0, ptr1, skip_point_align, addHeapObject(center_if_no_points));
    return takeObject(ret);
}

/**
* @param {number} x
* @param {number} y
* @param {number} t
* @returns {Array<any>}
*/
export function integerInterpolate(x, y, t) {
    const ret = wasm.integerInterpolate(x, y, t);
    return takeObject(ret);
}

/**
* @param {number} x1
* @param {number} y1
* @param {number} x2
* @param {number} y2
* @returns {Array<any>}
*/
export function lineAsCubicBezier(x1, y1, x2, y2) {
    const ret = wasm.lineAsCubicBezier(x1, y1, x2, y2);
    return takeObject(ret);
}

/**
* @param {number} x1
* @param {number} y1
* @param {number} x2
* @param {number} y2
* @param {number} x3
* @param {number} y3
* @returns {Array<any>}
*/
export function quadraticBezierAsCubicBezier(x1, y1, x2, y2, x3, y3) {
    const ret = wasm.quadraticBezierAsCubicBezier(x1, y1, x2, y2, x3, y3);
    return takeObject(ret);
}

/**
* @param {number} x1
* @param {number} y1
* @param {number} x2
* @param {number} y2
* @returns {boolean}
*/
export function considerPointsEquals(x1, y1, x2, y2) {
    const ret = wasm.considerPointsEquals(x1, y1, x2, y2);
    return ret !== 0;
}

/**
* @param {number} t
* @returns {number}
*/
export function sigmoid(t) {
    const ret = wasm.sigmoid(t);
    return ret;
}

/**
* @param {number} t
* @returns {number}
*/
export function linear(t) {
    const ret = wasm.linear(t);
    return ret;
}

/**
* @param {number} t
* @param {number} inflection
* @returns {number}
*/
export function smooth(t, inflection) {
    const ret = wasm.smooth(t, inflection);
    return ret;
}

/**
* @param {number} t
* @returns {number}
*/
export function smoothstep(t) {
    const ret = wasm.smoothstep(t);
    return ret;
}

/**
* @param {number} t
* @returns {number}
*/
export function smootherstep(t) {
    const ret = wasm.smootherstep(t);
    return ret;
}

/**
* @param {number} t
* @returns {number}
*/
export function smoothererstep(t) {
    const ret = wasm.smoothererstep(t);
    return ret;
}

/**
* @param {number} t
* @param {number} inflection
* @returns {number}
*/
export function rushInto(t, inflection) {
    const ret = wasm.rushInto(t, inflection);
    return ret;
}

/**
* @param {number} t
* @param {number} inflection
* @returns {number}
*/
export function rushFrom(t, inflection) {
    const ret = wasm.rushFrom(t, inflection);
    return ret;
}

/**
* @param {number} t
* @returns {number}
*/
export function slowInto(t) {
    const ret = wasm.easeOutCirc(t);
    return ret;
}

/**
* @param {number} t
* @returns {number}
*/
export function doubleSmooth(t) {
    const ret = wasm.doubleSmooth(t);
    return ret;
}

/**
* @param {number} t
* @param {number} inflection
* @returns {number}
*/
export function thereAndBack(t, inflection) {
    const ret = wasm.thereAndBack(t, inflection);
    return ret;
}

/**
* @param {number} t
* @param {number} pause_ratio
* @returns {number}
*/
export function thereAndBackWithPause(t, pause_ratio) {
    const ret = wasm.thereAndBackWithPause(t, pause_ratio);
    return ret;
}

/**
* @param {number} t
* @param {number} pull_factor
* @returns {number}
*/
export function runningStart(t, pull_factor) {
    const ret = wasm.runningStart(t, pull_factor);
    return ret;
}

/**
* @param {Function} func
* @param {number} t
* @param {number} proportion
* @returns {number}
*/
export function notQuiteThere(func, t, proportion) {
    const ret = wasm.notQuiteThere(addHeapObject(func), t, proportion);
    return ret;
}

/**
* @param {number} t
* @param {number} wiggles
* @returns {number}
*/
export function wiggle(t, wiggles) {
    const ret = wasm.wiggle(t, wiggles);
    return ret;
}

/**
* @param {Function} func
* @param {number} t
* @param {number} a
* @param {number} b
* @returns {number}
*/
export function squishRateFunc(func, t, a, b) {
    const ret = wasm.squishRateFunc(addHeapObject(func), t, a, b);
    return ret;
}

/**
* @param {number} t
* @returns {number}
*/
export function lingering(t) {
    const ret = wasm.lingering(t);
    return ret;
}

/**
* @param {number} t
* @param {number} half_life
* @returns {number}
*/
export function exponentialDecay(t, half_life) {
    const ret = wasm.exponentialDecay(t, half_life);
    return ret;
}

/**
* @param {number} t
* @returns {number}
*/
export function easeInSine(t) {
    const ret = wasm.easeInSine(t);
    return ret;
}

/**
* @param {number} t
* @returns {number}
*/
export function easeOutSine(t) {
    const ret = wasm.easeOutSine(t);
    return ret;
}

/**
* @param {number} t
* @returns {number}
*/
export function easeInOutSine(t) {
    const ret = wasm.easeInOutSine(t);
    return ret;
}

/**
* @param {number} t
* @returns {number}
*/
export function easeInQuad(t) {
    const ret = wasm.easeInQuad(t);
    return ret;
}

/**
* @param {number} t
* @returns {number}
*/
export function easeOutQuad(t) {
    const ret = wasm.easeOutQuad(t);
    return ret;
}

/**
* @param {number} t
* @returns {number}
*/
export function easeInOutQuad(t) {
    const ret = wasm.easeInOutQuad(t);
    return ret;
}

/**
* @param {number} t
* @returns {number}
*/
export function easeInCubic(t) {
    const ret = wasm.easeInCubic(t);
    return ret;
}

/**
* @param {number} t
* @returns {number}
*/
export function easeOutCubic(t) {
    const ret = wasm.easeOutCubic(t);
    return ret;
}

/**
* @param {number} t
* @returns {number}
*/
export function easeInOutCubic(t) {
    const ret = wasm.easeInOutCubic(t);
    return ret;
}

/**
* @param {number} t
* @returns {number}
*/
export function easeInQuart(t) {
    const ret = wasm.easeInQuart(t);
    return ret;
}

/**
* @param {number} t
* @returns {number}
*/
export function easeOutQuart(t) {
    const ret = wasm.easeOutQuart(t);
    return ret;
}

/**
* @param {number} t
* @returns {number}
*/
export function easeInOutQuart(t) {
    const ret = wasm.easeInOutQuart(t);
    return ret;
}

/**
* @param {number} t
* @returns {number}
*/
export function easeInQuint(t) {
    const ret = wasm.easeInQuint(t);
    return ret;
}

/**
* @param {number} t
* @returns {number}
*/
export function easeOutQuint(t) {
    const ret = wasm.easeOutQuint(t);
    return ret;
}

/**
* @param {number} t
* @returns {number}
*/
export function easeInOutQuint(t) {
    const ret = wasm.easeInOutQuint(t);
    return ret;
}

/**
* @param {number} t
* @returns {number}
*/
export function easeInExpo(t) {
    const ret = wasm.easeInExpo(t);
    return ret;
}

/**
* @param {number} t
* @returns {number}
*/
export function easeOutExpo(t) {
    const ret = wasm.easeOutExpo(t);
    return ret;
}

/**
* @param {number} t
* @returns {number}
*/
export function easeInOutExpo(t) {
    const ret = wasm.easeInOutExpo(t);
    return ret;
}

/**
* @param {number} t
* @returns {number}
*/
export function easeInCirc(t) {
    const ret = wasm.easeInCirc(t);
    return ret;
}

/**
* @param {number} t
* @returns {number}
*/
export function easeOutCirc(t) {
    const ret = wasm.easeOutCirc(t);
    return ret;
}

/**
* @param {number} t
* @returns {number}
*/
export function easeInOutCirc(t) {
    const ret = wasm.easeInOutCirc(t);
    return ret;
}

/**
* @param {number} t
* @returns {number}
*/
export function easeInBack(t) {
    const ret = wasm.easeInBack(t);
    return ret;
}

/**
* @param {number} t
* @returns {number}
*/
export function easeOutBack(t) {
    const ret = wasm.easeInBack(t);
    return ret;
}

/**
* @param {number} t
* @returns {number}
*/
export function easeInOutBack(t) {
    const ret = wasm.easeInOutBack(t);
    return ret;
}

/**
* @param {number} t
* @returns {number}
*/
export function easeInElastic(t) {
    const ret = wasm.easeInElastic(t);
    return ret;
}

/**
* @param {number} t
* @returns {number}
*/
export function easeOutElastic(t) {
    const ret = wasm.easeOutElastic(t);
    return ret;
}

/**
* @param {number} t
* @returns {number}
*/
export function easeInOutElastic(t) {
    const ret = wasm.easeInOutElastic(t);
    return ret;
}

/**
* @param {number} t
* @returns {number}
*/
export function easeOutBounce(t) {
    const ret = wasm.easeInBounce(t);
    return ret;
}

/**
* @param {number} t
* @returns {number}
*/
export function easeInBounce(t) {
    const ret = wasm.easeInBounce(t);
    return ret;
}

/**
* @param {number} t
* @returns {number}
*/
export function easeInOutBounce(t) {
    const ret = wasm.easeInOutBounce(t);
    return ret;
}

function getArrayF64FromWasm0(ptr, len) {
    ptr = ptr >>> 0;
    return getFloat64Memory0().subarray(ptr / 8, ptr / 8 + len);
}
/**
* @param {number} num_anim_funcs
* @param {number} lag_ratio
* @returns {Float64Array}
*/
export function makeTimings(num_anim_funcs, lag_ratio) {
    try {
        const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
        wasm.makeTimings(retptr, num_anim_funcs, lag_ratio);
        var r0 = getInt32Memory0()[retptr / 4 + 0];
        var r1 = getInt32Memory0()[retptr / 4 + 1];
        var v1 = getArrayF64FromWasm0(r0, r1).slice();
        wasm.__wbindgen_free(r0, r1 * 8, 8);
        return v1;
    } finally {
        wasm.__wbindgen_add_to_stack_pointer(16);
    }
}

/**
* @param {WasmVectorObject} vec_obj
* @param {(Function)[]} anim_funcs
* @param {number} lag_ratio
* @param {number} t
* @returns {WasmVectorObject}
*/
export function animationGroup(vec_obj, anim_funcs, lag_ratio, t) {
    _assertClass(vec_obj, WasmVectorObject);
    var ptr0 = vec_obj.__destroy_into_raw();
    const ptr1 = passArrayJsValueToWasm0(anim_funcs, wasm.__wbindgen_malloc);
    const len1 = WASM_VECTOR_LEN;
    const ret = wasm.animationGroup(ptr0, ptr1, len1, lag_ratio, t);
    return WasmVectorObject.__wrap(ret);
}

/**
* @param {WasmVectorObject} vec_obj
* @param {number} t
* @returns {WasmVectorObject}
*/
export function create(vec_obj, t) {
    _assertClass(vec_obj, WasmVectorObject);
    var ptr0 = vec_obj.__destroy_into_raw();
    const ret = wasm.create(ptr0, t);
    return WasmVectorObject.__wrap(ret);
}

/**
* @param {WasmVectorObject} vec_obj
* @param {number} t
* @returns {WasmVectorObject}
*/
export function drawStrokeThenFill(vec_obj, t) {
    _assertClass(vec_obj, WasmVectorObject);
    var ptr0 = vec_obj.__destroy_into_raw();
    const ret = wasm.drawStrokeThenFill(ptr0, t);
    return WasmVectorObject.__wrap(ret);
}

/**
* @param {WasmVectorObject} vec_obj
* @param {number} lag_ratio
* @param {number} t
* @returns {WasmVectorObject}
*/
export function write(vec_obj, lag_ratio, t) {
    _assertClass(vec_obj, WasmVectorObject);
    var ptr0 = vec_obj.__destroy_into_raw();
    const ret = wasm.write(ptr0, lag_ratio, t);
    return WasmVectorObject.__wrap(ret);
}

/**
* @param {WasmVectorObject} vec_obj
* @param {number} scale_factor
* @param {Array<any>} shift
* @param {number} t
* @returns {WasmVectorObject}
*/
export function fadeIn(vec_obj, scale_factor, shift, t) {
    _assertClass(vec_obj, WasmVectorObject);
    var ptr0 = vec_obj.__destroy_into_raw();
    const ret = wasm.fadeIn(ptr0, scale_factor, addHeapObject(shift), t);
    return WasmVectorObject.__wrap(ret);
}

/**
* @param {WasmVectorObject} vec_obj
* @param {number} scale_factor
* @param {Array<any>} shift
* @param {number} t
* @returns {WasmVectorObject}
*/
export function fadeOut(vec_obj, scale_factor, shift, t) {
    _assertClass(vec_obj, WasmVectorObject);
    var ptr0 = vec_obj.__destroy_into_raw();
    const ret = wasm.fadeOut(ptr0, scale_factor, addHeapObject(shift), t);
    return WasmVectorObject.__wrap(ret);
}

/**
* @param {WasmVectorObject} vec_obj
* @param {number} t
* @returns {WasmVectorObject}
*/
export function growArrowWithFinalTip(vec_obj, t) {
    _assertClass(vec_obj, WasmVectorObject);
    var ptr0 = vec_obj.__destroy_into_raw();
    const ret = wasm.growArrowWithFinalTip(ptr0, t);
    return WasmVectorObject.__wrap(ret);
}

/**
* @param {WasmVectorObject} vec_obj
* @param {number} t
* @returns {WasmVectorObject}
*/
export function growArrowWithInitialTip(vec_obj, t) {
    _assertClass(vec_obj, WasmVectorObject);
    var ptr0 = vec_obj.__destroy_into_raw();
    const ret = wasm.growArrowWithInitialTip(ptr0, t);
    return WasmVectorObject.__wrap(ret);
}

/**
* @param {WasmVectorObject} vec_obj
* @param {number} t
* @returns {WasmVectorObject}
*/
export function growArrowWithTipsAtBothEnds(vec_obj, t) {
    _assertClass(vec_obj, WasmVectorObject);
    var ptr0 = vec_obj.__destroy_into_raw();
    const ret = wasm.growArrowWithTipsAtBothEnds(ptr0, t);
    return WasmVectorObject.__wrap(ret);
}

/**
* @param {WasmVectorObject} vec_obj
* @param {number} t
* @returns {WasmVectorObject}
*/
export function growFromCenter(vec_obj, t) {
    _assertClass(vec_obj, WasmVectorObject);
    var ptr0 = vec_obj.__destroy_into_raw();
    const ret = wasm.growFromCenter(ptr0, t);
    return WasmVectorObject.__wrap(ret);
}

/**
* @param {WasmVectorObject} original
* @param {WasmVectorObject} target
* @param {number} t
* @returns {WasmVectorObject}
*/
export function morphShape(original, target, t) {
    _assertClass(original, WasmVectorObject);
    var ptr0 = original.__destroy_into_raw();
    _assertClass(target, WasmVectorObject);
    var ptr1 = target.__destroy_into_raw();
    const ret = wasm.morphShape(ptr0, ptr1, t);
    return WasmVectorObject.__wrap(ret);
}

/**
* @param {Array<any>} top_left_corner
* @param {Array<any>} bottom_right_corner
* @param {SVGScene} scene
* @param {number} t
*/
export function moveCameraSVG(top_left_corner, bottom_right_corner, scene, t) {
    _assertClass(scene, SVGScene);
    wasm.moveCameraSVG(addHeapObject(top_left_corner), addHeapObject(bottom_right_corner), scene.__wbg_ptr, t);
}

/**
* @param {Array<any>} top_left_corner
* @param {Array<any>} bottom_right_corner
* @param {Scene} scene
* @param {number} t
*/
export function moveCamera(top_left_corner, bottom_right_corner, scene, t) {
    _assertClass(scene, Scene);
    wasm.moveCamera(addHeapObject(top_left_corner), addHeapObject(bottom_right_corner), scene.__wbg_ptr, t);
}

/**
* @param {WasmVectorObject} vec_obj
* @param {number} angle
* @param {number} t
* @returns {WasmVectorObject}
*/
export function rotateAnimation(vec_obj, angle, t) {
    _assertClass(vec_obj, WasmVectorObject);
    var ptr0 = vec_obj.__destroy_into_raw();
    const ret = wasm.rotateAnimation(ptr0, angle, t);
    return WasmVectorObject.__wrap(ret);
}

/**
* @param {WasmVectorObject} vec_obj
* @param {number} scale_factor
* @param {number} t
* @returns {WasmVectorObject}
*/
export function scaleInPlace(vec_obj, scale_factor, t) {
    _assertClass(vec_obj, WasmVectorObject);
    var ptr0 = vec_obj.__destroy_into_raw();
    const ret = wasm.scaleInPlace(ptr0, scale_factor, t);
    return WasmVectorObject.__wrap(ret);
}

/**
* @param {WasmVectorObject} vec_obj
* @param {WasmColor} target_fill
* @param {number} t
* @returns {WasmVectorObject}
*/
export function setFillAnimation(vec_obj, target_fill, t) {
    _assertClass(vec_obj, WasmVectorObject);
    var ptr0 = vec_obj.__destroy_into_raw();
    _assertClass(target_fill, WasmColor);
    var ptr1 = target_fill.__destroy_into_raw();
    const ret = wasm.setFillAnimation(ptr0, ptr1, t);
    return WasmVectorObject.__wrap(ret);
}

/**
* @param {WasmVectorObject} vec_obj
* @param {WasmColor} target_stroke
* @param {number} t
* @returns {WasmVectorObject}
*/
export function setStrokeAnimation(vec_obj, target_stroke, t) {
    _assertClass(vec_obj, WasmVectorObject);
    var ptr0 = vec_obj.__destroy_into_raw();
    _assertClass(target_stroke, WasmColor);
    var ptr1 = target_stroke.__destroy_into_raw();
    const ret = wasm.setStrokeAnimation(ptr0, ptr1, t);
    return WasmVectorObject.__wrap(ret);
}

/**
* @param {WasmVectorObject} vec_obj
* @param {Array<any>} shift
* @param {number} t
* @returns {WasmVectorObject}
*/
export function shiftAnimation(vec_obj, shift, t) {
    _assertClass(vec_obj, WasmVectorObject);
    var ptr0 = vec_obj.__destroy_into_raw();
    const ret = wasm.shiftAnimation(ptr0, addHeapObject(shift), t);
    return WasmVectorObject.__wrap(ret);
}

/**
* @param {WasmVectorObject} vec_obj
* @param {number} t
* @returns {WasmVectorObject}
*/
export function showTemporaily(vec_obj, t) {
    _assertClass(vec_obj, WasmVectorObject);
    var ptr0 = vec_obj.__destroy_into_raw();
    const ret = wasm.showTemporaily(ptr0, t);
    return WasmVectorObject.__wrap(ret);
}

/**
* @param {WasmVectorObject} vec_obj
* @param {number} angle
* @param {number} t
* @returns {WasmVectorObject}
*/
export function spinningGrow(vec_obj, angle, t) {
    _assertClass(vec_obj, WasmVectorObject);
    var ptr0 = vec_obj.__destroy_into_raw();
    const ret = wasm.spinningGrow(ptr0, angle, t);
    return WasmVectorObject.__wrap(ret);
}

function handleError(f, args) {
    try {
        return f.apply(this, args);
    } catch (e) {
        wasm.__wbindgen_exn_store(addHeapObject(e));
    }
}
function __wbg_adapter_553(arg0, arg1, arg2, arg3) {
    wasm.wasm_bindgen__convert__closures__invoke2_mut__h271b87efd55193df(arg0, arg1, addHeapObject(arg2), addHeapObject(arg3));
}

const GenericSceneFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_genericscene_free(ptr >>> 0));
/**
*/
export class GenericScene {

    static __wrap(ptr) {
        ptr = ptr >>> 0;
        const obj = Object.create(GenericScene.prototype);
        obj.__wbg_ptr = ptr;
        GenericSceneFinalization.register(obj, obj.__wbg_ptr, obj);
        return obj;
    }

    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        GenericSceneFinalization.unregister(this);
        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_genericscene_free(ptr);
    }
    /**
    * @param {Scene} scene
    * @returns {GenericScene}
    */
    static fromScene(scene) {
        _assertClass(scene, Scene);
        var ptr0 = scene.__destroy_into_raw();
        const ret = wasm.genericscene_fromScene(ptr0);
        return GenericScene.__wrap(ret);
    }
    /**
    * @param {SVGScene} scene
    * @returns {GenericScene}
    */
    static fromSVGScene(scene) {
        _assertClass(scene, SVGScene);
        var ptr0 = scene.__destroy_into_raw();
        const ret = wasm.genericscene_fromSVGScene(ptr0);
        return GenericScene.__wrap(ret);
    }
    /**
    * @returns {boolean}
    */
    isScene() {
        const ret = wasm.genericscene_isScene(this.__wbg_ptr);
        return ret !== 0;
    }
    /**
    * @returns {boolean}
    */
    isSVGScene() {
        const ret = wasm.genericscene_isSVGScene(this.__wbg_ptr);
        return ret !== 0;
    }
    /**
    * @returns {boolean}
    */
    isVideoScene() {
        const ret = wasm.genericscene_isVideoScene(this.__wbg_ptr);
        return ret !== 0;
    }
    /**
    * @returns {bigint}
    */
    getFps() {
        const ret = wasm.genericscene_getFps(this.__wbg_ptr);
        return BigInt.asUintN(64, ret);
    }
    /**
    * @returns {bigint}
    */
    getHeight() {
        const ret = wasm.genericscene_getHeight(this.__wbg_ptr);
        return BigInt.asUintN(64, ret);
    }
    /**
    * @returns {bigint}
    */
    getWidth() {
        const ret = wasm.genericscene_getWidth(this.__wbg_ptr);
        return BigInt.asUintN(64, ret);
    }
    /**
    */
    renderFrame() {
        wasm.genericscene_renderFrame(this.__wbg_ptr);
    }
    /**
    */
    clear() {
        wasm.genericscene_clear(this.__wbg_ptr);
    }
    /**
    * @param {number} n
    */
    restore(n) {
        wasm.genericscene_restore(this.__wbg_ptr, n);
    }
    /**
    * @param {number} n
    */
    saveState(n) {
        wasm.genericscene_saveState(this.__wbg_ptr, n);
    }
    /**
    * @param {number} x
    * @param {number} y
    */
    setTopLeftCorner(x, y) {
        wasm.genericscene_setTopLeftCorner(this.__wbg_ptr, x, y);
    }
    /**
    * @param {number} x
    * @param {number} y
    */
    setBottomRightCorner(x, y) {
        wasm.genericscene_setBottomRightCorner(this.__wbg_ptr, x, y);
    }
    /**
    * @returns {Array<any>}
    */
    getTopLeftCorner() {
        const ret = wasm.genericscene_getTopLeftCorner(this.__wbg_ptr);
        return takeObject(ret);
    }
    /**
    * @returns {Array<any>}
    */
    getBottomRightCorner() {
        const ret = wasm.genericscene_getBottomRightCorner(this.__wbg_ptr);
        return takeObject(ret);
    }
    /**
    * @param {WasmGradientImageOrColor} color
    */
    setBackground(color) {
        _assertClass(color, WasmGradientImageOrColor);
        var ptr0 = color.__destroy_into_raw();
        wasm.genericscene_setBackground(this.__wbg_ptr, ptr0);
    }
    /**
    * @param {WasmVectorObject} object
    */
    add(object) {
        _assertClass(object, WasmVectorObject);
        var ptr0 = object.__destroy_into_raw();
        wasm.genericscene_add(this.__wbg_ptr, ptr0);
    }
    /**
    * @param {number} index
    * @param {WasmVectorObject} object
    */
    insert(index, object) {
        _assertClass(object, WasmVectorObject);
        var ptr0 = object.__destroy_into_raw();
        wasm.genericscene_insert(this.__wbg_ptr, index, ptr0);
    }
    /**
    * @param {number} index
    */
    remove(index) {
        wasm.genericscene_remove(this.__wbg_ptr, index);
    }
    /**
    * @returns {Array<any>}
    */
    getObjects() {
        const ret = wasm.genericscene_getObjects(this.__wbg_ptr);
        return takeObject(ret);
    }
    /**
    * @param {Array<any>} object_indices
    * @returns {Map<any, any>}
    */
    getObjectsFromIndices(object_indices) {
        const ret = wasm.genericscene_getObjectsFromIndices(this.__wbg_ptr, addHeapObject(object_indices));
        return takeObject(ret);
    }
    /**
    * @param {CanvasRenderingContext2D} context
    */
    setCanvasContext(context) {
        wasm.genericscene_setCanvasContext(this.__wbg_ptr, addHeapObject(context));
    }
    /**
    * @param {HTMLDivElement} div_container
    */
    setDivContainer(div_container) {
        wasm.genericscene_setDivContainer(this.__wbg_ptr, addHeapObject(div_container));
    }
    /**
    * @param {number} duration_in_ms
    * @returns {Promise<void>}
    */
    sleep(duration_in_ms) {
        const ret = wasm.genericscene_sleep(this.__wbg_ptr, duration_in_ms);
        return takeObject(ret);
    }
    /**
    * @param {Array<any>} objects
    */
    setObjects(objects) {
        wasm.genericscene_setObjects(this.__wbg_ptr, addHeapObject(objects));
    }
    /**
    * @param {Function} animation_func
    * @param {Uint32Array} object_indices
    * @param {bigint} duration_in_frames
    * @param {Function} rate_func
    * @returns {Promise<void>}
    */
    play(animation_func, object_indices, duration_in_frames, rate_func) {
        const ptr0 = passArray32ToWasm0(object_indices, wasm.__wbindgen_malloc);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.genericscene_play(this.__wbg_ptr, addHeapObject(animation_func), ptr0, len0, duration_in_frames, addHeapObject(rate_func));
        return takeObject(ret);
    }
    /**
    * @param {Function} animation_func
    * @param {Array<any>} objects
    * @param {number} t
    * @returns {Promise<void>}
    */
    makeFrame(animation_func, objects, t) {
        const ret = wasm.genericscene_makeFrame(this.__wbg_ptr, addHeapObject(animation_func), addHeapObject(objects), t);
        return takeObject(ret);
    }
    /**
    * @param {bigint} duration_in_frames
    * @returns {Promise<void>}
    */
    wait(duration_in_frames) {
        const ret = wasm.genericscene_wait(this.__wbg_ptr, duration_in_frames);
        return takeObject(ret);
    }
    /**
    * @param {Function} callback
    */
    setCallback(callback) {
        wasm.genericscene_setCallback(this.__wbg_ptr, addHeapObject(callback));
    }
    /**
    * @returns {Promise<void>}
    */
    callCallback() {
        const ret = wasm.genericscene_callCallback(this.__wbg_ptr);
        return takeObject(ret);
    }
    /**
    * @param {number} index
    * @param {string} _class
    */
    setClass(index, _class) {
        const ptr0 = passStringToWasm0(_class, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        wasm.genericscene_setClass(this.__wbg_ptr, index, ptr0, len0);
    }
    /**
    * @param {number} index
    */
    setStyle(index) {
        wasm.genericscene_setStyle(this.__wbg_ptr, index);
    }
}

const SVGSceneFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_svgscene_free(ptr >>> 0));
/**
*/
export class SVGScene {

    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        SVGSceneFinalization.unregister(this);
        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_svgscene_free(ptr);
    }
    /**
    * @param {bigint} width
    * @param {bigint} height
    * @param {bigint} fps
    */
    constructor(width, height, fps) {
        const ret = wasm.svgscene_new_js(width, height, fps);
        this.__wbg_ptr = ret >>> 0;
        return this;
    }
    /**
    * @returns {bigint}
    */
    getFps() {
        const ret = wasm.svgscene_getFps(this.__wbg_ptr);
        return BigInt.asUintN(64, ret);
    }
    /**
    * @returns {bigint}
    */
    getHeight() {
        const ret = wasm.svgscene_getHeight(this.__wbg_ptr);
        return BigInt.asUintN(64, ret);
    }
    /**
    * @returns {bigint}
    */
    getWidth() {
        const ret = wasm.svgscene_getWidth(this.__wbg_ptr);
        return BigInt.asUintN(64, ret);
    }
    /**
    */
    renderFrame() {
        wasm.svgscene_renderFrame(this.__wbg_ptr);
    }
    /**
    */
    clear() {
        wasm.svgscene_clear(this.__wbg_ptr);
    }
    /**
    * @param {number} n
    */
    restore(n) {
        wasm.svgscene_restore(this.__wbg_ptr, n);
    }
    /**
    * @param {number} n
    */
    saveState(n) {
        wasm.svgscene_saveState(this.__wbg_ptr, n);
    }
    /**
    * @param {number} x
    * @param {number} y
    */
    setTopLeftCorner(x, y) {
        wasm.svgscene_setTopLeftCorner(this.__wbg_ptr, x, y);
    }
    /**
    * @param {number} x
    * @param {number} y
    */
    setBottomRightCorner(x, y) {
        wasm.svgscene_setBottomRightCorner(this.__wbg_ptr, x, y);
    }
    /**
    * @returns {Array<any>}
    */
    getTopLeftCorner() {
        const ret = wasm.svgscene_getTopLeftCorner(this.__wbg_ptr);
        return takeObject(ret);
    }
    /**
    * @returns {Array<any>}
    */
    getBottomRightCorner() {
        const ret = wasm.svgscene_getBottomRightCorner(this.__wbg_ptr);
        return takeObject(ret);
    }
    /**
    * @param {WasmGradientImageOrColor} background
    */
    setBackground(background) {
        _assertClass(background, WasmGradientImageOrColor);
        var ptr0 = background.__destroy_into_raw();
        wasm.svgscene_setBackground(this.__wbg_ptr, ptr0);
    }
    /**
    * @param {WasmVectorObject} vec_obj
    */
    add(vec_obj) {
        _assertClass(vec_obj, WasmVectorObject);
        var ptr0 = vec_obj.__destroy_into_raw();
        wasm.svgscene_add(this.__wbg_ptr, ptr0);
    }
    /**
    * @param {number} index
    * @param {WasmVectorObject} vec_obj
    */
    insert(index, vec_obj) {
        _assertClass(vec_obj, WasmVectorObject);
        var ptr0 = vec_obj.__destroy_into_raw();
        wasm.svgscene_insert(this.__wbg_ptr, index, ptr0);
    }
    /**
    * @param {number} index
    */
    remove(index) {
        wasm.svgscene_remove(this.__wbg_ptr, index);
    }
    /**
    * @param {Array<any>} object_indices
    * @returns {Map<any, any>}
    */
    getObjectsFromIndices(object_indices) {
        const ret = wasm.svgscene_getObjectsFromIndices(this.__wbg_ptr, addHeapObject(object_indices));
        return takeObject(ret);
    }
    /**
    * @param {HTMLDivElement} div_container
    */
    setDivContainer(div_container) {
        wasm.svgscene_setDivContainer(this.__wbg_ptr, addHeapObject(div_container));
    }
    /**
    * @param {number} duration_in_ms
    * @returns {Promise<void>}
    */
    sleep(duration_in_ms) {
        const ret = wasm.svgscene_sleep(this.__wbg_ptr, duration_in_ms);
        return takeObject(ret);
    }
    /**
    * @param {Function} animation_func
    * @param {Uint32Array} object_indices
    * @param {bigint} duration_in_frames
    * @param {Function} rate_func
    * @returns {Promise<void>}
    */
    play(animation_func, object_indices, duration_in_frames, rate_func) {
        const ptr0 = passArray32ToWasm0(object_indices, wasm.__wbindgen_malloc);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.svgscene_play(this.__wbg_ptr, addHeapObject(animation_func), ptr0, len0, duration_in_frames, addHeapObject(rate_func));
        return takeObject(ret);
    }
    /**
    * @param {Function} animation_func
    * @param {Array<any>} objects
    * @param {number} t
    * @returns {Promise<void>}
    */
    makeFrame(animation_func, objects, t) {
        const ret = wasm.svgscene_makeFrame(this.__wbg_ptr, addHeapObject(animation_func), addHeapObject(objects), t);
        return takeObject(ret);
    }
    /**
    * @param {Array<any>} objects
    */
    setObjects(objects) {
        wasm.svgscene_setObjects(this.__wbg_ptr, addHeapObject(objects));
    }
    /**
    * @returns {Array<any>}
    */
    getObjects() {
        const ret = wasm.svgscene_getObjects(this.__wbg_ptr);
        return takeObject(ret);
    }
    /**
    * @param {bigint} duration_in_frames
    * @returns {Promise<void>}
    */
    wait(duration_in_frames) {
        const ret = wasm.svgscene_wait(this.__wbg_ptr, duration_in_frames);
        return takeObject(ret);
    }
    /**
    * @param {Function} callback
    */
    setCallback(callback) {
        wasm.svgscene_setCallback(this.__wbg_ptr, addHeapObject(callback));
    }
    /**
    * @returns {Promise<void>}
    */
    callCallback() {
        const ret = wasm.svgscene_callCallback(this.__wbg_ptr);
        return takeObject(ret);
    }
    /**
    * @param {number} index
    * @param {string} id
    */
    setClass(index, id) {
        const ptr0 = passStringToWasm0(id, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        wasm.svgscene_setClass(this.__wbg_ptr, index, ptr0, len0);
    }
    /**
    * @param {number} index
    */
    removeClass(index) {
        wasm.svgscene_removeClass(this.__wbg_ptr, index);
    }
}

const SceneFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_scene_free(ptr >>> 0));
/**
*/
export class Scene {

    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        SceneFinalization.unregister(this);
        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_scene_free(ptr);
    }
    /**
    * @param {bigint} width
    * @param {bigint} height
    * @param {bigint} fps
    */
    constructor(width, height, fps) {
        const ret = wasm.scene_new_js(width, height, fps);
        this.__wbg_ptr = ret >>> 0;
        return this;
    }
    /**
    * @returns {bigint}
    */
    getFps() {
        const ret = wasm.scene_getFps(this.__wbg_ptr);
        return BigInt.asUintN(64, ret);
    }
    /**
    * @returns {bigint}
    */
    getHeight() {
        const ret = wasm.scene_getHeight(this.__wbg_ptr);
        return BigInt.asUintN(64, ret);
    }
    /**
    * @returns {bigint}
    */
    getWidth() {
        const ret = wasm.scene_getWidth(this.__wbg_ptr);
        return BigInt.asUintN(64, ret);
    }
    /**
    */
    renderFrame() {
        wasm.scene_renderFrame(this.__wbg_ptr);
    }
    /**
    */
    clear() {
        wasm.scene_clear(this.__wbg_ptr);
    }
    /**
    * @param {number} n
    */
    restore(n) {
        wasm.scene_restore(this.__wbg_ptr, n);
    }
    /**
    * @param {number} n
    */
    saveState(n) {
        wasm.scene_saveState(this.__wbg_ptr, n);
    }
    /**
    * @param {number} x
    * @param {number} y
    */
    setTopLeftCorner(x, y) {
        wasm.scene_setTopLeftCorner(this.__wbg_ptr, x, y);
    }
    /**
    * @param {number} x
    * @param {number} y
    */
    setBottomRightCorner(x, y) {
        wasm.scene_setBottomRightCorner(this.__wbg_ptr, x, y);
    }
    /**
    * @returns {Array<any>}
    */
    getTopLeftCorner() {
        const ret = wasm.scene_getTopLeftCorner(this.__wbg_ptr);
        return takeObject(ret);
    }
    /**
    * @returns {Array<any>}
    */
    getBottomRightCorner() {
        const ret = wasm.scene_getBottomRightCorner(this.__wbg_ptr);
        return takeObject(ret);
    }
    /**
    * @param {WasmGradientImageOrColor} background
    */
    setBackground(background) {
        _assertClass(background, WasmGradientImageOrColor);
        var ptr0 = background.__destroy_into_raw();
        wasm.scene_setBackground(this.__wbg_ptr, ptr0);
    }
    /**
    * @param {WasmVectorObject} vec_obj
    */
    add(vec_obj) {
        _assertClass(vec_obj, WasmVectorObject);
        var ptr0 = vec_obj.__destroy_into_raw();
        wasm.scene_add(this.__wbg_ptr, ptr0);
    }
    /**
    * @param {number} index
    * @param {WasmVectorObject} vec_obj
    */
    insert(index, vec_obj) {
        _assertClass(vec_obj, WasmVectorObject);
        var ptr0 = vec_obj.__destroy_into_raw();
        wasm.scene_insert(this.__wbg_ptr, index, ptr0);
    }
    /**
    * @param {number} index
    */
    remove(index) {
        wasm.scene_remove(this.__wbg_ptr, index);
    }
    /**
    * @returns {Array<any>}
    */
    getObjects() {
        const ret = wasm.scene_getObjects(this.__wbg_ptr);
        return takeObject(ret);
    }
    /**
    * @param {Array<any>} object_indices
    * @returns {Map<any, any>}
    */
    getObjectsFromIndices(object_indices) {
        const ret = wasm.scene_getObjectsFromIndices(this.__wbg_ptr, addHeapObject(object_indices));
        return takeObject(ret);
    }
    /**
    * @param {CanvasRenderingContext2D} context
    */
    setCanvasContext(context) {
        wasm.scene_setCanvasContext(this.__wbg_ptr, addHeapObject(context));
    }
    /**
    * @param {number} duration_in_ms
    * @returns {Promise<void>}
    */
    sleep(duration_in_ms) {
        const ret = wasm.scene_sleep(this.__wbg_ptr, duration_in_ms);
        return takeObject(ret);
    }
    /**
    * @param {Array<any>} objects
    */
    setObjects(objects) {
        wasm.scene_setObjects(this.__wbg_ptr, addHeapObject(objects));
    }
    /**
    * @param {Function} animation_func
    * @param {Uint32Array} object_indices
    * @param {bigint} duration_in_frames
    * @param {Function} rate_func
    * @returns {Promise<void>}
    */
    play(animation_func, object_indices, duration_in_frames, rate_func) {
        const ptr0 = passArray32ToWasm0(object_indices, wasm.__wbindgen_malloc);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.scene_play(this.__wbg_ptr, addHeapObject(animation_func), ptr0, len0, duration_in_frames, addHeapObject(rate_func));
        return takeObject(ret);
    }
    /**
    * @param {Function} animation_func
    * @param {Array<any>} objects
    * @param {number} t
    * @returns {Promise<void>}
    */
    makeFrame(animation_func, objects, t) {
        const ret = wasm.scene_makeFrame(this.__wbg_ptr, addHeapObject(animation_func), addHeapObject(objects), t);
        return takeObject(ret);
    }
    /**
    * @param {bigint} duration_in_frames
    * @returns {Promise<void>}
    */
    wait(duration_in_frames) {
        const ret = wasm.scene_wait(this.__wbg_ptr, duration_in_frames);
        return takeObject(ret);
    }
    /**
    * @param {Function} callback
    */
    setCallback(callback) {
        wasm.scene_setCallback(this.__wbg_ptr, addHeapObject(callback));
    }
    /**
    * @returns {Promise<void>}
    */
    callCallback() {
        const ret = wasm.scene_callCallback(this.__wbg_ptr);
        return takeObject(ret);
    }
}

const WasmCameraFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_wasmcamera_free(ptr >>> 0));
/**
*/
export class WasmCamera {

    static __wrap(ptr) {
        ptr = ptr >>> 0;
        const obj = Object.create(WasmCamera.prototype);
        obj.__wbg_ptr = ptr;
        WasmCameraFinalization.register(obj, obj.__wbg_ptr, obj);
        return obj;
    }

    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        WasmCameraFinalization.unregister(this);
        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_wasmcamera_free(ptr);
    }
    /**
    * @param {Array<any>} position
    * @param {Array<any>} rotation
    * @param {number} focal_distance
    * @param {number} zoom
    * @param {number} width
    * @param {number} height
    */
    constructor(position, rotation, focal_distance, zoom, width, height) {
        const ret = wasm.wasmcamera_new(addHeapObject(position), addHeapObject(rotation), focal_distance, zoom, width, height);
        this.__wbg_ptr = ret >>> 0;
        return this;
    }
    /**
    * @returns {Array<any>}
    */
    getPosition() {
        const ret = wasm.wasmcamera_getPosition(this.__wbg_ptr);
        return takeObject(ret);
    }
    /**
    * @returns {Array<any>}
    */
    getRotation() {
        const ret = wasm.wasmcamera_getRotation(this.__wbg_ptr);
        return takeObject(ret);
    }
    /**
    * @returns {number}
    */
    getFocalDistance() {
        const ret = wasm.wasmcamera_getFocalDistance(this.__wbg_ptr);
        return ret;
    }
    /**
    * @returns {number}
    */
    getZoom() {
        const ret = wasm.wasmcamera_getZoom(this.__wbg_ptr);
        return ret;
    }
    /**
    * @returns {number}
    */
    getWidth() {
        const ret = wasm.wasmcamera_getWidth(this.__wbg_ptr);
        return ret;
    }
    /**
    * @returns {number}
    */
    getHeight() {
        const ret = wasm.wasmcamera_getHeight(this.__wbg_ptr);
        return ret;
    }
    /**
    * @returns {WasmCamera}
    */
    clone() {
        const ret = wasm.wasmcamera_clone(this.__wbg_ptr);
        return WasmCamera.__wrap(ret);
    }
}

const WasmColorFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_wasmcolor_free(ptr >>> 0));
/**
*/
export class WasmColor {

    static __wrap(ptr) {
        ptr = ptr >>> 0;
        const obj = Object.create(WasmColor.prototype);
        obj.__wbg_ptr = ptr;
        WasmColorFinalization.register(obj, obj.__wbg_ptr, obj);
        return obj;
    }

    static __unwrap(jsValue) {
        if (!(jsValue instanceof WasmColor)) {
            return 0;
        }
        return jsValue.__destroy_into_raw();
    }

    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        WasmColorFinalization.unregister(this);
        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_wasmcolor_free(ptr);
    }
    /**
    * @param {number} r
    * @param {number} g
    * @param {number} b
    * @param {number} a
    */
    constructor(r, g, b, a) {
        const ret = wasm.wasmcolor_new(r, g, b, a);
        this.__wbg_ptr = ret >>> 0;
        return this;
    }
    /**
    * @returns {number}
    */
    getR() {
        const ret = wasm.wasmcolor_getR(this.__wbg_ptr);
        return ret;
    }
    /**
    * @returns {number}
    */
    getG() {
        const ret = wasm.wasmcolor_getG(this.__wbg_ptr);
        return ret;
    }
    /**
    * @returns {number}
    */
    getB() {
        const ret = wasm.wasmcolor_getB(this.__wbg_ptr);
        return ret;
    }
    /**
    * @returns {number}
    */
    getA() {
        const ret = wasm.wasmcolor_getA(this.__wbg_ptr);
        return ret;
    }
}

const WasmGradientImageOrColorFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_wasmgradientimageorcolor_free(ptr >>> 0));
/**
*/
export class WasmGradientImageOrColor {

    static __wrap(ptr) {
        ptr = ptr >>> 0;
        const obj = Object.create(WasmGradientImageOrColor.prototype);
        obj.__wbg_ptr = ptr;
        WasmGradientImageOrColorFinalization.register(obj, obj.__wbg_ptr, obj);
        return obj;
    }

    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        WasmGradientImageOrColorFinalization.unregister(this);
        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_wasmgradientimageorcolor_free(ptr);
    }
    /**
    * @param {WasmColor} color
    * @returns {WasmGradientImageOrColor}
    */
    static fromColor(color) {
        _assertClass(color, WasmColor);
        var ptr0 = color.__destroy_into_raw();
        const ret = wasm.wasmgradientimageorcolor_fromColor(ptr0);
        return WasmGradientImageOrColor.__wrap(ret);
    }
    /**
    * @param {WasmLinearGradient} linear_gradient
    * @returns {WasmGradientImageOrColor}
    */
    static fromLinearGradient(linear_gradient) {
        _assertClass(linear_gradient, WasmLinearGradient);
        var ptr0 = linear_gradient.__destroy_into_raw();
        const ret = wasm.wasmgradientimageorcolor_fromLinearGradient(ptr0);
        return WasmGradientImageOrColor.__wrap(ret);
    }
    /**
    * @param {WasmRadialGradient} radial_gradient
    * @returns {WasmGradientImageOrColor}
    */
    static fromRadialGradient(radial_gradient) {
        _assertClass(radial_gradient, WasmRadialGradient);
        var ptr0 = radial_gradient.__destroy_into_raw();
        const ret = wasm.wasmgradientimageorcolor_fromRadialGradient(ptr0);
        return WasmGradientImageOrColor.__wrap(ret);
    }
    /**
    * @param {WasmImage} image
    * @returns {WasmGradientImageOrColor}
    */
    static fromImage(image) {
        _assertClass(image, WasmImage);
        var ptr0 = image.__destroy_into_raw();
        const ret = wasm.wasmgradientimageorcolor_fromImage(ptr0);
        return WasmGradientImageOrColor.__wrap(ret);
    }
    /**
    * @returns {boolean}
    */
    isColor() {
        const ret = wasm.wasmgradientimageorcolor_isColor(this.__wbg_ptr);
        return ret !== 0;
    }
    /**
    * @returns {boolean}
    */
    isLinearGradient() {
        const ret = wasm.wasmgradientimageorcolor_isLinearGradient(this.__wbg_ptr);
        return ret !== 0;
    }
    /**
    * @returns {boolean}
    */
    isRadialGradient() {
        const ret = wasm.wasmgradientimageorcolor_isRadialGradient(this.__wbg_ptr);
        return ret !== 0;
    }
    /**
    * @returns {boolean}
    */
    isImage() {
        const ret = wasm.wasmgradientimageorcolor_isImage(this.__wbg_ptr);
        return ret !== 0;
    }
    /**
    * @returns {WasmColor | undefined}
    */
    getColor() {
        const ret = wasm.wasmgradientimageorcolor_getColor(this.__wbg_ptr);
        return ret === 0 ? undefined : WasmColor.__wrap(ret);
    }
    /**
    * @returns {WasmLinearGradient | undefined}
    */
    getLinearGradient() {
        const ret = wasm.wasmgradientimageorcolor_getLinearGradient(this.__wbg_ptr);
        return ret === 0 ? undefined : WasmLinearGradient.__wrap(ret);
    }
    /**
    * @returns {WasmRadialGradient | undefined}
    */
    getRadialGradient() {
        const ret = wasm.wasmgradientimageorcolor_getRadialGradient(this.__wbg_ptr);
        return ret === 0 ? undefined : WasmRadialGradient.__wrap(ret);
    }
    /**
    * @returns {WasmImage | undefined}
    */
    getImage() {
        const ret = wasm.wasmgradientimageorcolor_getImage(this.__wbg_ptr);
        return ret === 0 ? undefined : WasmImage.__wrap(ret);
    }
    /**
    * @returns {WasmGradientImageOrColor}
    */
    clone() {
        const ret = wasm.wasmgradientimageorcolor_clone(this.__wbg_ptr);
        return WasmGradientImageOrColor.__wrap(ret);
    }
}

const WasmGradientStopFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_wasmgradientstop_free(ptr >>> 0));
/**
*/
export class WasmGradientStop {

    static __wrap(ptr) {
        ptr = ptr >>> 0;
        const obj = Object.create(WasmGradientStop.prototype);
        obj.__wbg_ptr = ptr;
        WasmGradientStopFinalization.register(obj, obj.__wbg_ptr, obj);
        return obj;
    }

    static __unwrap(jsValue) {
        if (!(jsValue instanceof WasmGradientStop)) {
            return 0;
        }
        return jsValue.__destroy_into_raw();
    }

    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        WasmGradientStopFinalization.unregister(this);
        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_wasmgradientstop_free(ptr);
    }
    /**
    * @param {number} offset
    * @param {WasmColor} color
    */
    constructor(offset, color) {
        _assertClass(color, WasmColor);
        var ptr0 = color.__destroy_into_raw();
        const ret = wasm.wasmgradientstop_new(offset, ptr0);
        this.__wbg_ptr = ret >>> 0;
        return this;
    }
    /**
    * @returns {number}
    */
    getOffset() {
        const ret = wasm.wasmgradientstop_getOffset(this.__wbg_ptr);
        return ret;
    }
    /**
    * @returns {WasmColor}
    */
    getColor() {
        const ret = wasm.wasmgradientstop_getColor(this.__wbg_ptr);
        return WasmColor.__wrap(ret);
    }
}

const WasmImageFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_wasmimage_free(ptr >>> 0));
/**
*/
export class WasmImage {

    static __wrap(ptr) {
        ptr = ptr >>> 0;
        const obj = Object.create(WasmImage.prototype);
        obj.__wbg_ptr = ptr;
        WasmImageFinalization.register(obj, obj.__wbg_ptr, obj);
        return obj;
    }

    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        WasmImageFinalization.unregister(this);
        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_wasmimage_free(ptr);
    }
    /**
    * @param {string} image_base64
    * @param {string} mime_type
    * @param {number} top
    * @param {number} left
    * @param {number} bottom
    * @param {number} right
    * @param {number} alpha
    */
    constructor(image_base64, mime_type, top, left, bottom, right, alpha) {
        const ptr0 = passStringToWasm0(image_base64, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        const ptr1 = passStringToWasm0(mime_type, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len1 = WASM_VECTOR_LEN;
        const ret = wasm.wasmimage_new(ptr0, len0, ptr1, len1, top, left, bottom, right, alpha);
        this.__wbg_ptr = ret >>> 0;
        return this;
    }
    /**
    * @returns {string}
    */
    getImageBase64() {
        let deferred1_0;
        let deferred1_1;
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.wasmimage_getImageBase64(retptr, this.__wbg_ptr);
            var r0 = getInt32Memory0()[retptr / 4 + 0];
            var r1 = getInt32Memory0()[retptr / 4 + 1];
            deferred1_0 = r0;
            deferred1_1 = r1;
            return getStringFromWasm0(r0, r1);
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
            wasm.__wbindgen_free(deferred1_0, deferred1_1, 1);
        }
    }
    /**
    * @returns {string}
    */
    getMimeType() {
        let deferred1_0;
        let deferred1_1;
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.wasmimage_getMimeType(retptr, this.__wbg_ptr);
            var r0 = getInt32Memory0()[retptr / 4 + 0];
            var r1 = getInt32Memory0()[retptr / 4 + 1];
            deferred1_0 = r0;
            deferred1_1 = r1;
            return getStringFromWasm0(r0, r1);
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
            wasm.__wbindgen_free(deferred1_0, deferred1_1, 1);
        }
    }
    /**
    * @returns {number}
    */
    getTop() {
        const ret = wasm.wasmcolor_getG(this.__wbg_ptr);
        return ret;
    }
    /**
    * @returns {number}
    */
    getLeft() {
        const ret = wasm.wasmcolor_getR(this.__wbg_ptr);
        return ret;
    }
    /**
    * @returns {number}
    */
    getBottom() {
        const ret = wasm.wasmcolor_getA(this.__wbg_ptr);
        return ret;
    }
    /**
    * @returns {number}
    */
    getRight() {
        const ret = wasm.wasmcolor_getB(this.__wbg_ptr);
        return ret;
    }
    /**
    * @returns {number}
    */
    getAlpha() {
        const ret = wasm.wasmgradientstop_getOffset(this.__wbg_ptr);
        return ret;
    }
}

const WasmLightSourceFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_wasmlightsource_free(ptr >>> 0));
/**
*/
export class WasmLightSource {

    static __wrap(ptr) {
        ptr = ptr >>> 0;
        const obj = Object.create(WasmLightSource.prototype);
        obj.__wbg_ptr = ptr;
        WasmLightSourceFinalization.register(obj, obj.__wbg_ptr, obj);
        return obj;
    }

    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        WasmLightSourceFinalization.unregister(this);
        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_wasmlightsource_free(ptr);
    }
    /**
    * @param {Array<any>} position
    */
    constructor(position) {
        const ret = wasm.wasmlightsource_new(addHeapObject(position));
        this.__wbg_ptr = ret >>> 0;
        return this;
    }
    /**
    * @returns {Array<any>}
    */
    getPosition() {
        const ret = wasm.wasmlightsource_getPosition(this.__wbg_ptr);
        return takeObject(ret);
    }
    /**
    * @returns {WasmLightSource}
    */
    clone() {
        const ret = wasm.wasmlightsource_clone(this.__wbg_ptr);
        return WasmLightSource.__wrap(ret);
    }
}

const WasmLinearGradientFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_wasmlineargradient_free(ptr >>> 0));
/**
*/
export class WasmLinearGradient {

    static __wrap(ptr) {
        ptr = ptr >>> 0;
        const obj = Object.create(WasmLinearGradient.prototype);
        obj.__wbg_ptr = ptr;
        WasmLinearGradientFinalization.register(obj, obj.__wbg_ptr, obj);
        return obj;
    }

    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        WasmLinearGradientFinalization.unregister(this);
        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_wasmlineargradient_free(ptr);
    }
    /**
    * @param {number} x1
    * @param {number} y1
    * @param {number} x2
    * @param {number} y2
    * @param {(WasmGradientStop)[]} stops
    * @param {number} alpha
    */
    constructor(x1, y1, x2, y2, stops, alpha) {
        const ptr0 = passArrayJsValueToWasm0(stops, wasm.__wbindgen_malloc);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.wasmlineargradient_new(x1, y1, x2, y2, ptr0, len0, alpha);
        this.__wbg_ptr = ret >>> 0;
        return this;
    }
    /**
    * @returns {number}
    */
    getX1() {
        const ret = wasm.wasmcolor_getR(this.__wbg_ptr);
        return ret;
    }
    /**
    * @returns {number}
    */
    getY1() {
        const ret = wasm.wasmcolor_getG(this.__wbg_ptr);
        return ret;
    }
    /**
    * @returns {number}
    */
    getX2() {
        const ret = wasm.wasmcolor_getB(this.__wbg_ptr);
        return ret;
    }
    /**
    * @returns {number}
    */
    getY2() {
        const ret = wasm.wasmcolor_getA(this.__wbg_ptr);
        return ret;
    }
    /**
    * @returns {(WasmGradientStop)[]}
    */
    getStops() {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.wasmlineargradient_getStops(retptr, this.__wbg_ptr);
            var r0 = getInt32Memory0()[retptr / 4 + 0];
            var r1 = getInt32Memory0()[retptr / 4 + 1];
            var v1 = getArrayJsValueFromWasm0(r0, r1).slice();
            wasm.__wbindgen_free(r0, r1 * 4, 4);
            return v1;
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
    * @returns {number}
    */
    getAlpha() {
        const ret = wasm.wasmgradientstop_getOffset(this.__wbg_ptr);
        return ret;
    }
}

const WasmRadialGradientFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_wasmradialgradient_free(ptr >>> 0));
/**
*/
export class WasmRadialGradient {

    static __wrap(ptr) {
        ptr = ptr >>> 0;
        const obj = Object.create(WasmRadialGradient.prototype);
        obj.__wbg_ptr = ptr;
        WasmRadialGradientFinalization.register(obj, obj.__wbg_ptr, obj);
        return obj;
    }

    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        WasmRadialGradientFinalization.unregister(this);
        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_wasmradialgradient_free(ptr);
    }
    /**
    * @param {number} cx
    * @param {number} cy
    * @param {number} r
    * @param {number} fx
    * @param {number} fy
    * @param {(WasmGradientStop)[]} stops
    * @param {number} alpha
    */
    constructor(cx, cy, r, fx, fy, stops, alpha) {
        const ptr0 = passArrayJsValueToWasm0(stops, wasm.__wbindgen_malloc);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.wasmradialgradient_new(cx, cy, r, fx, fy, ptr0, len0, alpha);
        this.__wbg_ptr = ret >>> 0;
        return this;
    }
    /**
    * @returns {number}
    */
    getCx() {
        const ret = wasm.wasmcolor_getR(this.__wbg_ptr);
        return ret;
    }
    /**
    * @returns {number}
    */
    getCy() {
        const ret = wasm.wasmcolor_getG(this.__wbg_ptr);
        return ret;
    }
    /**
    * @returns {number}
    */
    getR() {
        const ret = wasm.wasmcolor_getB(this.__wbg_ptr);
        return ret;
    }
    /**
    * @returns {number}
    */
    getFx() {
        const ret = wasm.wasmcolor_getA(this.__wbg_ptr);
        return ret;
    }
    /**
    * @returns {number}
    */
    getFy() {
        const ret = wasm.wasmgradientstop_getOffset(this.__wbg_ptr);
        return ret;
    }
    /**
    * @returns {(WasmGradientStop)[]}
    */
    getStops() {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.wasmradialgradient_getStops(retptr, this.__wbg_ptr);
            var r0 = getInt32Memory0()[retptr / 4 + 0];
            var r1 = getInt32Memory0()[retptr / 4 + 1];
            var v1 = getArrayJsValueFromWasm0(r0, r1).slice();
            wasm.__wbindgen_free(r0, r1 * 4, 4);
            return v1;
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
    * @returns {number}
    */
    getAlpha() {
        const ret = wasm.wasmradialgradient_getAlpha(this.__wbg_ptr);
        return ret;
    }
}

const WasmThreeDObjectFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_wasmthreedobject_free(ptr >>> 0));
/**
*/
export class WasmThreeDObject {

    static __wrap(ptr) {
        ptr = ptr >>> 0;
        const obj = Object.create(WasmThreeDObject.prototype);
        obj.__wbg_ptr = ptr;
        WasmThreeDObjectFinalization.register(obj, obj.__wbg_ptr, obj);
        return obj;
    }

    static __unwrap(jsValue) {
        if (!(jsValue instanceof WasmThreeDObject)) {
            return 0;
        }
        return jsValue.__destroy_into_raw();
    }

    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        WasmThreeDObjectFinalization.unregister(this);
        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_wasmthreedobject_free(ptr);
    }
    /**
    * @param {Array<any>} points
    * @param {(WasmThreeDObject)[]} subobjects
    * @param {WasmGradientImageOrColor} fill
    * @param {WasmGradientImageOrColor} stroke
    * @param {number} stroke_width
    */
    constructor(points, subobjects, fill, stroke, stroke_width) {
        const ptr0 = passArrayJsValueToWasm0(subobjects, wasm.__wbindgen_malloc);
        const len0 = WASM_VECTOR_LEN;
        _assertClass(fill, WasmGradientImageOrColor);
        var ptr1 = fill.__destroy_into_raw();
        _assertClass(stroke, WasmGradientImageOrColor);
        var ptr2 = stroke.__destroy_into_raw();
        const ret = wasm.wasmthreedobject_new(addHeapObject(points), ptr0, len0, ptr1, ptr2, stroke_width);
        this.__wbg_ptr = ret >>> 0;
        return this;
    }
    /**
    * @returns {Array<any>}
    */
    getPoints() {
        const ret = wasm.wasmthreedobject_getPoints(this.__wbg_ptr);
        return takeObject(ret);
    }
    /**
    * @returns {(WasmThreeDObject)[]}
    */
    getSubobjects() {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.wasmthreedobject_getSubobjects(retptr, this.__wbg_ptr);
            var r0 = getInt32Memory0()[retptr / 4 + 0];
            var r1 = getInt32Memory0()[retptr / 4 + 1];
            var v1 = getArrayJsValueFromWasm0(r0, r1).slice();
            wasm.__wbindgen_free(r0, r1 * 4, 4);
            return v1;
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
    * @returns {WasmGradientImageOrColor}
    */
    getFill() {
        const ret = wasm.wasmthreedobject_getFill(this.__wbg_ptr);
        return WasmGradientImageOrColor.__wrap(ret);
    }
    /**
    * @returns {WasmGradientImageOrColor}
    */
    getStroke() {
        const ret = wasm.wasmthreedobject_getStroke(this.__wbg_ptr);
        return WasmGradientImageOrColor.__wrap(ret);
    }
    /**
    * @returns {number}
    */
    getStrokeWidth() {
        const ret = wasm.wasmthreedobject_getStrokeWidth(this.__wbg_ptr);
        return ret;
    }
    /**
    * @param {Array<any>} points
    * @returns {WasmThreeDObject}
    */
    setPoints(points) {
        const ret = wasm.wasmthreedobject_setPoints(this.__wbg_ptr, addHeapObject(points));
        return WasmThreeDObject.__wrap(ret);
    }
    /**
    * @param {(WasmThreeDObject)[]} subobjects
    * @returns {WasmThreeDObject}
    */
    setSubobjects(subobjects) {
        const ptr0 = passArrayJsValueToWasm0(subobjects, wasm.__wbindgen_malloc);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.wasmthreedobject_setSubobjects(this.__wbg_ptr, ptr0, len0);
        return WasmThreeDObject.__wrap(ret);
    }
    /**
    * @param {WasmGradientImageOrColor} fill
    * @returns {WasmThreeDObject}
    */
    setFill(fill) {
        _assertClass(fill, WasmGradientImageOrColor);
        var ptr0 = fill.__destroy_into_raw();
        const ret = wasm.wasmthreedobject_setFill(this.__wbg_ptr, ptr0);
        return WasmThreeDObject.__wrap(ret);
    }
    /**
    * @param {WasmGradientImageOrColor} stroke
    * @returns {WasmThreeDObject}
    */
    setStroke(stroke) {
        _assertClass(stroke, WasmGradientImageOrColor);
        var ptr0 = stroke.__destroy_into_raw();
        const ret = wasm.wasmthreedobject_setStroke(this.__wbg_ptr, ptr0);
        return WasmThreeDObject.__wrap(ret);
    }
    /**
    * @param {number} stroke_width
    * @returns {WasmThreeDObject}
    */
    setStrokeWidth(stroke_width) {
        const ret = wasm.wasmthreedobject_setStrokeWidth(this.__wbg_ptr, stroke_width);
        return WasmThreeDObject.__wrap(ret);
    }
    /**
    * @param {number} factor
    * @param {boolean} recursive
    * @returns {WasmThreeDObject}
    */
    scale(factor, recursive) {
        const ret = wasm.wasmthreedobject_scale(this.__wbg_ptr, factor, recursive);
        return WasmThreeDObject.__wrap(ret);
    }
    /**
    * @param {Array<any>} factor
    * @param {boolean} recursive
    * @returns {WasmThreeDObject}
    */
    stretch(factor, recursive) {
        const ret = wasm.wasmthreedobject_stretch(this.__wbg_ptr, addHeapObject(factor), recursive);
        return WasmThreeDObject.__wrap(ret);
    }
    /**
    * @param {Array<any>} shift
    * @param {boolean} recursive
    * @returns {WasmThreeDObject}
    */
    shift(shift, recursive) {
        const ret = wasm.wasmthreedobject_shift(this.__wbg_ptr, addHeapObject(shift), recursive);
        return WasmThreeDObject.__wrap(ret);
    }
    /**
    * @param {number} angle
    * @param {boolean} recursive
    * @returns {WasmThreeDObject}
    */
    rotateX(angle, recursive) {
        const ret = wasm.wasmthreedobject_rotateX(this.__wbg_ptr, angle, recursive);
        return WasmThreeDObject.__wrap(ret);
    }
    /**
    * @param {number} angle
    * @param {boolean} recursive
    * @returns {WasmThreeDObject}
    */
    rotateY(angle, recursive) {
        const ret = wasm.wasmthreedobject_rotateY(this.__wbg_ptr, angle, recursive);
        return WasmThreeDObject.__wrap(ret);
    }
    /**
    * @param {number} angle
    * @param {boolean} recursive
    * @returns {WasmThreeDObject}
    */
    rotateZ(angle, recursive) {
        const ret = wasm.wasmthreedobject_rotateZ(this.__wbg_ptr, angle, recursive);
        return WasmThreeDObject.__wrap(ret);
    }
    /**
    * @param {WasmCamera} camera
    * @param {WasmLightSource} light_source
    * @returns {WasmVectorObject}
    */
    projectAndShade(camera, light_source) {
        _assertClass(camera, WasmCamera);
        _assertClass(light_source, WasmLightSource);
        const ret = wasm.wasmthreedobject_projectAndShade(this.__wbg_ptr, camera.__wbg_ptr, light_source.__wbg_ptr);
        return WasmVectorObject.__wrap(ret);
    }
    /**
    * @param {Function} uv_function
    * @param {Array<any>} u_range
    * @param {Array<any>} v_range
    * @param {number} u_segments
    * @param {number} v_segments
    * @param {(WasmColor)[]} fills
    * @param {(WasmColor)[]} strokes
    * @param {number} stroke_width
    * @returns {WasmThreeDObject}
    */
    static fromUvFunction(uv_function, u_range, v_range, u_segments, v_segments, fills, strokes, stroke_width) {
        const ptr0 = passArrayJsValueToWasm0(fills, wasm.__wbindgen_malloc);
        const len0 = WASM_VECTOR_LEN;
        const ptr1 = passArrayJsValueToWasm0(strokes, wasm.__wbindgen_malloc);
        const len1 = WASM_VECTOR_LEN;
        const ret = wasm.wasmthreedobject_fromUvFunction(addHeapObject(uv_function), addHeapObject(u_range), addHeapObject(v_range), u_segments, v_segments, ptr0, len0, ptr1, len1, stroke_width);
        return WasmThreeDObject.__wrap(ret);
    }
    /**
    * @returns {Array<any>}
    */
    getBoundingBox() {
        const ret = wasm.wasmthreedobject_getBoundingBox(this.__wbg_ptr);
        return takeObject(ret);
    }
    /**
    * @returns {Array<any>}
    */
    getCenter() {
        const ret = wasm.wasmthreedobject_getCenter(this.__wbg_ptr);
        return takeObject(ret);
    }
    /**
    * @param {Array<any>} point
    * @param {boolean} recursive
    * @returns {WasmThreeDObject}
    */
    moveTo(point, recursive) {
        const ret = wasm.wasmthreedobject_moveTo(this.__wbg_ptr, addHeapObject(point), recursive);
        return WasmThreeDObject.__wrap(ret);
    }
    /**
    * @returns {WasmThreeDObject}
    */
    clone() {
        const ret = wasm.wasmthreedobject_clone(this.__wbg_ptr);
        return WasmThreeDObject.__wrap(ret);
    }
}

const WasmVectorObjectFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_wasmvectorobject_free(ptr >>> 0));
/**
*/
export class WasmVectorObject {

    static __wrap(ptr) {
        ptr = ptr >>> 0;
        const obj = Object.create(WasmVectorObject.prototype);
        obj.__wbg_ptr = ptr;
        WasmVectorObjectFinalization.register(obj, obj.__wbg_ptr, obj);
        return obj;
    }

    static __unwrap(jsValue) {
        if (!(jsValue instanceof WasmVectorObject)) {
            return 0;
        }
        return jsValue.__destroy_into_raw();
    }

    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        WasmVectorObjectFinalization.unregister(this);
        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_wasmvectorobject_free(ptr);
    }
    /**
    */
    constructor() {
        const ret = wasm.wasmvectorobject_new();
        this.__wbg_ptr = ret >>> 0;
        return this;
    }
    /**
    * @returns {number}
    */
    getIndex() {
        const ret = wasm.wasmvectorobject_getIndex(this.__wbg_ptr);
        return ret >>> 0;
    }
    /**
    * @param {number} increment
    * @param {boolean} recursive
    * @returns {WasmVectorObject}
    */
    incrementIndex(increment, recursive) {
        const ret = wasm.wasmvectorobject_incrementIndex(this.__wbg_ptr, increment, recursive);
        return WasmVectorObject.__wrap(ret);
    }
    /**
    * @returns {Array<any>}
    */
    getPoints() {
        const ret = wasm.wasmvectorobject_getPoints(this.__wbg_ptr);
        return takeObject(ret);
    }
    /**
    * @returns {WasmGradientImageOrColor}
    */
    getFill() {
        const ret = wasm.wasmvectorobject_getFill(this.__wbg_ptr);
        return WasmGradientImageOrColor.__wrap(ret);
    }
    /**
    * @returns {WasmGradientImageOrColor}
    */
    getStroke() {
        const ret = wasm.wasmvectorobject_getStroke(this.__wbg_ptr);
        return WasmGradientImageOrColor.__wrap(ret);
    }
    /**
    * @returns {number}
    */
    getStrokeWidth() {
        const ret = wasm.wasmvectorobject_getStrokeWidth(this.__wbg_ptr);
        return ret;
    }
    /**
    * @returns {string}
    */
    getLineCap() {
        let deferred1_0;
        let deferred1_1;
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.wasmvectorobject_getLineCap(retptr, this.__wbg_ptr);
            var r0 = getInt32Memory0()[retptr / 4 + 0];
            var r1 = getInt32Memory0()[retptr / 4 + 1];
            deferred1_0 = r0;
            deferred1_1 = r1;
            return getStringFromWasm0(r0, r1);
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
            wasm.__wbindgen_free(deferred1_0, deferred1_1, 1);
        }
    }
    /**
    * @returns {string}
    */
    getLineJoin() {
        let deferred1_0;
        let deferred1_1;
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.wasmvectorobject_getLineJoin(retptr, this.__wbg_ptr);
            var r0 = getInt32Memory0()[retptr / 4 + 0];
            var r1 = getInt32Memory0()[retptr / 4 + 1];
            deferred1_0 = r0;
            deferred1_1 = r1;
            return getStringFromWasm0(r0, r1);
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
            wasm.__wbindgen_free(deferred1_0, deferred1_1, 1);
        }
    }
    /**
    * @param {number} start
    * @param {number} end
    * @param {boolean} recursive
    * @returns {WasmVectorObject}
    */
    getPartialCopy(start, end, recursive) {
        const ret = wasm.wasmvectorobject_getPartialCopy(this.__wbg_ptr, start, end, recursive);
        return WasmVectorObject.__wrap(ret);
    }
    /**
    * @returns {Array<any>}
    */
    getSubpaths() {
        const ret = wasm.wasmvectorobject_getSubpaths(this.__wbg_ptr);
        return takeObject(ret);
    }
    /**
    * @returns {Array<any>}
    */
    getCubicBezierTuples() {
        const ret = wasm.wasmvectorobject_getCubicBezierTuples(this.__wbg_ptr);
        return takeObject(ret);
    }
    /**
    * @returns {(WasmVectorObject)[]}
    */
    getSubobjects() {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.wasmvectorobject_getSubobjects(retptr, this.__wbg_ptr);
            var r0 = getInt32Memory0()[retptr / 4 + 0];
            var r1 = getInt32Memory0()[retptr / 4 + 1];
            var v1 = getArrayJsValueFromWasm0(r0, r1).slice();
            wasm.__wbindgen_free(r0, r1 * 4, 4);
            return v1;
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
    * @param {number} factor
    * @param {boolean} recursive
    * @returns {WasmVectorObject}
    */
    scale(factor, recursive) {
        const ret = wasm.wasmvectorobject_scale(this.__wbg_ptr, factor, recursive);
        return WasmVectorObject.__wrap(ret);
    }
    /**
    * @param {number} x_factor
    * @param {number} y_factor
    * @param {boolean} recursive
    * @returns {WasmVectorObject}
    */
    stretch(x_factor, y_factor, recursive) {
        const ret = wasm.wasmvectorobject_stretch(this.__wbg_ptr, x_factor, y_factor, recursive);
        return WasmVectorObject.__wrap(ret);
    }
    /**
    * @param {number} x_shift
    * @param {number} y_shift
    * @param {boolean} recursive
    * @returns {WasmVectorObject}
    */
    shift(x_shift, y_shift, recursive) {
        const ret = wasm.wasmvectorobject_shift(this.__wbg_ptr, x_shift, y_shift, recursive);
        return WasmVectorObject.__wrap(ret);
    }
    /**
    * @returns {Array<any>}
    */
    mergedPoints() {
        const ret = wasm.wasmvectorobject_mergedPoints(this.__wbg_ptr);
        return takeObject(ret);
    }
    /**
    * @returns {Array<any>}
    */
    getBoundingBox() {
        const ret = wasm.wasmvectorobject_getBoundingBox(this.__wbg_ptr);
        return takeObject(ret);
    }
    /**
    * @returns {Array<any>}
    */
    getCenter() {
        const ret = wasm.wasmvectorobject_getCenter(this.__wbg_ptr);
        return takeObject(ret);
    }
    /**
    * @returns {Array<any>}
    */
    getCenterOfMass() {
        const ret = wasm.wasmvectorobject_getCenterOfMass(this.__wbg_ptr);
        return takeObject(ret);
    }
    /**
    * @returns {number}
    */
    getHeight() {
        const ret = wasm.wasmvectorobject_getHeight(this.__wbg_ptr);
        return ret;
    }
    /**
    * @returns {number}
    */
    getWidth() {
        const ret = wasm.wasmvectorobject_getWidth(this.__wbg_ptr);
        return ret;
    }
    /**
    * @param {number} index
    * @returns {WasmVectorObject}
    */
    setIndex(index) {
        const ret = wasm.wasmvectorobject_setIndex(this.__wbg_ptr, index);
        return WasmVectorObject.__wrap(ret);
    }
    /**
    * @param {WasmGradientImageOrColor} fill
    * @param {boolean} recursive
    * @returns {WasmVectorObject}
    */
    setFill(fill, recursive) {
        _assertClass(fill, WasmGradientImageOrColor);
        var ptr0 = fill.__destroy_into_raw();
        const ret = wasm.wasmvectorobject_setFill(this.__wbg_ptr, ptr0, recursive);
        return WasmVectorObject.__wrap(ret);
    }
    /**
    * @param {number} opacity
    * @param {boolean} recursive
    * @returns {WasmVectorObject}
    */
    setFillOpacity(opacity, recursive) {
        const ret = wasm.wasmvectorobject_setFillOpacity(this.__wbg_ptr, opacity, recursive);
        return WasmVectorObject.__wrap(ret);
    }
    /**
    * @param {number} x
    * @param {number} y
    * @param {boolean} recursive
    * @returns {WasmVectorObject}
    */
    moveTo(x, y, recursive) {
        const ret = wasm.wasmvectorobject_moveTo(this.__wbg_ptr, x, y, recursive);
        return WasmVectorObject.__wrap(ret);
    }
    /**
    * @param {WasmGradientImageOrColor} stroke
    * @param {boolean} recursive
    * @returns {WasmVectorObject}
    */
    setStroke(stroke, recursive) {
        _assertClass(stroke, WasmGradientImageOrColor);
        var ptr0 = stroke.__destroy_into_raw();
        const ret = wasm.wasmvectorobject_setStroke(this.__wbg_ptr, ptr0, recursive);
        return WasmVectorObject.__wrap(ret);
    }
    /**
    * @param {number} opacity
    * @param {boolean} recursive
    * @returns {WasmVectorObject}
    */
    setStrokeOpacity(opacity, recursive) {
        const ret = wasm.wasmvectorobject_setStrokeOpacity(this.__wbg_ptr, opacity, recursive);
        return WasmVectorObject.__wrap(ret);
    }
    /**
    * @param {number} width
    * @param {boolean} recursive
    * @returns {WasmVectorObject}
    */
    setStrokeWidth(width, recursive) {
        const ret = wasm.wasmvectorobject_setStrokeWidth(this.__wbg_ptr, width, recursive);
        return WasmVectorObject.__wrap(ret);
    }
    /**
    * @param {string} line_cap
    * @param {boolean} recursive
    * @returns {WasmVectorObject}
    */
    setLineCap(line_cap, recursive) {
        const ptr0 = passStringToWasm0(line_cap, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.wasmvectorobject_setLineCap(this.__wbg_ptr, ptr0, len0, recursive);
        return WasmVectorObject.__wrap(ret);
    }
    /**
    * @param {string} line_join
    * @param {boolean} recursive
    * @returns {WasmVectorObject}
    */
    setLineJoin(line_join, recursive) {
        const ptr0 = passStringToWasm0(line_join, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.wasmvectorobject_setLineJoin(this.__wbg_ptr, ptr0, len0, recursive);
        return WasmVectorObject.__wrap(ret);
    }
    /**
    * @param {Array<any>} points
    * @returns {WasmVectorObject}
    */
    setPoints(points) {
        const ret = wasm.wasmvectorobject_setPoints(this.__wbg_ptr, addHeapObject(points));
        return WasmVectorObject.__wrap(ret);
    }
    /**
    * @param {(WasmVectorObject)[]} subobjects
    * @returns {WasmVectorObject}
    */
    setSubobjects(subobjects) {
        const ptr0 = passArrayJsValueToWasm0(subobjects, wasm.__wbindgen_malloc);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.wasmvectorobject_setSubobjects(this.__wbg_ptr, ptr0, len0);
        return WasmVectorObject.__wrap(ret);
    }
    /**
    * @param {number} angle
    * @param {boolean} recursive
    * @returns {WasmVectorObject}
    */
    rotate(angle, recursive) {
        const ret = wasm.wasmvectorobject_rotate(this.__wbg_ptr, angle, recursive);
        return WasmVectorObject.__wrap(ret);
    }
    /**
    * @param {number} key_x
    * @param {number} key_y
    * @returns {Array<any>}
    */
    getCriticalPoint(key_x, key_y) {
        const ret = wasm.wasmvectorobject_getCriticalPoint(this.__wbg_ptr, key_x, key_y);
        return takeObject(ret);
    }
    /**
    * @returns {number}
    */
    getFillOpacity() {
        const ret = wasm.wasmvectorobject_getFillOpacity(this.__wbg_ptr);
        return ret;
    }
    /**
    * @returns {number}
    */
    getStrokeOpacity() {
        const ret = wasm.wasmvectorobject_getStrokeOpacity(this.__wbg_ptr);
        return ret;
    }
    /**
    * @param {WasmVectorObject} other
    * @param {Array<any>} direction
    * @param {number} buff
    * @param {Array<any>} aligned_edge
    * @param {boolean} recursive
    * @returns {WasmVectorObject}
    */
    nextToOther(other, direction, buff, aligned_edge, recursive) {
        _assertClass(other, WasmVectorObject);
        var ptr0 = other.__destroy_into_raw();
        const ret = wasm.wasmvectorobject_nextToOther(this.__wbg_ptr, ptr0, addHeapObject(direction), buff, addHeapObject(aligned_edge), recursive);
        return WasmVectorObject.__wrap(ret);
    }
    /**
    * @param {Array<any>} direction
    * @param {number} buff
    * @param {Array<any>} aligned_edge
    * @param {boolean} recursive
    * @returns {WasmVectorObject}
    */
    arrangeSubobjects(direction, buff, aligned_edge, recursive) {
        const ret = wasm.wasmvectorobject_arrangeSubobjects(this.__wbg_ptr, addHeapObject(direction), buff, addHeapObject(aligned_edge), recursive);
        return WasmVectorObject.__wrap(ret);
    }
    /**
    * @param {Array<any>} point
    * @param {Array<any>} direction
    * @param {number} buff
    * @param {Array<any>} aligned_edge
    * @param {boolean} recursive
    * @returns {WasmVectorObject}
    */
    nextToPoint(point, direction, buff, aligned_edge, recursive) {
        const ret = wasm.wasmvectorobject_nextToPoint(this.__wbg_ptr, addHeapObject(point), addHeapObject(direction), buff, addHeapObject(aligned_edge), recursive);
        return WasmVectorObject.__wrap(ret);
    }
    /**
    * @returns {WasmVectorObject}
    */
    clone() {
        const ret = wasm.wasmvectorobject_clone(this.__wbg_ptr);
        return WasmVectorObject.__wrap(ret);
    }
}

async function __wbg_load(module, imports) {
    if (typeof Response === 'function' && module instanceof Response) {
        if (typeof WebAssembly.instantiateStreaming === 'function') {
            try {
                return await WebAssembly.instantiateStreaming(module, imports);

            } catch (e) {
                if (module.headers.get('Content-Type') != 'application/wasm') {
                    console.warn("`WebAssembly.instantiateStreaming` failed because your server does not serve wasm with `application/wasm` MIME type. Falling back to `WebAssembly.instantiate` which is slower. Original error:\n", e);

                } else {
                    throw e;
                }
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
}

function __wbg_get_imports() {
    const imports = {};
    imports.wbg = {};
    imports.wbg.__wbg_wasmgradientstop_new = function(arg0) {
        const ret = WasmGradientStop.__wrap(arg0);
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_wasmthreedobject_new = function(arg0) {
        const ret = WasmThreeDObject.__wrap(arg0);
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_wasmvectorobject_new = function(arg0) {
        const ret = WasmVectorObject.__wrap(arg0);
        return addHeapObject(ret);
    };
    imports.wbg.__wbindgen_object_drop_ref = function(arg0) {
        takeObject(arg0);
    };
    imports.wbg.__wbindgen_number_get = function(arg0, arg1) {
        const obj = getObject(arg1);
        const ret = typeof(obj) === 'number' ? obj : undefined;
        getFloat64Memory0()[arg0 / 8 + 1] = isLikeNone(ret) ? 0 : ret;
        getInt32Memory0()[arg0 / 4 + 0] = !isLikeNone(ret);
    };
    imports.wbg.__wbindgen_number_new = function(arg0) {
        const ret = arg0;
        return addHeapObject(ret);
    };
    imports.wbg.__wbindgen_string_new = function(arg0, arg1) {
        const ret = getStringFromWasm0(arg0, arg1);
        return addHeapObject(ret);
    };
    imports.wbg.__wbindgen_boolean_get = function(arg0) {
        const v = getObject(arg0);
        const ret = typeof(v) === 'boolean' ? (v ? 1 : 0) : 2;
        return ret;
    };
    imports.wbg.__wbindgen_string_get = function(arg0, arg1) {
        const obj = getObject(arg1);
        const ret = typeof(obj) === 'string' ? obj : undefined;
        var ptr1 = isLikeNone(ret) ? 0 : passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        var len1 = WASM_VECTOR_LEN;
        getInt32Memory0()[arg0 / 4 + 1] = len1;
        getInt32Memory0()[arg0 / 4 + 0] = ptr1;
    };
    imports.wbg.__wbindgen_is_function = function(arg0) {
        const ret = typeof(getObject(arg0)) === 'function';
        return ret;
    };
    imports.wbg.__wbg_wasmcolor_unwrap = function(arg0) {
        const ret = WasmColor.__unwrap(takeObject(arg0));
        return ret;
    };
    imports.wbg.__wbg_wasmgradientstop_unwrap = function(arg0) {
        const ret = WasmGradientStop.__unwrap(takeObject(arg0));
        return ret;
    };
    imports.wbg.__wbg_wasmvectorobject_unwrap = function(arg0) {
        const ret = WasmVectorObject.__unwrap(takeObject(arg0));
        return ret;
    };
    imports.wbg.__wbg_log_cc851dd35ef4eecf = function(arg0, arg1) {
        console.log(getStringFromWasm0(arg0, arg1));
    };
    imports.wbg.__wbindgen_object_clone_ref = function(arg0) {
        const ret = getObject(arg0);
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_wasmthreedobject_unwrap = function(arg0) {
        const ret = WasmThreeDObject.__unwrap(takeObject(arg0));
        return ret;
    };
    imports.wbg.__wbindgen_cb_drop = function(arg0) {
        const obj = takeObject(arg0).original;
        if (obj.cnt-- == 1) {
            obj.a = 0;
            return true;
        }
        const ret = false;
        return ret;
    };
    imports.wbg.__wbindgen_error_new = function(arg0, arg1) {
        const ret = new Error(getStringFromWasm0(arg0, arg1));
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_error_716e8f1278ad61d6 = function(arg0) {
        console.error(takeObject(arg0));
    };
    imports.wbg.__wbg_instanceof_Window_f401953a2cf86220 = function(arg0) {
        let result;
        try {
            result = getObject(arg0) instanceof Window;
        } catch (_) {
            result = false;
        }
        const ret = result;
        return ret;
    };
    imports.wbg.__wbg_document_5100775d18896c16 = function(arg0) {
        const ret = getObject(arg0).document;
        return isLikeNone(ret) ? 0 : addHeapObject(ret);
    };
    imports.wbg.__wbg_setTimeout_c172d5704ef82276 = function() { return handleError(function (arg0, arg1, arg2) {
        const ret = getObject(arg0).setTimeout(getObject(arg1), arg2);
        return ret;
    }, arguments) };
    imports.wbg.__wbg_createElement_8bae7856a4bb7411 = function() { return handleError(function (arg0, arg1, arg2) {
        const ret = getObject(arg0).createElement(getStringFromWasm0(arg1, arg2));
        return addHeapObject(ret);
    }, arguments) };
    imports.wbg.__wbg_createElementNS_556a62fb298be5a2 = function() { return handleError(function (arg0, arg1, arg2, arg3, arg4) {
        const ret = getObject(arg0).createElementNS(arg1 === 0 ? undefined : getStringFromWasm0(arg1, arg2), getStringFromWasm0(arg3, arg4));
        return addHeapObject(ret);
    }, arguments) };
    imports.wbg.__wbg_setinnerHTML_26d69b59e1af99c7 = function(arg0, arg1, arg2) {
        getObject(arg0).innerHTML = getStringFromWasm0(arg1, arg2);
    };
    imports.wbg.__wbg_outerHTML_e073aa84e7bc1eaf = function(arg0, arg1) {
        const ret = getObject(arg1).outerHTML;
        const ptr1 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len1 = WASM_VECTOR_LEN;
        getInt32Memory0()[arg0 / 4 + 1] = len1;
        getInt32Memory0()[arg0 / 4 + 0] = ptr1;
    };
    imports.wbg.__wbg_setAttribute_3c9f6c303b696daa = function() { return handleError(function (arg0, arg1, arg2, arg3, arg4) {
        getObject(arg0).setAttribute(getStringFromWasm0(arg1, arg2), getStringFromWasm0(arg3, arg4));
    }, arguments) };
    imports.wbg.__wbg_instanceof_CanvasRenderingContext2d_20bf99ccc051643b = function(arg0) {
        let result;
        try {
            result = getObject(arg0) instanceof CanvasRenderingContext2D;
        } catch (_) {
            result = false;
        }
        const ret = result;
        return ret;
    };
    imports.wbg.__wbg_setglobalAlpha_d73578e4c446b8b4 = function(arg0, arg1) {
        getObject(arg0).globalAlpha = arg1;
    };
    imports.wbg.__wbg_setstrokeStyle_c79ba6bc36a7f302 = function(arg0, arg1) {
        getObject(arg0).strokeStyle = getObject(arg1);
    };
    imports.wbg.__wbg_setfillStyle_4de94b275f5761f2 = function(arg0, arg1) {
        getObject(arg0).fillStyle = getObject(arg1);
    };
    imports.wbg.__wbg_setlineWidth_ea4c8cb72d8cdc31 = function(arg0, arg1) {
        getObject(arg0).lineWidth = arg1;
    };
    imports.wbg.__wbg_setlineCap_561c8efd4e48949c = function(arg0, arg1, arg2) {
        getObject(arg0).lineCap = getStringFromWasm0(arg1, arg2);
    };
    imports.wbg.__wbg_setlineJoin_c2f314b5744d240f = function(arg0, arg1, arg2) {
        getObject(arg0).lineJoin = getStringFromWasm0(arg1, arg2);
    };
    imports.wbg.__wbg_drawImage_14f72ed9b8430e9d = function() { return handleError(function (arg0, arg1, arg2, arg3, arg4, arg5) {
        getObject(arg0).drawImage(getObject(arg1), arg2, arg3, arg4, arg5);
    }, arguments) };
    imports.wbg.__wbg_beginPath_c7b9e681f2d031ca = function(arg0) {
        getObject(arg0).beginPath();
    };
    imports.wbg.__wbg_fill_7f376d2e52c3054e = function(arg0) {
        getObject(arg0).fill();
    };
    imports.wbg.__wbg_stroke_b125233fc8b11e59 = function(arg0) {
        getObject(arg0).stroke();
    };
    imports.wbg.__wbg_createLinearGradient_c6e8705fffba9558 = function(arg0, arg1, arg2, arg3, arg4) {
        const ret = getObject(arg0).createLinearGradient(arg1, arg2, arg3, arg4);
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_createPattern_f88dd375094c94dc = function() { return handleError(function (arg0, arg1, arg2, arg3) {
        const ret = getObject(arg0).createPattern(getObject(arg1), getStringFromWasm0(arg2, arg3));
        return isLikeNone(ret) ? 0 : addHeapObject(ret);
    }, arguments) };
    imports.wbg.__wbg_createRadialGradient_72dd3cd4393b5c5d = function() { return handleError(function (arg0, arg1, arg2, arg3, arg4, arg5, arg6) {
        const ret = getObject(arg0).createRadialGradient(arg1, arg2, arg3, arg4, arg5, arg6);
        return addHeapObject(ret);
    }, arguments) };
    imports.wbg.__wbg_bezierCurveTo_b022738e9f321e48 = function(arg0, arg1, arg2, arg3, arg4, arg5, arg6) {
        getObject(arg0).bezierCurveTo(arg1, arg2, arg3, arg4, arg5, arg6);
    };
    imports.wbg.__wbg_closePath_1e01ade2e4928be9 = function(arg0) {
        getObject(arg0).closePath();
    };
    imports.wbg.__wbg_moveTo_5526d0fa563650fa = function(arg0, arg1, arg2) {
        getObject(arg0).moveTo(arg1, arg2);
    };
    imports.wbg.__wbg_clearRect_05de681275dda635 = function(arg0, arg1, arg2, arg3, arg4) {
        getObject(arg0).clearRect(arg1, arg2, arg3, arg4);
    };
    imports.wbg.__wbg_fillRect_b5c8166281bac9df = function(arg0, arg1, arg2, arg3, arg4) {
        getObject(arg0).fillRect(arg1, arg2, arg3, arg4);
    };
    imports.wbg.__wbg_resetTransform_69a6c2187d17b61f = function() { return handleError(function (arg0) {
        getObject(arg0).resetTransform();
    }, arguments) };
    imports.wbg.__wbg_scale_9babba91f6f5b5d4 = function() { return handleError(function (arg0, arg1, arg2) {
        getObject(arg0).scale(arg1, arg2);
    }, arguments) };
    imports.wbg.__wbg_translate_2ec050ab1f49f6fc = function() { return handleError(function (arg0, arg1, arg2) {
        getObject(arg0).translate(arg1, arg2);
    }, arguments) };
    imports.wbg.__wbg_addColorStop_9269a253957ed919 = function() { return handleError(function (arg0, arg1, arg2, arg3) {
        getObject(arg0).addColorStop(arg1, getStringFromWasm0(arg2, arg3));
    }, arguments) };
    imports.wbg.__wbg_appendChild_580ccb11a660db68 = function() { return handleError(function (arg0, arg1) {
        const ret = getObject(arg0).appendChild(getObject(arg1));
        return addHeapObject(ret);
    }, arguments) };
    imports.wbg.__wbg_instanceof_HtmlCanvasElement_46bdbf323b0b18d1 = function(arg0) {
        let result;
        try {
            result = getObject(arg0) instanceof HTMLCanvasElement;
        } catch (_) {
            result = false;
        }
        const ret = result;
        return ret;
    };
    imports.wbg.__wbg_setwidth_080107476e633963 = function(arg0, arg1) {
        getObject(arg0).width = arg1 >>> 0;
    };
    imports.wbg.__wbg_setheight_dc240617639f1f51 = function(arg0, arg1) {
        getObject(arg0).height = arg1 >>> 0;
    };
    imports.wbg.__wbg_getContext_df50fa48a8876636 = function() { return handleError(function (arg0, arg1, arg2) {
        const ret = getObject(arg0).getContext(getStringFromWasm0(arg1, arg2));
        return isLikeNone(ret) ? 0 : addHeapObject(ret);
    }, arguments) };
    imports.wbg.__wbg_src_4486cdef354bb1c6 = function(arg0, arg1) {
        const ret = getObject(arg1).src;
        const ptr1 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len1 = WASM_VECTOR_LEN;
        getInt32Memory0()[arg0 / 4 + 1] = len1;
        getInt32Memory0()[arg0 / 4 + 0] = ptr1;
    };
    imports.wbg.__wbg_setsrc_681ceacdf6845f60 = function(arg0, arg1, arg2) {
        getObject(arg0).src = getStringFromWasm0(arg1, arg2);
    };
    imports.wbg.__wbg_new_e6ce9457ca710f38 = function() { return handleError(function () {
        const ret = new Image();
        return addHeapObject(ret);
    }, arguments) };
    imports.wbg.__wbg_queueMicrotask_481971b0d87f3dd4 = function(arg0) {
        queueMicrotask(getObject(arg0));
    };
    imports.wbg.__wbg_queueMicrotask_3cbae2ec6b6cd3d6 = function(arg0) {
        const ret = getObject(arg0).queueMicrotask;
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_crypto_1d1f22824a6a080c = function(arg0) {
        const ret = getObject(arg0).crypto;
        return addHeapObject(ret);
    };
    imports.wbg.__wbindgen_is_object = function(arg0) {
        const val = getObject(arg0);
        const ret = typeof(val) === 'object' && val !== null;
        return ret;
    };
    imports.wbg.__wbg_process_4a72847cc503995b = function(arg0) {
        const ret = getObject(arg0).process;
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_versions_f686565e586dd935 = function(arg0) {
        const ret = getObject(arg0).versions;
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_node_104a2ff8d6ea03a2 = function(arg0) {
        const ret = getObject(arg0).node;
        return addHeapObject(ret);
    };
    imports.wbg.__wbindgen_is_string = function(arg0) {
        const ret = typeof(getObject(arg0)) === 'string';
        return ret;
    };
    imports.wbg.__wbg_require_cca90b1a94a0255b = function() { return handleError(function () {
        const ret = module.require;
        return addHeapObject(ret);
    }, arguments) };
    imports.wbg.__wbg_msCrypto_eb05e62b530a1508 = function(arg0) {
        const ret = getObject(arg0).msCrypto;
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_randomFillSync_5c9c955aa56b6049 = function() { return handleError(function (arg0, arg1) {
        getObject(arg0).randomFillSync(takeObject(arg1));
    }, arguments) };
    imports.wbg.__wbg_getRandomValues_3aa56aa6edec874c = function() { return handleError(function (arg0, arg1) {
        getObject(arg0).getRandomValues(getObject(arg1));
    }, arguments) };
    imports.wbg.__wbg_get_bd8e338fbd5f5cc8 = function(arg0, arg1) {
        const ret = getObject(arg0)[arg1 >>> 0];
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_length_cd7af8117672b8b8 = function(arg0) {
        const ret = getObject(arg0).length;
        return ret;
    };
    imports.wbg.__wbg_new_16b304a2cfa7ff4a = function() {
        const ret = new Array();
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_newnoargs_e258087cd0daa0ea = function(arg0, arg1) {
        const ret = new Function(getStringFromWasm0(arg0, arg1));
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_new_d9bc3a0147634640 = function() {
        const ret = new Map();
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_get_e3c254076557e348 = function() { return handleError(function (arg0, arg1) {
        const ret = Reflect.get(getObject(arg0), getObject(arg1));
        return addHeapObject(ret);
    }, arguments) };
    imports.wbg.__wbg_call_27c0f87801dedf93 = function() { return handleError(function (arg0, arg1) {
        const ret = getObject(arg0).call(getObject(arg1));
        return addHeapObject(ret);
    }, arguments) };
    imports.wbg.__wbg_self_ce0dbfc45cf2f5be = function() { return handleError(function () {
        const ret = self.self;
        return addHeapObject(ret);
    }, arguments) };
    imports.wbg.__wbg_window_c6fb939a7f436783 = function() { return handleError(function () {
        const ret = window.window;
        return addHeapObject(ret);
    }, arguments) };
    imports.wbg.__wbg_globalThis_d1e6af4856ba331b = function() { return handleError(function () {
        const ret = globalThis.globalThis;
        return addHeapObject(ret);
    }, arguments) };
    imports.wbg.__wbg_global_207b558942527489 = function() { return handleError(function () {
        const ret = global.global;
        return addHeapObject(ret);
    }, arguments) };
    imports.wbg.__wbindgen_is_undefined = function(arg0) {
        const ret = getObject(arg0) === undefined;
        return ret;
    };
    imports.wbg.__wbg_eval_020a6ea487e91ede = function() { return handleError(function (arg0, arg1) {
        const ret = eval(getStringFromWasm0(arg0, arg1));
        return addHeapObject(ret);
    }, arguments) };
    imports.wbg.__wbg_from_89e3fc3ba5e6fb48 = function(arg0) {
        const ret = Array.from(getObject(arg0));
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_isArray_2ab64d95e09ea0ae = function(arg0) {
        const ret = Array.isArray(getObject(arg0));
        return ret;
    };
    imports.wbg.__wbg_of_4a2b313a453ec059 = function(arg0) {
        const ret = Array.of(getObject(arg0));
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_of_647f9238b4d5407a = function(arg0, arg1) {
        const ret = Array.of(getObject(arg0), getObject(arg1));
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_of_6a70eed8d41f469c = function(arg0, arg1, arg2) {
        const ret = Array.of(getObject(arg0), getObject(arg1), getObject(arg2));
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_of_40f6b8e691c04867 = function(arg0, arg1, arg2, arg3) {
        const ret = Array.of(getObject(arg0), getObject(arg1), getObject(arg2), getObject(arg3));
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_push_a5b05aedc7234f9f = function(arg0, arg1) {
        const ret = getObject(arg0).push(getObject(arg1));
        return ret;
    };
    imports.wbg.__wbg_call_b3ca7c6051f9bec1 = function() { return handleError(function (arg0, arg1, arg2) {
        const ret = getObject(arg0).call(getObject(arg1), getObject(arg2));
        return addHeapObject(ret);
    }, arguments) };
    imports.wbg.__wbg_call_8e7cb608789c2528 = function() { return handleError(function (arg0, arg1, arg2, arg3) {
        const ret = getObject(arg0).call(getObject(arg1), getObject(arg2), getObject(arg3));
        return addHeapObject(ret);
    }, arguments) };
    imports.wbg.__wbg_set_8417257aaedc936b = function(arg0, arg1, arg2) {
        const ret = getObject(arg0).set(getObject(arg1), getObject(arg2));
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_instanceof_Promise_b438ddea4cacc51f = function(arg0) {
        let result;
        try {
            result = getObject(arg0) instanceof Promise;
        } catch (_) {
            result = false;
        }
        const ret = result;
        return ret;
    };
    imports.wbg.__wbg_new_81740750da40724f = function(arg0, arg1) {
        try {
            var state0 = {a: arg0, b: arg1};
            var cb0 = (arg0, arg1) => {
                const a = state0.a;
                state0.a = 0;
                try {
                    return __wbg_adapter_553(a, state0.b, arg0, arg1);
                } finally {
                    state0.a = a;
                }
            };
            const ret = new Promise(cb0);
            return addHeapObject(ret);
        } finally {
            state0.a = state0.b = 0;
        }
    };
    imports.wbg.__wbg_resolve_b0083a7967828ec8 = function(arg0) {
        const ret = Promise.resolve(getObject(arg0));
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_then_0c86a60e8fcfe9f6 = function(arg0, arg1) {
        const ret = getObject(arg0).then(getObject(arg1));
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_then_a73caa9a87991566 = function(arg0, arg1, arg2) {
        const ret = getObject(arg0).then(getObject(arg1), getObject(arg2));
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_buffer_12d079cc21e14bdb = function(arg0) {
        const ret = getObject(arg0).buffer;
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_newwithbyteoffsetandlength_aa4a17c33a06e5cb = function(arg0, arg1, arg2) {
        const ret = new Uint8Array(getObject(arg0), arg1 >>> 0, arg2 >>> 0);
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_new_63b92bc8671ed464 = function(arg0) {
        const ret = new Uint8Array(getObject(arg0));
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_set_a47bac70306a19a7 = function(arg0, arg1, arg2) {
        getObject(arg0).set(getObject(arg1), arg2 >>> 0);
    };
    imports.wbg.__wbg_newwithlength_e9b4878cebadb3d3 = function(arg0) {
        const ret = new Uint8Array(arg0 >>> 0);
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_subarray_a1f73cd4b5b42fe1 = function(arg0, arg1, arg2) {
        const ret = getObject(arg0).subarray(arg1 >>> 0, arg2 >>> 0);
        return addHeapObject(ret);
    };
    imports.wbg.__wbindgen_debug_string = function(arg0, arg1) {
        const ret = debugString(getObject(arg1));
        const ptr1 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len1 = WASM_VECTOR_LEN;
        getInt32Memory0()[arg0 / 4 + 1] = len1;
        getInt32Memory0()[arg0 / 4 + 0] = ptr1;
    };
    imports.wbg.__wbindgen_throw = function(arg0, arg1) {
        throw new Error(getStringFromWasm0(arg0, arg1));
    };
    imports.wbg.__wbindgen_memory = function() {
        const ret = wasm.memory;
        return addHeapObject(ret);
    };
    imports.wbg.__wbindgen_closure_wrapper855 = function(arg0, arg1, arg2) {
        const ret = makeClosure(arg0, arg1, 148, __wbg_adapter_34);
        return addHeapObject(ret);
    };
    imports.wbg.__wbindgen_closure_wrapper857 = function(arg0, arg1, arg2) {
        const ret = makeClosure(arg0, arg1, 148, __wbg_adapter_37);
        return addHeapObject(ret);
    };
    imports.wbg.__wbindgen_closure_wrapper1612 = function(arg0, arg1, arg2) {
        const ret = makeMutClosure(arg0, arg1, 195, __wbg_adapter_40);
        return addHeapObject(ret);
    };

    return imports;
}

function __wbg_init_memory(imports, maybe_memory) {

}

function __wbg_finalize_init(instance, module) {
    wasm = instance.exports;
    __wbg_init.__wbindgen_wasm_module = module;
    cachedFloat64Memory0 = null;
    cachedInt32Memory0 = null;
    cachedUint32Memory0 = null;
    cachedUint8Memory0 = null;


    return wasm;
}

function initSync(module) {
    if (wasm !== undefined) return wasm;

    const imports = __wbg_get_imports();

    __wbg_init_memory(imports);

    if (!(module instanceof WebAssembly.Module)) {
        module = new WebAssembly.Module(module);
    }

    const instance = new WebAssembly.Instance(module, imports);

    return __wbg_finalize_init(instance, module);
}

async function __wbg_init(input) {
    if (wasm !== undefined) return wasm;

    if (typeof input === 'undefined') {
        input = new URL('mathlikeanim_rs_bg.wasm', import.meta.url);
    }
    const imports = __wbg_get_imports();

    if (typeof input === 'string' || (typeof Request === 'function' && input instanceof Request) || (typeof URL === 'function' && input instanceof URL)) {
        input = fetch(input);
    }

    __wbg_init_memory(imports);

    const { instance, module } = await __wbg_load(await input, imports);

    return __wbg_finalize_init(instance, module);
}

export { initSync }
export default __wbg_init;
