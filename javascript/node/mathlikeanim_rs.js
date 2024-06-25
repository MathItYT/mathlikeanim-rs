let imports = {};
imports['__wbindgen_placeholder__'] = module.exports;
let wasm;
const { Image, createCanvas } = require(`canvas`);
const { spawn } = require(`child_process`);
const { TextDecoder, TextEncoder } = require(`util`);

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

let cachedTextDecoder = new TextDecoder('utf-8', { ignoreBOM: true, fatal: true });

cachedTextDecoder.decode();

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

let cachedTextEncoder = new TextEncoder('utf-8');

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
function __wbg_adapter_28(arg0, arg1) {
    const ret = wasm._dyn_core__ops__function__Fn_____Output___R_as_wasm_bindgen__closure__WasmClosure___describe__invoke__hda56adbe1374f31a(arg0, arg1);
    return takeObject(ret);
}

function __wbg_adapter_31(arg0, arg1, arg2, arg3) {
    const ret = wasm._dyn_core__ops__function__Fn__A_B___Output___R_as_wasm_bindgen__closure__WasmClosure___describe__invoke__hcde03b78e8ae1438(arg0, arg1, addHeapObject(arg2), arg3);
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
function __wbg_adapter_34(arg0, arg1, arg2) {
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
module.exports.addFinalTip = function(shape, tip_side_length, tip_color) {
    _assertClass(shape, WasmVectorObject);
    var ptr0 = shape.__destroy_into_raw();
    _assertClass(tip_color, WasmColor);
    var ptr1 = tip_color.__destroy_into_raw();
    const ret = wasm.addFinalTip(ptr0, tip_side_length, ptr1);
    return WasmVectorObject.__wrap(ret);
};

/**
* @param {WasmVectorObject} shape
* @param {number} tip_side_length
* @param {WasmColor} tip_color
* @returns {WasmVectorObject}
*/
module.exports.addInitialTip = function(shape, tip_side_length, tip_color) {
    _assertClass(shape, WasmVectorObject);
    var ptr0 = shape.__destroy_into_raw();
    _assertClass(tip_color, WasmColor);
    var ptr1 = tip_color.__destroy_into_raw();
    const ret = wasm.addInitialTip(ptr0, tip_side_length, ptr1);
    return WasmVectorObject.__wrap(ret);
};

/**
* @param {WasmVectorObject} shape
* @param {number} tip_side_length
* @param {WasmColor} tip_color
* @returns {WasmVectorObject}
*/
module.exports.addBothSidesTips = function(shape, tip_side_length, tip_color) {
    _assertClass(shape, WasmVectorObject);
    var ptr0 = shape.__destroy_into_raw();
    _assertClass(tip_color, WasmColor);
    var ptr1 = tip_color.__destroy_into_raw();
    const ret = wasm.addBothSidesTips(ptr0, tip_side_length, ptr1);
    return WasmVectorObject.__wrap(ret);
};

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
module.exports.arc = function(center, radius, start_angle, end_angle, num_points, stroke_color, fill_color, stroke_width, line_cap, line_join, index) {
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
};

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
module.exports.circle = function(center, radius, num_points, stroke_color, fill_color, stroke_width, line_cap, line_join, index) {
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
};

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
module.exports.ellipticalArc = function(center, x_radius, y_radius, start_angle, end_angle, num_points, stroke_color, fill_color, stroke_width, line_cap, line_join, index) {
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
};

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
module.exports.ellipse = function(center, x_radius, y_radius, num_points, stroke_color, fill_color, stroke_width, line_cap, line_join, index) {
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
};

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
module.exports.annularSector = function(center, inner_radius, outer_radius, start_angle, end_angle, num_points, stroke_color, fill_color, stroke_width, line_cap, line_join, index) {
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
};

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
module.exports.line = function(start_point, end_point, stroke_color, stroke_width, line_cap, line_join, index) {
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
};

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
module.exports.polygon = function(points, stroke_color, fill_color, stroke_width, line_cap, line_join, index) {
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
};

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
module.exports.regularPolygon = function(center, side_length, num_sides, stroke_color, fill_color, stroke_width, line_cap, line_join, index) {
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
};

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
module.exports.square = function(center, side_length, stroke_color, fill_color, stroke_width, line_cap, line_join, index) {
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
};

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
module.exports.rectangle = function(center, width, height, stroke_color, fill_color, stroke_width, line_cap, line_join, index) {
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
};

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
module.exports.equilateralTriangle = function(center, side_length, stroke_color, fill_color, stroke_width, line_cap, line_join, index) {
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
};

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
module.exports.triangle = function(point1, point2, point3, stroke_color, fill_color, stroke_width, line_cap, line_join, index) {
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
};

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
module.exports.rightTriangle = function(point1, point2, stroke_color, fill_color, stroke_width, line_cap, line_join, index) {
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
};

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
module.exports.axes = function(x_min, x_max, x_step, y_min, y_max, y_step, center, x_length, y_length, color, stroke_width, line_cap, line_join, index, add_x_ticks, add_y_ticks, x_tick_size, y_tick_size, add_x_tip, add_y_tip) {
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
};

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
module.exports.coordsToPoint = function(axes, x, y, x_min, x_max, y_min, y_max) {
    _assertClass(axes, WasmVectorObject);
    const ret = wasm.coordsToPoint(axes.__wbg_ptr, x, y, x_min, x_max, y_min, y_max);
    return takeObject(ret);
};

/**
* @param {WasmVectorObject} axes
* @param {Array<any>} point
* @param {number} x_min
* @param {number} x_max
* @param {number} y_min
* @param {number} y_max
* @returns {Array<any>}
*/
module.exports.pointToCoords = function(axes, point, x_min, x_max, y_min, y_max) {
    _assertClass(axes, WasmVectorObject);
    const ret = wasm.pointToCoords(axes.__wbg_ptr, addHeapObject(point), x_min, x_max, y_min, y_max);
    return takeObject(ret);
};

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
module.exports.parametricPlotInAxes = function(f, t_min, t_max, t_step, axes, x_min, x_max, y_min, y_max, color, stroke_width, line_cap, line_join, index) {
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
};

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
module.exports.plotInAxes = function(f, x_min, x_max, y_min, y_max, x1, x2, x_step, axes, color, stroke_width, line_cap, line_join, index) {
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
};

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
module.exports.contourPlotInAxes = function(f, x_min, x_max, y_min, y_max, x_1, x_2, x_step, y_1, y_2, y_step, axes, color, stroke_width, line_cap, line_join, index, intervals) {
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
};

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
module.exports.areaUnderCurve = function(axes, plot, x_min, x_max, y_min, y_max, x1, x2, color, index) {
    _assertClass(axes, WasmVectorObject);
    _assertClass(plot, WasmVectorObject);
    let ptr0 = 0;
    if (!isLikeNone(color)) {
        _assertClass(color, WasmColor);
        ptr0 = color.__destroy_into_raw();
    }
    const ret = wasm.areaUnderCurve(axes.__wbg_ptr, plot.__wbg_ptr, x_min, x_max, y_min, y_max, x1, x2, ptr0, !isLikeNone(index), isLikeNone(index) ? 0 : index);
    return WasmVectorObject.__wrap(ret);
};

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
module.exports.riemannRectanglesForPlot = function(f, x_min, x_max, y_min, y_max, direction, x_1, x_2, n_rects, axes, stroke_color, fill_color, stroke_width, line_cap, line_join, index) {
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
};

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
module.exports.secantLineForPlot = function(f, x_1, x_2, length, axes, x_min, x_max, y_min, y_max, color, stroke_width, line_cap, line_join, index) {
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
};

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
module.exports.parametricFunction = function(f, t_min, t_max, t_step, color, stroke_width, line_cap, line_join, index) {
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
};

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
module.exports.contourPlot = function(f, x_min, x_max, y_min, y_max, x_step, y_step, color, stroke_width, line_cap, line_join, index, intervals) {
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
};

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
module.exports.realFunction = function(f, x_min, x_max, x_step, color, stroke_width, line_cap, line_join, index) {
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
};

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
module.exports.numberLine = function(x_min, x_max, x_step, color, stroke_width, line_cap, line_join, index, center, length, add_tip, add_ticks, tick_size, angle) {
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
};

/**
* @param {WasmVectorObject} number_line
* @param {number} number
* @param {number} x_min
* @param {number} x_max
* @returns {Array<any>}
*/
module.exports.numberToPoint = function(number_line, number, x_min, x_max) {
    _assertClass(number_line, WasmVectorObject);
    const ret = wasm.numberToPoint(number_line.__wbg_ptr, number, x_min, x_max);
    return takeObject(ret);
};

/**
* @param {WasmVectorObject} number_line
* @param {Array<any>} point
* @param {number} x_min
* @param {number} x_max
* @returns {number}
*/
module.exports.pointToNumber = function(number_line, point, x_min, x_max) {
    _assertClass(number_line, WasmVectorObject);
    const ret = wasm.pointToNumber(number_line.__wbg_ptr, addHeapObject(point), x_min, x_max);
    return ret;
};

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
module.exports.getNumbersTex = function(number_line, numbers, number_to_vector, x_min, x_max, height, direction, buff, index) {
    _assertClass(number_line, WasmVectorObject);
    var ptr0 = number_line.__destroy_into_raw();
    const ret = wasm.getNumbersTex(ptr0, addHeapObject(numbers), addHeapObject(number_to_vector), x_min, x_max, height, isLikeNone(direction) ? 0 : addHeapObject(direction), !isLikeNone(buff), isLikeNone(buff) ? 0 : buff, !isLikeNone(index), isLikeNone(index) ? 0 : index);
    return takeObject(ret);
};

/**
* @param {string} svg
* @returns {WasmVectorObject}
*/
module.exports.svgToVector = function(svg) {
    const ptr0 = passStringToWasm0(svg, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    const len0 = WASM_VECTOR_LEN;
    const ret = wasm.svgToVector(ptr0, len0);
    return WasmVectorObject.__wrap(ret);
};

/**
* @param {number} angle
* @param {number} axis
* @returns {Array<any>}
*/
module.exports.rotMatrix = function(angle, axis) {
    const ret = wasm.rotMatrix(angle, axis);
    return takeObject(ret);
};

/**
* @param {Array<any>} a
* @param {Array<any>} b
* @returns {Array<any>}
*/
module.exports.matrixProduct = function(a, b) {
    const ret = wasm.matrixProduct(addHeapObject(a), addHeapObject(b));
    return takeObject(ret);
};

/**
* @param {number} phi
* @param {number} theta
* @param {number} gamma
* @returns {Array<any>}
*/
module.exports.rotMatrixEuler = function(phi, theta, gamma) {
    const ret = wasm.rotMatrixEuler(phi, theta, gamma);
    return takeObject(ret);
};

/**
* @param {Array<any>} a
* @returns {Array<any>}
*/
module.exports.transposeMatrix = function(a) {
    const ret = wasm.transposeMatrix(addHeapObject(a));
    return takeObject(ret);
};

/**
* @param {Array<any>} matrix
* @param {Array<any>} points
* @returns {Array<any>}
*/
module.exports.applyMatrix = function(matrix, points) {
    const ret = wasm.applyMatrix(addHeapObject(matrix), addHeapObject(points));
    return takeObject(ret);
};

/**
* @param {Array<any>} points
* @param {Array<any>} shift
* @returns {Array<any>}
*/
module.exports.shiftPoints3D = function(points, shift) {
    const ret = wasm.shiftPoints3D(addHeapObject(points), addHeapObject(shift));
    return takeObject(ret);
};

/**
* @param {WasmGradientImageOrColor} color
* @returns {WasmGradientImageOrColor}
*/
module.exports.ensureValidThreeDColor = function(color) {
    _assertClass(color, WasmGradientImageOrColor);
    var ptr0 = color.__destroy_into_raw();
    const ret = wasm.ensureValidThreeDColor(ptr0);
    return WasmGradientImageOrColor.__wrap(ret);
};

/**
* @param {WasmColor} color
* @param {Array<any>} point
* @param {Array<any>} unit_normal
* @param {WasmLightSource} light_source
* @returns {WasmColor}
*/
module.exports.getShadedRgb = function(color, point, unit_normal, light_source) {
    _assertClass(color, WasmColor);
    var ptr0 = color.__destroy_into_raw();
    _assertClass(light_source, WasmLightSource);
    const ret = wasm.getShadedRgb(ptr0, addHeapObject(point), addHeapObject(unit_normal), light_source.__wbg_ptr);
    return WasmColor.__wrap(ret);
};

/**
* @param {Array<any>} points
* @returns {Array<any>}
*/
module.exports.getStartCorner = function(points) {
    const ret = wasm.getStartCorner(addHeapObject(points));
    return takeObject(ret);
};

/**
* @param {Array<any>} points
* @returns {Array<any>}
*/
module.exports.getEndCorner = function(points) {
    const ret = wasm.getEndCorner(addHeapObject(points));
    return takeObject(ret);
};

/**
* @param {Array<any>} a
* @param {Array<any>} b
* @returns {Array<any>}
*/
module.exports.crossProduct = function(a, b) {
    const ret = wasm.crossProduct(addHeapObject(a), addHeapObject(b));
    return takeObject(ret);
};

/**
* @param {Array<any>} v1
* @param {Array<any>} v2
* @returns {Array<any>}
*/
module.exports.getUnitNormal = function(v1, v2) {
    const ret = wasm.getUnitNormal(addHeapObject(v1), addHeapObject(v2));
    return takeObject(ret);
};

/**
* @param {Array<any>} points
* @returns {Array<any>}
*/
module.exports.getStartAnchors = function(points) {
    const ret = wasm.getStartAnchors(addHeapObject(points));
    return takeObject(ret);
};

/**
* @param {Array<any>} points
* @returns {Array<any>}
*/
module.exports.getEndAnchors = function(points) {
    const ret = wasm.getEndAnchors(addHeapObject(points));
    return takeObject(ret);
};

/**
* @param {Array<any>} points
* @returns {Array<any>}
*/
module.exports.getAnchors = function(points) {
    const ret = wasm.getAnchors(addHeapObject(points));
    return takeObject(ret);
};

/**
* @param {Array<any>} points
* @param {number} index
* @returns {Array<any>}
*/
module.exports.getCornerUnitNormal = function(points, index) {
    const ret = wasm.getCornerUnitNormal(addHeapObject(points), index);
    return takeObject(ret);
};

/**
* @param {Array<any>} points
* @returns {Array<any>}
*/
module.exports.getStartCornerUnitNormal = function(points) {
    const ret = wasm.getStartCornerUnitNormal(addHeapObject(points));
    return takeObject(ret);
};

/**
* @param {Array<any>} points
* @returns {Array<any>}
*/
module.exports.getEndCornerUnitNormal = function(points) {
    const ret = wasm.getEndCornerUnitNormal(addHeapObject(points));
    return takeObject(ret);
};

/**
* @param {WasmGradientImageOrColor} color
* @param {Array<any>} points
* @param {WasmLightSource} light_source
* @param {WasmCamera} camera
* @returns {WasmGradientImageOrColor}
*/
module.exports.getShadedColor = function(color, points, light_source, camera) {
    _assertClass(color, WasmGradientImageOrColor);
    _assertClass(light_source, WasmLightSource);
    _assertClass(camera, WasmCamera);
    const ret = wasm.getShadedColor(color.__wbg_ptr, addHeapObject(points), light_source.__wbg_ptr, camera.__wbg_ptr);
    return WasmGradientImageOrColor.__wrap(ret);
};

/**
* @param {Array<any>} points
* @param {WasmCamera} camera
* @returns {Array<any>}
*/
module.exports.projectPoints = function(points, camera) {
    _assertClass(camera, WasmCamera);
    const ret = wasm.projectPoints(addHeapObject(points), camera.__wbg_ptr);
    return takeObject(ret);
};

/**
* @param {Array<any>} point1
* @param {Array<any>} point2
* @returns {Array<any>}
*/
module.exports.lineAsCubicBezier3D = function(point1, point2) {
    const ret = wasm.lineAsCubicBezier3D(addHeapObject(point1), addHeapObject(point2));
    return takeObject(ret);
};

function getArrayF64FromWasm0(ptr, len) {
    ptr = ptr >>> 0;
    return getFloat64Memory0().subarray(ptr / 8, ptr / 8 + len);
}
/**
* @param {number} num_anim_funcs
* @param {number} lag_ratio
* @returns {Float64Array}
*/
module.exports.makeTimings = function(num_anim_funcs, lag_ratio) {
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
};

/**
* @param {WasmVectorObject} vec_obj
* @param {(Function)[]} anim_funcs
* @param {number} lag_ratio
* @param {number} t
* @returns {WasmVectorObject}
*/
module.exports.animationGroup = function(vec_obj, anim_funcs, lag_ratio, t) {
    _assertClass(vec_obj, WasmVectorObject);
    var ptr0 = vec_obj.__destroy_into_raw();
    const ptr1 = passArrayJsValueToWasm0(anim_funcs, wasm.__wbindgen_malloc);
    const len1 = WASM_VECTOR_LEN;
    const ret = wasm.animationGroup(ptr0, ptr1, len1, lag_ratio, t);
    return WasmVectorObject.__wrap(ret);
};

/**
* @param {WasmVectorObject} vec_obj
* @param {number} t
* @returns {WasmVectorObject}
*/
module.exports.create = function(vec_obj, t) {
    _assertClass(vec_obj, WasmVectorObject);
    var ptr0 = vec_obj.__destroy_into_raw();
    const ret = wasm.create(ptr0, t);
    return WasmVectorObject.__wrap(ret);
};

/**
* @param {WasmVectorObject} vec_obj
* @param {number} t
* @returns {WasmVectorObject}
*/
module.exports.drawStrokeThenFill = function(vec_obj, t) {
    _assertClass(vec_obj, WasmVectorObject);
    var ptr0 = vec_obj.__destroy_into_raw();
    const ret = wasm.drawStrokeThenFill(ptr0, t);
    return WasmVectorObject.__wrap(ret);
};

/**
* @param {WasmVectorObject} vec_obj
* @param {number} lag_ratio
* @param {number} t
* @returns {WasmVectorObject}
*/
module.exports.write = function(vec_obj, lag_ratio, t) {
    _assertClass(vec_obj, WasmVectorObject);
    var ptr0 = vec_obj.__destroy_into_raw();
    const ret = wasm.write(ptr0, lag_ratio, t);
    return WasmVectorObject.__wrap(ret);
};

/**
* @param {WasmVectorObject} vec_obj
* @param {number} scale_factor
* @param {Array<any>} shift
* @param {number} t
* @returns {WasmVectorObject}
*/
module.exports.fadeIn = function(vec_obj, scale_factor, shift, t) {
    _assertClass(vec_obj, WasmVectorObject);
    var ptr0 = vec_obj.__destroy_into_raw();
    const ret = wasm.fadeIn(ptr0, scale_factor, addHeapObject(shift), t);
    return WasmVectorObject.__wrap(ret);
};

/**
* @param {WasmVectorObject} vec_obj
* @param {number} scale_factor
* @param {Array<any>} shift
* @param {number} t
* @returns {WasmVectorObject}
*/
module.exports.fadeOut = function(vec_obj, scale_factor, shift, t) {
    _assertClass(vec_obj, WasmVectorObject);
    var ptr0 = vec_obj.__destroy_into_raw();
    const ret = wasm.fadeOut(ptr0, scale_factor, addHeapObject(shift), t);
    return WasmVectorObject.__wrap(ret);
};

/**
* @param {WasmVectorObject} vec_obj
* @param {number} t
* @returns {WasmVectorObject}
*/
module.exports.growArrowWithFinalTip = function(vec_obj, t) {
    _assertClass(vec_obj, WasmVectorObject);
    var ptr0 = vec_obj.__destroy_into_raw();
    const ret = wasm.growArrowWithFinalTip(ptr0, t);
    return WasmVectorObject.__wrap(ret);
};

/**
* @param {WasmVectorObject} vec_obj
* @param {number} t
* @returns {WasmVectorObject}
*/
module.exports.growArrowWithInitialTip = function(vec_obj, t) {
    _assertClass(vec_obj, WasmVectorObject);
    var ptr0 = vec_obj.__destroy_into_raw();
    const ret = wasm.growArrowWithInitialTip(ptr0, t);
    return WasmVectorObject.__wrap(ret);
};

/**
* @param {WasmVectorObject} vec_obj
* @param {number} t
* @returns {WasmVectorObject}
*/
module.exports.growArrowWithTipsAtBothEnds = function(vec_obj, t) {
    _assertClass(vec_obj, WasmVectorObject);
    var ptr0 = vec_obj.__destroy_into_raw();
    const ret = wasm.growArrowWithTipsAtBothEnds(ptr0, t);
    return WasmVectorObject.__wrap(ret);
};

/**
* @param {WasmVectorObject} vec_obj
* @param {number} t
* @returns {WasmVectorObject}
*/
module.exports.growFromCenter = function(vec_obj, t) {
    _assertClass(vec_obj, WasmVectorObject);
    var ptr0 = vec_obj.__destroy_into_raw();
    const ret = wasm.growFromCenter(ptr0, t);
    return WasmVectorObject.__wrap(ret);
};

/**
* @param {WasmVectorObject} original
* @param {WasmVectorObject} target
* @param {number} t
* @returns {WasmVectorObject}
*/
module.exports.morphShape = function(original, target, t) {
    _assertClass(original, WasmVectorObject);
    var ptr0 = original.__destroy_into_raw();
    _assertClass(target, WasmVectorObject);
    var ptr1 = target.__destroy_into_raw();
    const ret = wasm.morphShape(ptr0, ptr1, t);
    return WasmVectorObject.__wrap(ret);
};

/**
* @param {Array<any>} top_left_corner
* @param {Array<any>} bottom_right_corner
* @param {VideoScene} scene
* @param {number} t
*/
module.exports.moveCameraVideo = function(top_left_corner, bottom_right_corner, scene, t) {
    _assertClass(scene, VideoScene);
    wasm.moveCameraVideo(addHeapObject(top_left_corner), addHeapObject(bottom_right_corner), scene.__wbg_ptr, t);
};

/**
* @param {WasmVectorObject} vec_obj
* @param {number} angle
* @param {number} t
* @returns {WasmVectorObject}
*/
module.exports.rotateAnimation = function(vec_obj, angle, t) {
    _assertClass(vec_obj, WasmVectorObject);
    var ptr0 = vec_obj.__destroy_into_raw();
    const ret = wasm.rotateAnimation(ptr0, angle, t);
    return WasmVectorObject.__wrap(ret);
};

/**
* @param {WasmVectorObject} vec_obj
* @param {number} scale_factor
* @param {number} t
* @returns {WasmVectorObject}
*/
module.exports.scaleInPlace = function(vec_obj, scale_factor, t) {
    _assertClass(vec_obj, WasmVectorObject);
    var ptr0 = vec_obj.__destroy_into_raw();
    const ret = wasm.scaleInPlace(ptr0, scale_factor, t);
    return WasmVectorObject.__wrap(ret);
};

/**
* @param {WasmVectorObject} vec_obj
* @param {WasmColor} target_fill
* @param {number} t
* @returns {WasmVectorObject}
*/
module.exports.setFillAnimation = function(vec_obj, target_fill, t) {
    _assertClass(vec_obj, WasmVectorObject);
    var ptr0 = vec_obj.__destroy_into_raw();
    _assertClass(target_fill, WasmColor);
    var ptr1 = target_fill.__destroy_into_raw();
    const ret = wasm.setFillAnimation(ptr0, ptr1, t);
    return WasmVectorObject.__wrap(ret);
};

/**
* @param {WasmVectorObject} vec_obj
* @param {WasmColor} target_stroke
* @param {number} t
* @returns {WasmVectorObject}
*/
module.exports.setStrokeAnimation = function(vec_obj, target_stroke, t) {
    _assertClass(vec_obj, WasmVectorObject);
    var ptr0 = vec_obj.__destroy_into_raw();
    _assertClass(target_stroke, WasmColor);
    var ptr1 = target_stroke.__destroy_into_raw();
    const ret = wasm.setStrokeAnimation(ptr0, ptr1, t);
    return WasmVectorObject.__wrap(ret);
};

/**
* @param {WasmVectorObject} vec_obj
* @param {Array<any>} shift
* @param {number} t
* @returns {WasmVectorObject}
*/
module.exports.shiftAnimation = function(vec_obj, shift, t) {
    _assertClass(vec_obj, WasmVectorObject);
    var ptr0 = vec_obj.__destroy_into_raw();
    const ret = wasm.shiftAnimation(ptr0, addHeapObject(shift), t);
    return WasmVectorObject.__wrap(ret);
};

/**
* @param {WasmVectorObject} vec_obj
* @param {number} t
* @returns {WasmVectorObject}
*/
module.exports.showTemporaily = function(vec_obj, t) {
    _assertClass(vec_obj, WasmVectorObject);
    var ptr0 = vec_obj.__destroy_into_raw();
    const ret = wasm.showTemporaily(ptr0, t);
    return WasmVectorObject.__wrap(ret);
};

/**
* @param {WasmVectorObject} vec_obj
* @param {number} angle
* @param {number} t
* @returns {WasmVectorObject}
*/
module.exports.spinningGrow = function(vec_obj, angle, t) {
    _assertClass(vec_obj, WasmVectorObject);
    var ptr0 = vec_obj.__destroy_into_raw();
    const ret = wasm.spinningGrow(ptr0, angle, t);
    return WasmVectorObject.__wrap(ret);
};

/**
* @param {number} ux
* @param {number} uy
* @param {number} vx
* @param {number} vy
* @returns {number}
*/
module.exports.radian = function(ux, uy, vx, vy) {
    const ret = wasm.radian(ux, uy, vx, vy);
    return ret;
};

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
module.exports.ellipticalArcPath = function(last_move, rx, ry, rotation, large_arc, sweep, x, y) {
    const ret = wasm.ellipticalArcPath(addHeapObject(last_move), rx, ry, rotation, large_arc, sweep, x, y);
    return takeObject(ret);
};

/**
* @param {Array<any>} points
* @returns {Array<any>}
*/
module.exports.getBbox = function(points) {
    const ret = wasm.getBbox(addHeapObject(points));
    return takeObject(ret);
};

/**
* @param {Array<any>} points
* @param {Array<any>} center_if_no_points
* @returns {Array<any>}
*/
module.exports.center = function(points, center_if_no_points) {
    const ret = wasm.center(addHeapObject(points), addHeapObject(center_if_no_points));
    return takeObject(ret);
};

/**
* @param {bigint} n
* @returns {bigint}
*/
module.exports.factorial = function(n) {
    const ret = wasm.factorial(n);
    return BigInt.asUintN(64, ret);
};

/**
* @param {string} hex
* @param {number} a
* @returns {WasmColor}
*/
module.exports.hexToColor = function(hex, a) {
    const ptr0 = passStringToWasm0(hex, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    const len0 = WASM_VECTOR_LEN;
    const ret = wasm.hexToColor(ptr0, len0, a);
    return WasmColor.__wrap(ret);
};

/**
* @param {Array<any>} points
* @param {number} t
* @returns {Array<any>}
*/
module.exports.bezier = function(points, t) {
    const ret = wasm.bezier(addHeapObject(points), t);
    return takeObject(ret);
};

/**
* @param {Array<any>} numbers
* @param {number} t
* @returns {number}
*/
module.exports.bezierNumber = function(numbers, t) {
    const ret = wasm.bezierNumber(addHeapObject(numbers), t);
    return ret;
};

/**
* @param {bigint} n
* @param {bigint} r
* @returns {bigint}
*/
module.exports.permutation = function(n, r) {
    const ret = wasm.permutation(n, r);
    return BigInt.asUintN(64, ret);
};

/**
* @param {bigint} n
* @param {bigint} r
* @returns {bigint}
*/
module.exports.choose = function(n, r) {
    const ret = wasm.choose(n, r);
    return BigInt.asUintN(64, ret);
};

/**
* @param {number} x1
* @param {number} y1
* @param {number} x2
* @param {number} y2
* @returns {number}
*/
module.exports.distanceSquared = function(x1, y1, x2, y2) {
    const ret = wasm.distanceSquared(x1, y1, x2, y2);
    return ret;
};

/**
* @param {number} x
* @param {number} y
* @param {number} t
* @returns {number}
*/
module.exports.interpolate = function(x, y, t) {
    const ret = wasm.interpolate(x, y, t);
    return ret;
};

/**
* @param {Array<any>} x
* @param {Array<any>} y
* @param {number} t
* @returns {Array<any>}
*/
module.exports.interpolateTuple = function(x, y, t) {
    const ret = wasm.interpolateTuple(addHeapObject(x), addHeapObject(y), t);
    return takeObject(ret);
};

/**
* @param {Array<any>} x
* @param {Array<any>} y
* @param {number} t
* @returns {Array<any>}
*/
module.exports.interpolateTuple3D = function(x, y, t) {
    const ret = wasm.interpolateTuple3D(addHeapObject(x), addHeapObject(y), t);
    return takeObject(ret);
};

/**
* @param {WasmColor} x
* @param {WasmColor} y
* @param {number} t
* @returns {WasmColor}
*/
module.exports.interpolateColor = function(x, y, t) {
    _assertClass(x, WasmColor);
    var ptr0 = x.__destroy_into_raw();
    _assertClass(y, WasmColor);
    var ptr1 = y.__destroy_into_raw();
    const ret = wasm.interpolateColor(ptr0, ptr1, t);
    return WasmColor.__wrap(ret);
};

/**
* @param {Array<any>} anchors1
* @param {Array<any>} handles1
* @param {Array<any>} handles2
* @param {Array<any>} anchors2
* @returns {Array<any>}
*/
module.exports.pointsFromAnchorsAndHandles = function(anchors1, handles1, handles2, anchors2) {
    const ret = wasm.pointsFromAnchorsAndHandles(addHeapObject(anchors1), addHeapObject(handles1), addHeapObject(handles2), addHeapObject(anchors2));
    return takeObject(ret);
};

/**
* @param {Array<any>} points
* @param {Array<any>} point
* @returns {Array<any>}
*/
module.exports.startNewPath = function(points, point) {
    const ret = wasm.startNewPath(addHeapObject(points), addHeapObject(point));
    return takeObject(ret);
};

/**
* @param {Array<any>} points
* @returns {boolean}
*/
module.exports.hasNewPathBegun = function(points) {
    const ret = wasm.hasNewPathBegun(addHeapObject(points));
    return ret !== 0;
};

/**
* @param {Array<any>} points
* @param {number} n
* @returns {Array<any>}
*/
module.exports.getNthSubpath = function(points, n) {
    const ret = wasm.getNthSubpath(addHeapObject(points), n);
    return takeObject(ret);
};

/**
* @param {number} n
* @param {Array<any>} points
* @returns {Array<any>}
*/
module.exports.insertNCurvesToPointList = function(n, points) {
    const ret = wasm.insertNCurvesToPointList(n, addHeapObject(points));
    return takeObject(ret);
};

/**
* @param {WasmVectorObject} vec_obj1
* @param {WasmVectorObject} vec_obj2
* @returns {Array<any>}
*/
module.exports.nullPointAlign = function(vec_obj1, vec_obj2) {
    _assertClass(vec_obj1, WasmVectorObject);
    var ptr0 = vec_obj1.__destroy_into_raw();
    _assertClass(vec_obj2, WasmVectorObject);
    var ptr1 = vec_obj2.__destroy_into_raw();
    const ret = wasm.nullPointAlign(ptr0, ptr1);
    return takeObject(ret);
};

/**
* @param {Array<any>} points1
* @param {Array<any>} points2
* @param {Array<any>} center_if_no_points
* @returns {Array<any>}
*/
module.exports.alignPoints = function(points1, points2, center_if_no_points) {
    const ret = wasm.alignPoints(addHeapObject(points1), addHeapObject(points2), addHeapObject(center_if_no_points));
    return takeObject(ret);
};

/**
* @param {WasmVectorObject} vec_obj
* @param {number} n
* @param {Array<any>} center_if_no_points
* @returns {WasmVectorObject}
*/
module.exports.addNMoreSubobjects = function(vec_obj, n, center_if_no_points) {
    _assertClass(vec_obj, WasmVectorObject);
    var ptr0 = vec_obj.__destroy_into_raw();
    const ret = wasm.addNMoreSubobjects(ptr0, n, addHeapObject(center_if_no_points));
    return WasmVectorObject.__wrap(ret);
};

/**
* @param {WasmVectorObject} vec_obj1
* @param {WasmVectorObject} vec_obj2
* @param {Array<any>} center_if_no_points
* @returns {(WasmVectorObject)[]}
*/
module.exports.alignSubobjects = function(vec_obj1, vec_obj2, center_if_no_points) {
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
};

/**
* @param {WasmVectorObject} vec_obj1
* @param {WasmVectorObject} vec_obj2
* @param {boolean} skip_point_align
* @param {Array<any>} center_if_no_points
* @returns {Array<any>}
*/
module.exports.alignData = function(vec_obj1, vec_obj2, skip_point_align, center_if_no_points) {
    _assertClass(vec_obj1, WasmVectorObject);
    var ptr0 = vec_obj1.__destroy_into_raw();
    _assertClass(vec_obj2, WasmVectorObject);
    var ptr1 = vec_obj2.__destroy_into_raw();
    const ret = wasm.alignData(ptr0, ptr1, skip_point_align, addHeapObject(center_if_no_points));
    return takeObject(ret);
};

/**
* @param {number} x
* @param {number} y
* @param {number} t
* @returns {Array<any>}
*/
module.exports.integerInterpolate = function(x, y, t) {
    const ret = wasm.integerInterpolate(x, y, t);
    return takeObject(ret);
};

/**
* @param {number} x1
* @param {number} y1
* @param {number} x2
* @param {number} y2
* @returns {Array<any>}
*/
module.exports.lineAsCubicBezier = function(x1, y1, x2, y2) {
    const ret = wasm.lineAsCubicBezier(x1, y1, x2, y2);
    return takeObject(ret);
};

/**
* @param {number} x1
* @param {number} y1
* @param {number} x2
* @param {number} y2
* @param {number} x3
* @param {number} y3
* @returns {Array<any>}
*/
module.exports.quadraticBezierAsCubicBezier = function(x1, y1, x2, y2, x3, y3) {
    const ret = wasm.quadraticBezierAsCubicBezier(x1, y1, x2, y2, x3, y3);
    return takeObject(ret);
};

/**
* @param {number} x1
* @param {number} y1
* @param {number} x2
* @param {number} y2
* @returns {boolean}
*/
module.exports.considerPointsEquals = function(x1, y1, x2, y2) {
    const ret = wasm.considerPointsEquals(x1, y1, x2, y2);
    return ret !== 0;
};

/**
* @param {number} t
* @returns {number}
*/
module.exports.sigmoid = function(t) {
    const ret = wasm.sigmoid(t);
    return ret;
};

/**
* @param {number} t
* @returns {number}
*/
module.exports.linear = function(t) {
    const ret = wasm.linear(t);
    return ret;
};

/**
* @param {number} t
* @param {number} inflection
* @returns {number}
*/
module.exports.smooth = function(t, inflection) {
    const ret = wasm.smooth(t, inflection);
    return ret;
};

/**
* @param {number} t
* @returns {number}
*/
module.exports.smoothstep = function(t) {
    const ret = wasm.smoothstep(t);
    return ret;
};

/**
* @param {number} t
* @returns {number}
*/
module.exports.smootherstep = function(t) {
    const ret = wasm.smootherstep(t);
    return ret;
};

/**
* @param {number} t
* @returns {number}
*/
module.exports.smoothererstep = function(t) {
    const ret = wasm.smoothererstep(t);
    return ret;
};

/**
* @param {number} t
* @param {number} inflection
* @returns {number}
*/
module.exports.rushInto = function(t, inflection) {
    const ret = wasm.rushInto(t, inflection);
    return ret;
};

/**
* @param {number} t
* @param {number} inflection
* @returns {number}
*/
module.exports.rushFrom = function(t, inflection) {
    const ret = wasm.rushFrom(t, inflection);
    return ret;
};

/**
* @param {number} t
* @returns {number}
*/
module.exports.slowInto = function(t) {
    const ret = wasm.easeOutCirc(t);
    return ret;
};

/**
* @param {number} t
* @returns {number}
*/
module.exports.doubleSmooth = function(t) {
    const ret = wasm.doubleSmooth(t);
    return ret;
};

/**
* @param {number} t
* @param {number} inflection
* @returns {number}
*/
module.exports.thereAndBack = function(t, inflection) {
    const ret = wasm.thereAndBack(t, inflection);
    return ret;
};

/**
* @param {number} t
* @param {number} pause_ratio
* @returns {number}
*/
module.exports.thereAndBackWithPause = function(t, pause_ratio) {
    const ret = wasm.thereAndBackWithPause(t, pause_ratio);
    return ret;
};

/**
* @param {number} t
* @param {number} pull_factor
* @returns {number}
*/
module.exports.runningStart = function(t, pull_factor) {
    const ret = wasm.runningStart(t, pull_factor);
    return ret;
};

/**
* @param {Function} func
* @param {number} t
* @param {number} proportion
* @returns {number}
*/
module.exports.notQuiteThere = function(func, t, proportion) {
    const ret = wasm.notQuiteThere(addHeapObject(func), t, proportion);
    return ret;
};

/**
* @param {number} t
* @param {number} wiggles
* @returns {number}
*/
module.exports.wiggle = function(t, wiggles) {
    const ret = wasm.wiggle(t, wiggles);
    return ret;
};

/**
* @param {Function} func
* @param {number} t
* @param {number} a
* @param {number} b
* @returns {number}
*/
module.exports.squishRateFunc = function(func, t, a, b) {
    const ret = wasm.squishRateFunc(addHeapObject(func), t, a, b);
    return ret;
};

/**
* @param {number} t
* @returns {number}
*/
module.exports.lingering = function(t) {
    const ret = wasm.lingering(t);
    return ret;
};

/**
* @param {number} t
* @param {number} half_life
* @returns {number}
*/
module.exports.exponentialDecay = function(t, half_life) {
    const ret = wasm.exponentialDecay(t, half_life);
    return ret;
};

/**
* @param {number} t
* @returns {number}
*/
module.exports.easeInSine = function(t) {
    const ret = wasm.easeInSine(t);
    return ret;
};

/**
* @param {number} t
* @returns {number}
*/
module.exports.easeOutSine = function(t) {
    const ret = wasm.easeOutSine(t);
    return ret;
};

/**
* @param {number} t
* @returns {number}
*/
module.exports.easeInOutSine = function(t) {
    const ret = wasm.easeInOutSine(t);
    return ret;
};

/**
* @param {number} t
* @returns {number}
*/
module.exports.easeInQuad = function(t) {
    const ret = wasm.easeInQuad(t);
    return ret;
};

/**
* @param {number} t
* @returns {number}
*/
module.exports.easeOutQuad = function(t) {
    const ret = wasm.easeOutQuad(t);
    return ret;
};

/**
* @param {number} t
* @returns {number}
*/
module.exports.easeInOutQuad = function(t) {
    const ret = wasm.easeInOutQuad(t);
    return ret;
};

/**
* @param {number} t
* @returns {number}
*/
module.exports.easeInCubic = function(t) {
    const ret = wasm.easeInCubic(t);
    return ret;
};

/**
* @param {number} t
* @returns {number}
*/
module.exports.easeOutCubic = function(t) {
    const ret = wasm.easeOutCubic(t);
    return ret;
};

/**
* @param {number} t
* @returns {number}
*/
module.exports.easeInOutCubic = function(t) {
    const ret = wasm.easeInOutCubic(t);
    return ret;
};

/**
* @param {number} t
* @returns {number}
*/
module.exports.easeInQuart = function(t) {
    const ret = wasm.easeInQuart(t);
    return ret;
};

/**
* @param {number} t
* @returns {number}
*/
module.exports.easeOutQuart = function(t) {
    const ret = wasm.easeOutQuart(t);
    return ret;
};

/**
* @param {number} t
* @returns {number}
*/
module.exports.easeInOutQuart = function(t) {
    const ret = wasm.easeInOutQuart(t);
    return ret;
};

/**
* @param {number} t
* @returns {number}
*/
module.exports.easeInQuint = function(t) {
    const ret = wasm.easeInQuint(t);
    return ret;
};

/**
* @param {number} t
* @returns {number}
*/
module.exports.easeOutQuint = function(t) {
    const ret = wasm.easeOutQuint(t);
    return ret;
};

/**
* @param {number} t
* @returns {number}
*/
module.exports.easeInOutQuint = function(t) {
    const ret = wasm.easeInOutQuint(t);
    return ret;
};

/**
* @param {number} t
* @returns {number}
*/
module.exports.easeInExpo = function(t) {
    const ret = wasm.easeInExpo(t);
    return ret;
};

/**
* @param {number} t
* @returns {number}
*/
module.exports.easeOutExpo = function(t) {
    const ret = wasm.easeOutExpo(t);
    return ret;
};

/**
* @param {number} t
* @returns {number}
*/
module.exports.easeInOutExpo = function(t) {
    const ret = wasm.easeInOutExpo(t);
    return ret;
};

/**
* @param {number} t
* @returns {number}
*/
module.exports.easeInCirc = function(t) {
    const ret = wasm.easeInCirc(t);
    return ret;
};

/**
* @param {number} t
* @returns {number}
*/
module.exports.easeOutCirc = function(t) {
    const ret = wasm.easeOutCirc(t);
    return ret;
};

/**
* @param {number} t
* @returns {number}
*/
module.exports.easeInOutCirc = function(t) {
    const ret = wasm.easeInOutCirc(t);
    return ret;
};

/**
* @param {number} t
* @returns {number}
*/
module.exports.easeInBack = function(t) {
    const ret = wasm.easeInBack(t);
    return ret;
};

/**
* @param {number} t
* @returns {number}
*/
module.exports.easeOutBack = function(t) {
    const ret = wasm.easeInBack(t);
    return ret;
};

/**
* @param {number} t
* @returns {number}
*/
module.exports.easeInOutBack = function(t) {
    const ret = wasm.easeInOutBack(t);
    return ret;
};

/**
* @param {number} t
* @returns {number}
*/
module.exports.easeInElastic = function(t) {
    const ret = wasm.easeInElastic(t);
    return ret;
};

/**
* @param {number} t
* @returns {number}
*/
module.exports.easeOutElastic = function(t) {
    const ret = wasm.easeOutElastic(t);
    return ret;
};

/**
* @param {number} t
* @returns {number}
*/
module.exports.easeInOutElastic = function(t) {
    const ret = wasm.easeInOutElastic(t);
    return ret;
};

/**
* @param {number} t
* @returns {number}
*/
module.exports.easeOutBounce = function(t) {
    const ret = wasm.easeInBounce(t);
    return ret;
};

/**
* @param {number} t
* @returns {number}
*/
module.exports.easeInBounce = function(t) {
    const ret = wasm.easeInBounce(t);
    return ret;
};

/**
* @param {number} t
* @returns {number}
*/
module.exports.easeInOutBounce = function(t) {
    const ret = wasm.easeInOutBounce(t);
    return ret;
};

/**
* @param {string} text
* @param {string} font_family
* @returns {Promise<WasmVectorObject>}
*/
module.exports.textToVector = function(text, font_family) {
    const ptr0 = passStringToWasm0(text, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    const len0 = WASM_VECTOR_LEN;
    const ptr1 = passStringToWasm0(font_family, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    const len1 = WASM_VECTOR_LEN;
    const ret = wasm.textToVector(ptr0, len0, ptr1, len1);
    return takeObject(ret);
};

/**
* @param {string} expression
* @returns {Promise<WasmVectorObject>}
*/
module.exports.mathjax = function(expression) {
    const ptr0 = passStringToWasm0(expression, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    const len0 = WASM_VECTOR_LEN;
    const ret = wasm.mathjax(ptr0, len0);
    return takeObject(ret);
};

function passArray32ToWasm0(arg, malloc) {
    const ptr = malloc(arg.length * 4, 4) >>> 0;
    getUint32Memory0().set(arg, ptr / 4);
    WASM_VECTOR_LEN = arg.length;
    return ptr;
}

function handleError(f, args) {
    try {
        return f.apply(this, args);
    } catch (e) {
        wasm.__wbindgen_exn_store(addHeapObject(e));
    }
}
function __wbg_adapter_497(arg0, arg1, arg2, arg3) {
    wasm.wasm_bindgen__convert__closures__invoke2_mut__h271b87efd55193df(arg0, arg1, addHeapObject(arg2), addHeapObject(arg3));
}

const GenericSceneFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_genericscene_free(ptr >>> 0));
/**
*/
class GenericScene {

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
    * @param {VideoScene} scene
    * @returns {GenericScene}
    */
    static fromVideoScene(scene) {
        _assertClass(scene, VideoScene);
        var ptr0 = scene.__destroy_into_raw();
        const ret = wasm.genericscene_fromVideoScene(ptr0);
        return GenericScene.__wrap(ret);
    }
    /**
    * @returns {boolean}
    */
    isScene() {
        const ret = wasm.genericscene_isSVGScene(this.__wbg_ptr);
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
}
module.exports.GenericScene = GenericScene;

const VideoSceneFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_videoscene_free(ptr >>> 0));
/**
*/
class VideoScene {

    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        VideoSceneFinalization.unregister(this);
        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_videoscene_free(ptr);
    }
    /**
    * @param {bigint} width
    * @param {bigint} height
    * @param {bigint} fps
    */
    constructor(width, height, fps) {
        const ret = wasm.videoscene_new_js(width, height, fps);
        this.__wbg_ptr = ret >>> 0;
        return this;
    }
    /**
    * @returns {bigint}
    */
    getFps() {
        const ret = wasm.videoscene_getFps(this.__wbg_ptr);
        return BigInt.asUintN(64, ret);
    }
    /**
    * @returns {bigint}
    */
    getHeight() {
        const ret = wasm.videoscene_getHeight(this.__wbg_ptr);
        return BigInt.asUintN(64, ret);
    }
    /**
    * @returns {bigint}
    */
    getWidth() {
        const ret = wasm.videoscene_getWidth(this.__wbg_ptr);
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
        wasm.videoscene_clear(this.__wbg_ptr);
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
    * @param {WasmGradientImageOrColor} background
    */
    setBackground(background) {
        _assertClass(background, WasmGradientImageOrColor);
        var ptr0 = background.__destroy_into_raw();
        wasm.videoscene_setBackground(this.__wbg_ptr, ptr0);
    }
    /**
    * @param {WasmVectorObject} vec_obj
    */
    add(vec_obj) {
        _assertClass(vec_obj, WasmVectorObject);
        var ptr0 = vec_obj.__destroy_into_raw();
        wasm.videoscene_add(this.__wbg_ptr, ptr0);
    }
    /**
    * @param {number} index
    * @param {WasmVectorObject} vec_obj
    */
    insert(index, vec_obj) {
        _assertClass(vec_obj, WasmVectorObject);
        var ptr0 = vec_obj.__destroy_into_raw();
        wasm.videoscene_insert(this.__wbg_ptr, index, ptr0);
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
    * @param {any} context
    */
    setCanvasContext(context) {
        wasm.videoscene_setCanvasContext(this.__wbg_ptr, addHeapObject(context));
    }
    /**
    * @param {number} duration_in_ms
    * @returns {Promise<void>}
    */
    sleep(duration_in_ms) {
        const ret = wasm.videoscene_sleep(this.__wbg_ptr, duration_in_ms);
        return takeObject(ret);
    }
    /**
    * @param {Array<any>} objects
    */
    setObjects(objects) {
        wasm.videoscene_setObjects(this.__wbg_ptr, addHeapObject(objects));
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
        const ret = wasm.videoscene_play(this.__wbg_ptr, addHeapObject(animation_func), ptr0, len0, duration_in_frames, addHeapObject(rate_func));
        return takeObject(ret);
    }
    /**
    * @param {Function} animation_func
    * @param {Array<any>} objects
    * @param {number} t
    * @returns {Promise<void>}
    */
    makeFrame(animation_func, objects, t) {
        const ret = wasm.videoscene_makeFrame(this.__wbg_ptr, addHeapObject(animation_func), addHeapObject(objects), t);
        return takeObject(ret);
    }
    /**
    * @param {bigint} duration_in_frames
    * @returns {Promise<void>}
    */
    wait(duration_in_frames) {
        const ret = wasm.videoscene_wait(this.__wbg_ptr, duration_in_frames);
        return takeObject(ret);
    }
    /**
    * @param {Function} callback
    */
    setCallback(callback) {
        wasm.videoscene_setCallback(this.__wbg_ptr, addHeapObject(callback));
    }
    /**
    * @returns {Promise<void>}
    */
    callCallback() {
        const ret = wasm.videoscene_callCallback(this.__wbg_ptr);
        return takeObject(ret);
    }
    /**
    * @param {string} path
    * @param {string | undefined} [codec]
    * @param {string | undefined} [pix_fmt]
    * @param {string | undefined} [qp]
    */
    initFFmpeg(path, codec, pix_fmt, qp) {
        const ptr0 = passStringToWasm0(path, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        var ptr1 = isLikeNone(codec) ? 0 : passStringToWasm0(codec, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        var len1 = WASM_VECTOR_LEN;
        var ptr2 = isLikeNone(pix_fmt) ? 0 : passStringToWasm0(pix_fmt, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        var len2 = WASM_VECTOR_LEN;
        var ptr3 = isLikeNone(qp) ? 0 : passStringToWasm0(qp, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        var len3 = WASM_VECTOR_LEN;
        wasm.videoscene_initFFmpeg(this.__wbg_ptr, ptr0, len0, ptr1, len1, ptr2, len2, ptr3, len3);
    }
    /**
    */
    closeFFmpeg() {
        wasm.videoscene_closeFFmpeg(this.__wbg_ptr);
    }
}
module.exports.VideoScene = VideoScene;

const WasmCameraFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_wasmcamera_free(ptr >>> 0));
/**
*/
class WasmCamera {

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
module.exports.WasmCamera = WasmCamera;

const WasmColorFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_wasmcolor_free(ptr >>> 0));
/**
*/
class WasmColor {

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
module.exports.WasmColor = WasmColor;

const WasmGradientImageOrColorFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_wasmgradientimageorcolor_free(ptr >>> 0));
/**
*/
class WasmGradientImageOrColor {

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
module.exports.WasmGradientImageOrColor = WasmGradientImageOrColor;

const WasmGradientStopFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_wasmgradientstop_free(ptr >>> 0));
/**
*/
class WasmGradientStop {

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
module.exports.WasmGradientStop = WasmGradientStop;

const WasmImageFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_wasmimage_free(ptr >>> 0));
/**
*/
class WasmImage {

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
module.exports.WasmImage = WasmImage;

const WasmLightSourceFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_wasmlightsource_free(ptr >>> 0));
/**
*/
class WasmLightSource {

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
module.exports.WasmLightSource = WasmLightSource;

const WasmLinearGradientFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_wasmlineargradient_free(ptr >>> 0));
/**
*/
class WasmLinearGradient {

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
module.exports.WasmLinearGradient = WasmLinearGradient;

const WasmRadialGradientFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_wasmradialgradient_free(ptr >>> 0));
/**
*/
class WasmRadialGradient {

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
module.exports.WasmRadialGradient = WasmRadialGradient;

const WasmThreeDObjectFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_wasmthreedobject_free(ptr >>> 0));
/**
*/
class WasmThreeDObject {

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
module.exports.WasmThreeDObject = WasmThreeDObject;

const WasmVectorObjectFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_wasmvectorobject_free(ptr >>> 0));
/**
*/
class WasmVectorObject {

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
module.exports.WasmVectorObject = WasmVectorObject;

module.exports.__wbg_wasmthreedobject_new = function(arg0) {
    const ret = WasmThreeDObject.__wrap(arg0);
    return addHeapObject(ret);
};

module.exports.__wbg_wasmvectorobject_new = function(arg0) {
    const ret = WasmVectorObject.__wrap(arg0);
    return addHeapObject(ret);
};

module.exports.__wbg_wasmgradientstop_new = function(arg0) {
    const ret = WasmGradientStop.__wrap(arg0);
    return addHeapObject(ret);
};

module.exports.__wbindgen_object_drop_ref = function(arg0) {
    takeObject(arg0);
};

module.exports.__wbindgen_number_get = function(arg0, arg1) {
    const obj = getObject(arg1);
    const ret = typeof(obj) === 'number' ? obj : undefined;
    getFloat64Memory0()[arg0 / 8 + 1] = isLikeNone(ret) ? 0 : ret;
    getInt32Memory0()[arg0 / 4 + 0] = !isLikeNone(ret);
};

module.exports.__wbindgen_number_new = function(arg0) {
    const ret = arg0;
    return addHeapObject(ret);
};

module.exports.__wbindgen_string_new = function(arg0, arg1) {
    const ret = getStringFromWasm0(arg0, arg1);
    return addHeapObject(ret);
};

module.exports.__wbindgen_boolean_get = function(arg0) {
    const v = getObject(arg0);
    const ret = typeof(v) === 'boolean' ? (v ? 1 : 0) : 2;
    return ret;
};

module.exports.__wbindgen_string_get = function(arg0, arg1) {
    const obj = getObject(arg1);
    const ret = typeof(obj) === 'string' ? obj : undefined;
    var ptr1 = isLikeNone(ret) ? 0 : passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    var len1 = WASM_VECTOR_LEN;
    getInt32Memory0()[arg0 / 4 + 1] = len1;
    getInt32Memory0()[arg0 / 4 + 0] = ptr1;
};

module.exports.__wbindgen_is_function = function(arg0) {
    const ret = typeof(getObject(arg0)) === 'function';
    return ret;
};

module.exports.__wbg_wasmcolor_unwrap = function(arg0) {
    const ret = WasmColor.__unwrap(takeObject(arg0));
    return ret;
};

module.exports.__wbg_wasmgradientstop_unwrap = function(arg0) {
    const ret = WasmGradientStop.__unwrap(takeObject(arg0));
    return ret;
};

module.exports.__wbg_wasmvectorobject_unwrap = function(arg0) {
    const ret = WasmVectorObject.__unwrap(takeObject(arg0));
    return ret;
};

module.exports.__wbg_log_5b0a66dc01cfcbc2 = function(arg0, arg1) {
    console.log(getStringFromWasm0(arg0, arg1));
};

module.exports.__wbindgen_object_clone_ref = function(arg0) {
    const ret = getObject(arg0);
    return addHeapObject(ret);
};

module.exports.__wbg_wasmthreedobject_unwrap = function(arg0) {
    const ret = WasmThreeDObject.__unwrap(takeObject(arg0));
    return ret;
};

module.exports.__wbg_beginPath_23a9d082cf1315c2 = function(arg0) {
    getObject(arg0).beginPath();
};

module.exports.__wbg_moveTo_30dea165698185dc = function(arg0, arg1, arg2) {
    getObject(arg0).moveTo(arg1, arg2);
};

module.exports.__wbg_bezierCurveTo_2f670ffcbda1c0a7 = function(arg0, arg1, arg2, arg3, arg4, arg5, arg6) {
    getObject(arg0).bezierCurveTo(arg1, arg2, arg3, arg4, arg5, arg6);
};

module.exports.__wbg_closePath_e267be613540ec80 = function(arg0) {
    getObject(arg0).closePath();
};

module.exports.__wbg_setfillStyle_647b3200bdbd026c = function(arg0, arg1, arg2) {
    let deferred0_0;
    let deferred0_1;
    try {
        deferred0_0 = arg1;
        deferred0_1 = arg2;
        getObject(arg0).fillStyle = getStringFromWasm0(arg1, arg2);
    } finally {
        wasm.__wbindgen_free(deferred0_0, deferred0_1, 1);
    }
};

module.exports.__wbg_fill_c21f5c44f22c5bfb = function(arg0) {
    getObject(arg0).fill();
};

module.exports.__wbg_new_175cd2b1b91493ab = function() {
    const ret = new Image();
    return addHeapObject(ret);
};

module.exports.__wbg_setsrc_55c1b76046b92307 = function(arg0, arg1, arg2) {
    let deferred0_0;
    let deferred0_1;
    try {
        deferred0_0 = arg1;
        deferred0_1 = arg2;
        getObject(arg0).src = getStringFromWasm0(arg1, arg2);
    } finally {
        wasm.__wbindgen_free(deferred0_0, deferred0_1, 1);
    }
};

module.exports.__wbg_createCanvas_5360d349aecfd6f6 = function(arg0, arg1) {
    const ret = createCanvas(arg0 >>> 0, arg1 >>> 0);
    return addHeapObject(ret);
};

module.exports.__wbg_getContext_5374073611ee68cf = function(arg0, arg1, arg2) {
    const ret = getObject(arg0).getContext(getStringFromWasm0(arg1, arg2));
    return addHeapObject(ret);
};

module.exports.__wbg_setglobalAlpha_813a53868d26f872 = function(arg0, arg1) {
    getObject(arg0).globalAlpha = arg1;
};

module.exports.__wbg_drawImage_7f921f86c50a9a91 = function(arg0, arg1, arg2, arg3, arg4, arg5) {
    getObject(arg0).drawImage(getObject(arg1), arg2, arg3, arg4, arg5);
};

module.exports.__wbg_createPattern_310e8a6ff9ef0a0c = function(arg0, arg1, arg2, arg3) {
    const ret = getObject(arg0).createPattern(getObject(arg1), getStringFromWasm0(arg2, arg3));
    return addHeapObject(ret);
};

module.exports.__wbg_setfillStyle_e6cf0c05d00f2f65 = function(arg0, arg1) {
    getObject(arg0).fillStyle = takeObject(arg1);
};

module.exports.__wbg_createRadialGradient_f917dbb3ae3a8d9d = function(arg0, arg1, arg2, arg3, arg4, arg5, arg6) {
    const ret = getObject(arg0).createRadialGradient(arg1, arg2, arg3, arg4, arg5, arg6);
    return addHeapObject(ret);
};

module.exports.__wbg_addColorStop_f4dde7c62ebf88b9 = function(arg0, arg1, arg2, arg3) {
    let deferred0_0;
    let deferred0_1;
    try {
        deferred0_0 = arg2;
        deferred0_1 = arg3;
        getObject(arg0).addColorStop(arg1, getStringFromWasm0(arg2, arg3));
    } finally {
        wasm.__wbindgen_free(deferred0_0, deferred0_1, 1);
    }
};

module.exports.__wbg_setfillStyle_7ad5ad0ae8f035ad = function(arg0, arg1) {
    getObject(arg0).fillStyle = takeObject(arg1);
};

module.exports.__wbg_createLinearGradient_e22419a19e2a1818 = function(arg0, arg1, arg2, arg3, arg4) {
    const ret = getObject(arg0).createLinearGradient(arg1, arg2, arg3, arg4);
    return addHeapObject(ret);
};

module.exports.__wbg_setstrokeStyle_cab165e995015cb1 = function(arg0, arg1, arg2) {
    let deferred0_0;
    let deferred0_1;
    try {
        deferred0_0 = arg1;
        deferred0_1 = arg2;
        getObject(arg0).strokeStyle = getStringFromWasm0(arg1, arg2);
    } finally {
        wasm.__wbindgen_free(deferred0_0, deferred0_1, 1);
    }
};

module.exports.__wbg_setlineWidth_27c9097a0e66bb2e = function(arg0, arg1) {
    getObject(arg0).lineWidth = arg1;
};

module.exports.__wbg_setlineCap_f792a599312de54f = function(arg0, arg1, arg2) {
    let deferred0_0;
    let deferred0_1;
    try {
        deferred0_0 = arg1;
        deferred0_1 = arg2;
        getObject(arg0).lineCap = getStringFromWasm0(arg1, arg2);
    } finally {
        wasm.__wbindgen_free(deferred0_0, deferred0_1, 1);
    }
};

module.exports.__wbg_setlineJoin_46d1780e5302249c = function(arg0, arg1, arg2) {
    let deferred0_0;
    let deferred0_1;
    try {
        deferred0_0 = arg1;
        deferred0_1 = arg2;
        getObject(arg0).lineJoin = getStringFromWasm0(arg1, arg2);
    } finally {
        wasm.__wbindgen_free(deferred0_0, deferred0_1, 1);
    }
};

module.exports.__wbg_stroke_ae3d84ae43ae721e = function(arg0) {
    getObject(arg0).stroke();
};

module.exports.__wbg_setstrokeStyle_51f2dcb8e09334f8 = function(arg0, arg1) {
    getObject(arg0).strokeStyle = takeObject(arg1);
};

module.exports.__wbg_setstrokeStyle_4de2d2c978bd0728 = function(arg0, arg1) {
    getObject(arg0).strokeStyle = takeObject(arg1);
};

module.exports.__wbg_resetTransform_b2539086ffa036f1 = function(arg0) {
    getObject(arg0).resetTransform();
};

module.exports.__wbg_scale_03c58684ecca53f4 = function(arg0, arg1, arg2) {
    getObject(arg0).scale(arg1, arg2);
};

module.exports.__wbg_translate_3917a51390064c13 = function(arg0, arg1, arg2) {
    getObject(arg0).translate(arg1, arg2);
};

module.exports.__wbg_clearRect_91b1b212891de6d5 = function(arg0, arg1, arg2, arg3, arg4) {
    getObject(arg0).clearRect(arg1, arg2, arg3, arg4);
};

module.exports.__wbg_setwidth_0f7710c4f796c6e3 = function(arg0, arg1) {
    getObject(arg0).width = arg1 >>> 0;
};

module.exports.__wbg_setheight_c82c3dc50c43d0a6 = function(arg0, arg1) {
    getObject(arg0).height = arg1 >>> 0;
};

module.exports.__wbg_fillRect_00a7671db57ce9f9 = function(arg0, arg1, arg2, arg3, arg4) {
    getObject(arg0).fillRect(arg1, arg2, arg3, arg4);
};

module.exports.__wbg_spawn_f80c477c46348f28 = function(arg0, arg1, arg2, arg3) {
    var v0 = getArrayJsValueFromWasm0(arg2, arg3).slice();
    wasm.__wbindgen_free(arg2, arg3 * 4, 4);
    const ret = spawn(getStringFromWasm0(arg0, arg1), v0);
    return addHeapObject(ret);
};

module.exports.__wbg_canvas_85baf8e191b03fea = function(arg0) {
    const ret = getObject(arg0).canvas;
    return addHeapObject(ret);
};

module.exports.__wbg_toBuffer_b65e155ad0028835 = function(arg0, arg1, arg2) {
    const ret = getObject(arg0).toBuffer(getStringFromWasm0(arg1, arg2));
    return addHeapObject(ret);
};

module.exports.__wbg_write_3757407a89dcc07f = function(arg0, arg1) {
    getObject(arg0).write(takeObject(arg1));
};

module.exports.__wbg_stdin_492618d6f23eb15d = function(arg0) {
    const ret = getObject(arg0).stdin;
    return addHeapObject(ret);
};

module.exports.__wbg_end_4afef9b51c2a0fd6 = function(arg0) {
    getObject(arg0).end();
};

module.exports.__wbg_error_61c269c0ba2e9f2f = function(arg0) {
    console.error(takeObject(arg0));
};

module.exports.__wbindgen_cb_drop = function(arg0) {
    const obj = takeObject(arg0).original;
    if (obj.cnt-- == 1) {
        obj.a = 0;
        return true;
    }
    const ret = false;
    return ret;
};

module.exports.__wbindgen_error_new = function(arg0, arg1) {
    const ret = new Error(getStringFromWasm0(arg0, arg1));
    return addHeapObject(ret);
};

module.exports.__wbg_queueMicrotask_481971b0d87f3dd4 = function(arg0) {
    queueMicrotask(getObject(arg0));
};

module.exports.__wbg_queueMicrotask_3cbae2ec6b6cd3d6 = function(arg0) {
    const ret = getObject(arg0).queueMicrotask;
    return addHeapObject(ret);
};

module.exports.__wbg_get_bd8e338fbd5f5cc8 = function(arg0, arg1) {
    const ret = getObject(arg0)[arg1 >>> 0];
    return addHeapObject(ret);
};

module.exports.__wbg_length_cd7af8117672b8b8 = function(arg0) {
    const ret = getObject(arg0).length;
    return ret;
};

module.exports.__wbg_new_16b304a2cfa7ff4a = function() {
    const ret = new Array();
    return addHeapObject(ret);
};

module.exports.__wbg_newnoargs_e258087cd0daa0ea = function(arg0, arg1) {
    const ret = new Function(getStringFromWasm0(arg0, arg1));
    return addHeapObject(ret);
};

module.exports.__wbg_new_d9bc3a0147634640 = function() {
    const ret = new Map();
    return addHeapObject(ret);
};

module.exports.__wbg_get_e3c254076557e348 = function() { return handleError(function (arg0, arg1) {
    const ret = Reflect.get(getObject(arg0), getObject(arg1));
    return addHeapObject(ret);
}, arguments) };

module.exports.__wbg_call_27c0f87801dedf93 = function() { return handleError(function (arg0, arg1) {
    const ret = getObject(arg0).call(getObject(arg1));
    return addHeapObject(ret);
}, arguments) };

module.exports.__wbg_self_ce0dbfc45cf2f5be = function() { return handleError(function () {
    const ret = self.self;
    return addHeapObject(ret);
}, arguments) };

module.exports.__wbg_window_c6fb939a7f436783 = function() { return handleError(function () {
    const ret = window.window;
    return addHeapObject(ret);
}, arguments) };

module.exports.__wbg_globalThis_d1e6af4856ba331b = function() { return handleError(function () {
    const ret = globalThis.globalThis;
    return addHeapObject(ret);
}, arguments) };

module.exports.__wbg_global_207b558942527489 = function() { return handleError(function () {
    const ret = global.global;
    return addHeapObject(ret);
}, arguments) };

module.exports.__wbindgen_is_undefined = function(arg0) {
    const ret = getObject(arg0) === undefined;
    return ret;
};

module.exports.__wbg_eval_020a6ea487e91ede = function() { return handleError(function (arg0, arg1) {
    const ret = eval(getStringFromWasm0(arg0, arg1));
    return addHeapObject(ret);
}, arguments) };

module.exports.__wbg_from_89e3fc3ba5e6fb48 = function(arg0) {
    const ret = Array.from(getObject(arg0));
    return addHeapObject(ret);
};

module.exports.__wbg_isArray_2ab64d95e09ea0ae = function(arg0) {
    const ret = Array.isArray(getObject(arg0));
    return ret;
};

module.exports.__wbg_of_4a2b313a453ec059 = function(arg0) {
    const ret = Array.of(getObject(arg0));
    return addHeapObject(ret);
};

module.exports.__wbg_of_647f9238b4d5407a = function(arg0, arg1) {
    const ret = Array.of(getObject(arg0), getObject(arg1));
    return addHeapObject(ret);
};

module.exports.__wbg_of_6a70eed8d41f469c = function(arg0, arg1, arg2) {
    const ret = Array.of(getObject(arg0), getObject(arg1), getObject(arg2));
    return addHeapObject(ret);
};

module.exports.__wbg_of_40f6b8e691c04867 = function(arg0, arg1, arg2, arg3) {
    const ret = Array.of(getObject(arg0), getObject(arg1), getObject(arg2), getObject(arg3));
    return addHeapObject(ret);
};

module.exports.__wbg_push_a5b05aedc7234f9f = function(arg0, arg1) {
    const ret = getObject(arg0).push(getObject(arg1));
    return ret;
};

module.exports.__wbg_call_b3ca7c6051f9bec1 = function() { return handleError(function (arg0, arg1, arg2) {
    const ret = getObject(arg0).call(getObject(arg1), getObject(arg2));
    return addHeapObject(ret);
}, arguments) };

module.exports.__wbg_call_8e7cb608789c2528 = function() { return handleError(function (arg0, arg1, arg2, arg3) {
    const ret = getObject(arg0).call(getObject(arg1), getObject(arg2), getObject(arg3));
    return addHeapObject(ret);
}, arguments) };

module.exports.__wbg_set_8417257aaedc936b = function(arg0, arg1, arg2) {
    const ret = getObject(arg0).set(getObject(arg1), getObject(arg2));
    return addHeapObject(ret);
};

module.exports.__wbg_instanceof_Promise_b438ddea4cacc51f = function(arg0) {
    let result;
    try {
        result = getObject(arg0) instanceof Promise;
    } catch (_) {
        result = false;
    }
    const ret = result;
    return ret;
};

module.exports.__wbg_new_81740750da40724f = function(arg0, arg1) {
    try {
        var state0 = {a: arg0, b: arg1};
        var cb0 = (arg0, arg1) => {
            const a = state0.a;
            state0.a = 0;
            try {
                return __wbg_adapter_497(a, state0.b, arg0, arg1);
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

module.exports.__wbg_resolve_b0083a7967828ec8 = function(arg0) {
    const ret = Promise.resolve(getObject(arg0));
    return addHeapObject(ret);
};

module.exports.__wbg_then_0c86a60e8fcfe9f6 = function(arg0, arg1) {
    const ret = getObject(arg0).then(getObject(arg1));
    return addHeapObject(ret);
};

module.exports.__wbg_then_a73caa9a87991566 = function(arg0, arg1, arg2) {
    const ret = getObject(arg0).then(getObject(arg1), getObject(arg2));
    return addHeapObject(ret);
};

module.exports.__wbindgen_debug_string = function(arg0, arg1) {
    const ret = debugString(getObject(arg1));
    const ptr1 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    const len1 = WASM_VECTOR_LEN;
    getInt32Memory0()[arg0 / 4 + 1] = len1;
    getInt32Memory0()[arg0 / 4 + 0] = ptr1;
};

module.exports.__wbindgen_throw = function(arg0, arg1) {
    throw new Error(getStringFromWasm0(arg0, arg1));
};

module.exports.__wbindgen_closure_wrapper970 = function(arg0, arg1, arg2) {
    const ret = makeClosure(arg0, arg1, 117, __wbg_adapter_28);
    return addHeapObject(ret);
};

module.exports.__wbindgen_closure_wrapper972 = function(arg0, arg1, arg2) {
    const ret = makeClosure(arg0, arg1, 117, __wbg_adapter_31);
    return addHeapObject(ret);
};

module.exports.__wbindgen_closure_wrapper1172 = function(arg0, arg1, arg2) {
    const ret = makeMutClosure(arg0, arg1, 152, __wbg_adapter_34);
    return addHeapObject(ret);
};

const path = require('path').join(__dirname, 'mathlikeanim_rs_bg.wasm');
const bytes = require('fs').readFileSync(path);

const wasmModule = new WebAssembly.Module(bytes);
const wasmInstance = new WebAssembly.Instance(wasmModule, imports);
wasm = wasmInstance.exports;
module.exports.__wasm = wasm;

