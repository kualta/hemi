import { RawInterpreter } from './snippets/dioxus-interpreter-js-7e2aed97ebee2c55/inline0.js';
import { setAttributeInner } from './snippets/dioxus-interpreter-js-7e2aed97ebee2c55/src/js/common.js';
import { WebDioxusChannel } from './snippets/dioxus-interpreter-js-7e2aed97ebee2c55/src/js/eval.js';
import { get_form_data } from './snippets/dioxus-web-10186f9fcc0b4418/inline0.js';
import { get_select_data } from './snippets/dioxus-web-10186f9fcc0b4418/inline1.js';

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

function addHeapObject(obj) {
    if (heap_next === heap.length) heap.push(heap.length + 1);
    const idx = heap_next;
    heap_next = heap[idx];

    if (typeof(heap_next) !== 'number') throw new Error('corrupt heap');

    heap[idx] = obj;
    return idx;
}

const cachedTextDecoder = (typeof TextDecoder !== 'undefined' ? new TextDecoder('utf-8', { ignoreBOM: true, fatal: true }) : { decode: () => { throw Error('TextDecoder not available') } } );

if (typeof TextDecoder !== 'undefined') { cachedTextDecoder.decode(); };

let cachedUint8ArrayMemory0 = null;

function getUint8ArrayMemory0() {
    if (cachedUint8ArrayMemory0 === null || cachedUint8ArrayMemory0.byteLength === 0) {
        cachedUint8ArrayMemory0 = new Uint8Array(wasm.memory.buffer);
    }
    return cachedUint8ArrayMemory0;
}

function getStringFromWasm0(ptr, len) {
    ptr = ptr >>> 0;
    return cachedTextDecoder.decode(getUint8ArrayMemory0().subarray(ptr, ptr + len));
}

function _assertBoolean(n) {
    if (typeof(n) !== 'boolean') {
        throw new Error(`expected a boolean argument, found ${typeof(n)}`);
    }
}

function _assertNum(n) {
    if (typeof(n) !== 'number') throw new Error(`expected a number argument, found ${typeof(n)}`);
}

function isLikeNone(x) {
    return x === undefined || x === null;
}

let cachedDataViewMemory0 = null;

function getDataViewMemory0() {
    if (cachedDataViewMemory0 === null || cachedDataViewMemory0.buffer.detached === true || (cachedDataViewMemory0.buffer.detached === undefined && cachedDataViewMemory0.buffer !== wasm.memory.buffer)) {
        cachedDataViewMemory0 = new DataView(wasm.memory.buffer);
    }
    return cachedDataViewMemory0;
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

    if (typeof(arg) !== 'string') throw new Error(`expected a string argument, found ${typeof(arg)}`);

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
        const ret = encodeString(arg, view);
        if (ret.read !== arg.length) throw new Error('failed to pass whole string');
        offset += ret.written;
        ptr = realloc(ptr, len, offset, 1) >>> 0;
    }

    WASM_VECTOR_LEN = offset;
    return ptr;
}

function _assertBigInt(n) {
    if (typeof(n) !== 'bigint') throw new Error(`expected a bigint argument, found ${typeof(n)}`);
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

function logError(f, args) {
    try {
        return f.apply(this, args);
    } catch (e) {
        let error = (function () {
            try {
                return e instanceof Error ? `${e.message}\n\nStack:\n${e.stack}` : e.toString();
            } catch(_) {
                return "<failed to stringify thrown value>";
            }
        }());
        console.error("wasm-bindgen: imported JS function that was not marked as `catch` threw an error:", error);
        throw e;
    }
}

let stack_pointer = 128;

function addBorrowedObject(obj) {
    if (stack_pointer == 1) throw new Error('out of js stack');
    heap[--stack_pointer] = obj;
    return stack_pointer;
}
function __wbg_adapter_48(arg0, arg1, arg2) {
    try {
        _assertNum(arg0);
        _assertNum(arg1);
        wasm._dyn_core__ops__function__FnMut___A____Output___R_as_wasm_bindgen__closure__WasmClosure___describe__invoke__h06d8769790dc4c09(arg0, arg1, addBorrowedObject(arg2));
    } finally {
        heap[stack_pointer++] = undefined;
    }
}

function __wbg_adapter_51(arg0, arg1) {
    _assertNum(arg0);
    _assertNum(arg1);
    wasm._dyn_core__ops__function__FnMut_____Output___R_as_wasm_bindgen__closure__WasmClosure___describe__invoke__he0f070d056c1b759(arg0, arg1);
}

function __wbg_adapter_54(arg0, arg1, arg2) {
    _assertNum(arg0);
    _assertNum(arg1);
    wasm._dyn_core__ops__function__FnMut__A____Output___R_as_wasm_bindgen__closure__WasmClosure___describe__invoke__hc718de9de01398c8(arg0, arg1, addHeapObject(arg2));
}

function getArrayJsValueFromWasm0(ptr, len) {
    ptr = ptr >>> 0;
    const mem = getDataViewMemory0();
    const result = [];
    for (let i = ptr; i < ptr + 4 * len; i += 4) {
        result.push(takeObject(mem.getUint32(i, true)));
    }
    return result;
}

function passArrayJsValueToWasm0(array, malloc) {
    const ptr = malloc(array.length * 4, 4) >>> 0;
    const mem = getDataViewMemory0();
    for (let i = 0; i < array.length; i++) {
        mem.setUint32(ptr + 4 * i, addHeapObject(array[i]), true);
    }
    WASM_VECTOR_LEN = array.length;
    return ptr;
}

function handleError(f, args) {
    try {
        return f.apply(this, args);
    } catch (e) {
        wasm.__wbindgen_exn_store(addHeapObject(e));
    }
}

const __wbindgen_enum_RequestCredentials = ["omit", "same-origin", "include"];

const __wbindgen_enum_RequestMode = ["same-origin", "no-cors", "cors", "navigate"];

const __wbindgen_enum_ScrollBehavior = ["auto", "instant", "smooth"];

const __wbindgen_enum_ScrollLogicalPosition = ["start", "center", "end", "nearest"];

const JSOwnerFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_jsowner_free(ptr >>> 0, 1));

export class JSOwner {

    constructor() {
        throw new Error('cannot invoke `new` directly');
    }

    static __wrap(ptr) {
        ptr = ptr >>> 0;
        const obj = Object.create(JSOwner.prototype);
        obj.__wbg_ptr = ptr;
        JSOwnerFinalization.register(obj, obj.__wbg_ptr, obj);
        return obj;
    }

    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        JSOwnerFinalization.unregister(this);
        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_jsowner_free(ptr, 0);
    }
}

async function __wbg_load(module, imports) {
    if (typeof Response === 'function' && module instanceof Response) {
        if (typeof WebAssembly.instantiateStreaming === 'function') {
            try {
                return await WebAssembly.instantiateStreaming(module, imports);

            } catch (e) {
                if (module.headers.get('Content-Type') != 'application/wasm') {
                    console.warn("`WebAssembly.instantiateStreaming` failed because your server does not serve Wasm with `application/wasm` MIME type. Falling back to `WebAssembly.instantiate` which is slower. Original error:\n", e);

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
    imports.wbg.__wbg_newwithsrc_6f3688e85a51b721 = function() { return handleError(function (arg0, arg1) {
        const ret = new Audio(getStringFromWasm0(arg0, arg1));
        return addHeapObject(ret);
    }, arguments) };
    imports.wbg.__wbg_play_789f1fa6482cff25 = function() { return handleError(function (arg0) {
        const ret = getObject(arg0).play();
        return addHeapObject(ret);
    }, arguments) };
    imports.wbg.__wbindgen_object_drop_ref = function(arg0) {
        takeObject(arg0);
    };
    imports.wbg.__wbg_new_abda76e883ba8a5f = function() { return logError(function () {
        const ret = new Error();
        return addHeapObject(ret);
    }, arguments) };
    imports.wbg.__wbg_stack_658279fe44541cf6 = function() { return logError(function (arg0, arg1) {
        const ret = getObject(arg1).stack;
        const ptr1 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len1 = WASM_VECTOR_LEN;
        getDataViewMemory0().setInt32(arg0 + 4 * 1, len1, true);
        getDataViewMemory0().setInt32(arg0 + 4 * 0, ptr1, true);
    }, arguments) };
    imports.wbg.__wbg_error_f851667af71bcfc6 = function() { return logError(function (arg0, arg1) {
        let deferred0_0;
        let deferred0_1;
        try {
            deferred0_0 = arg0;
            deferred0_1 = arg1;
            console.error(getStringFromWasm0(arg0, arg1));
        } finally {
            wasm.__wbindgen_free(deferred0_0, deferred0_1, 1);
        }
    }, arguments) };
    imports.wbg.__wbg_deltaMode_f31810d86a9defec = function() { return logError(function (arg0) {
        const ret = getObject(arg0).deltaMode;
        _assertNum(ret);
        return ret;
    }, arguments) };
    imports.wbg.__wbg_deltaX_10154f810008c0a0 = function() { return logError(function (arg0) {
        const ret = getObject(arg0).deltaX;
        return ret;
    }, arguments) };
    imports.wbg.__wbg_deltaY_afd77a1b9e0d9ccd = function() { return logError(function (arg0) {
        const ret = getObject(arg0).deltaY;
        return ret;
    }, arguments) };
    imports.wbg.__wbg_deltaZ_ec44501c143f6d88 = function() { return logError(function (arg0) {
        const ret = getObject(arg0).deltaZ;
        return ret;
    }, arguments) };
    imports.wbg.__wbg_data_86e77dc14916d155 = function() { return logError(function (arg0, arg1) {
        const ret = getObject(arg1).data;
        var ptr1 = isLikeNone(ret) ? 0 : passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        var len1 = WASM_VECTOR_LEN;
        getDataViewMemory0().setInt32(arg0 + 4 * 1, len1, true);
        getDataViewMemory0().setInt32(arg0 + 4 * 0, ptr1, true);
    }, arguments) };
    imports.wbg.__wbg_animationName_841417bb5df8825f = function() { return logError(function (arg0, arg1) {
        const ret = getObject(arg1).animationName;
        const ptr1 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len1 = WASM_VECTOR_LEN;
        getDataViewMemory0().setInt32(arg0 + 4 * 1, len1, true);
        getDataViewMemory0().setInt32(arg0 + 4 * 0, ptr1, true);
    }, arguments) };
    imports.wbg.__wbg_pseudoElement_b2234158091018ae = function() { return logError(function (arg0, arg1) {
        const ret = getObject(arg1).pseudoElement;
        const ptr1 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len1 = WASM_VECTOR_LEN;
        getDataViewMemory0().setInt32(arg0 + 4 * 1, len1, true);
        getDataViewMemory0().setInt32(arg0 + 4 * 0, ptr1, true);
    }, arguments) };
    imports.wbg.__wbg_elapsedTime_2a54081e9269631d = function() { return logError(function (arg0) {
        const ret = getObject(arg0).elapsedTime;
        return ret;
    }, arguments) };
    imports.wbg.__wbg_getBoundingClientRect_5ad16be1e2955e83 = function() { return logError(function (arg0) {
        const ret = getObject(arg0).getBoundingClientRect();
        return addHeapObject(ret);
    }, arguments) };
    imports.wbg.__wbg_left_324ad4ce0086311f = function() { return logError(function (arg0) {
        const ret = getObject(arg0).left;
        return ret;
    }, arguments) };
    imports.wbg.__wbg_top_5f4586313f3e086f = function() { return logError(function (arg0) {
        const ret = getObject(arg0).top;
        return ret;
    }, arguments) };
    imports.wbg.__wbg_width_28175f04c07458aa = function() { return logError(function (arg0) {
        const ret = getObject(arg0).width;
        return ret;
    }, arguments) };
    imports.wbg.__wbg_height_dbd0616ae39a99b1 = function() { return logError(function (arg0) {
        const ret = getObject(arg0).height;
        return ret;
    }, arguments) };
    imports.wbg.__wbg_new_e69b5f66fda8f13c = function() { return logError(function () {
        const ret = new Object();
        return addHeapObject(ret);
    }, arguments) };
    imports.wbg.__wbg_setbehavior_e58c14ac43ed56a1 = function() { return logError(function (arg0, arg1) {
        getObject(arg0).behavior = __wbindgen_enum_ScrollBehavior[arg1];
    }, arguments) };
    imports.wbg.__wbg_scrollIntoView_006062858903bbd0 = function() { return logError(function (arg0, arg1) {
        getObject(arg0).scrollIntoView(getObject(arg1));
    }, arguments) };
    imports.wbg.__wbg_instanceof_HtmlElement_aab18e065dc9207d = function() { return logError(function (arg0) {
        let result;
        try {
            result = getObject(arg0) instanceof HTMLElement;
        } catch (_) {
            result = false;
        }
        const ret = result;
        _assertBoolean(ret);
        return ret;
    }, arguments) };
    imports.wbg.__wbindgen_object_clone_ref = function(arg0) {
        const ret = getObject(arg0);
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_blur_d7e0bcc31c40e996 = function() { return handleError(function (arg0) {
        getObject(arg0).blur();
    }, arguments) };
    imports.wbg.__wbg_focus_6b6181f7644f6dbc = function() { return handleError(function (arg0) {
        getObject(arg0).focus();
    }, arguments) };
    imports.wbg.__wbg_elapsedTime_476a4e01c4f63fda = function() { return logError(function (arg0) {
        const ret = getObject(arg0).elapsedTime;
        return ret;
    }, arguments) };
    imports.wbg.__wbg_propertyName_44ca202b08d008f3 = function() { return logError(function (arg0, arg1) {
        const ret = getObject(arg1).propertyName;
        const ptr1 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len1 = WASM_VECTOR_LEN;
        getDataViewMemory0().setInt32(arg0 + 4 * 1, len1, true);
        getDataViewMemory0().setInt32(arg0 + 4 * 0, ptr1, true);
    }, arguments) };
    imports.wbg.__wbg_pseudoElement_f3ba1319b33578fa = function() { return logError(function (arg0, arg1) {
        const ret = getObject(arg1).pseudoElement;
        const ptr1 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len1 = WASM_VECTOR_LEN;
        getDataViewMemory0().setInt32(arg0 + 4 * 1, len1, true);
        getDataViewMemory0().setInt32(arg0 + 4 * 0, ptr1, true);
    }, arguments) };
    imports.wbg.__wbg_identifier_b858c904e1c72507 = function() { return logError(function (arg0) {
        const ret = getObject(arg0).identifier;
        _assertNum(ret);
        return ret;
    }, arguments) };
    imports.wbg.__wbg_radiusX_f00767113c0e51ea = function() { return logError(function (arg0) {
        const ret = getObject(arg0).radiusX;
        _assertNum(ret);
        return ret;
    }, arguments) };
    imports.wbg.__wbg_radiusY_c4043b226e03d720 = function() { return logError(function (arg0) {
        const ret = getObject(arg0).radiusY;
        _assertNum(ret);
        return ret;
    }, arguments) };
    imports.wbg.__wbg_rotationAngle_e35c1b560312ec61 = function() { return logError(function (arg0) {
        const ret = getObject(arg0).rotationAngle;
        return ret;
    }, arguments) };
    imports.wbg.__wbg_force_5af67f6cd0b9c097 = function() { return logError(function (arg0) {
        const ret = getObject(arg0).force;
        return ret;
    }, arguments) };
    imports.wbg.__wbg_clientX_0e075d664eb70517 = function() { return logError(function (arg0) {
        const ret = getObject(arg0).clientX;
        _assertNum(ret);
        return ret;
    }, arguments) };
    imports.wbg.__wbg_clientY_32b24b7be6b2e79d = function() { return logError(function (arg0) {
        const ret = getObject(arg0).clientY;
        _assertNum(ret);
        return ret;
    }, arguments) };
    imports.wbg.__wbg_screenX_66fdb34b7f1552ac = function() { return logError(function (arg0) {
        const ret = getObject(arg0).screenX;
        _assertNum(ret);
        return ret;
    }, arguments) };
    imports.wbg.__wbg_screenY_0949c88f98db641e = function() { return logError(function (arg0) {
        const ret = getObject(arg0).screenY;
        _assertNum(ret);
        return ret;
    }, arguments) };
    imports.wbg.__wbg_pageX_f570d523d89c16ec = function() { return logError(function (arg0) {
        const ret = getObject(arg0).pageX;
        _assertNum(ret);
        return ret;
    }, arguments) };
    imports.wbg.__wbg_pageY_ff077f56016c03aa = function() { return logError(function (arg0) {
        const ret = getObject(arg0).pageY;
        _assertNum(ret);
        return ret;
    }, arguments) };
    imports.wbg.__wbg_key_001eb20ba3b3d2fd = function() { return logError(function (arg0, arg1) {
        const ret = getObject(arg1).key;
        const ptr1 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len1 = WASM_VECTOR_LEN;
        getDataViewMemory0().setInt32(arg0 + 4 * 1, len1, true);
        getDataViewMemory0().setInt32(arg0 + 4 * 0, ptr1, true);
    }, arguments) };
    imports.wbg.__wbg_code_bec0d5222298000e = function() { return logError(function (arg0, arg1) {
        const ret = getObject(arg1).code;
        const ptr1 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len1 = WASM_VECTOR_LEN;
        getDataViewMemory0().setInt32(arg0 + 4 * 1, len1, true);
        getDataViewMemory0().setInt32(arg0 + 4 * 0, ptr1, true);
    }, arguments) };
    imports.wbg.__wbg_location_a7e2614c5720fcd7 = function() { return logError(function (arg0) {
        const ret = getObject(arg0).location;
        _assertNum(ret);
        return ret;
    }, arguments) };
    imports.wbg.__wbg_repeat_1f81f308f5d8d519 = function() { return logError(function (arg0) {
        const ret = getObject(arg0).repeat;
        _assertBoolean(ret);
        return ret;
    }, arguments) };
    imports.wbg.__wbg_isComposing_527cd20b8f4c31d2 = function() { return logError(function (arg0) {
        const ret = getObject(arg0).isComposing;
        _assertBoolean(ret);
        return ret;
    }, arguments) };
    imports.wbg.__wbg_altKey_ebf03e2308f51c08 = function() { return logError(function (arg0) {
        const ret = getObject(arg0).altKey;
        _assertBoolean(ret);
        return ret;
    }, arguments) };
    imports.wbg.__wbg_ctrlKey_f592192d87040d94 = function() { return logError(function (arg0) {
        const ret = getObject(arg0).ctrlKey;
        _assertBoolean(ret);
        return ret;
    }, arguments) };
    imports.wbg.__wbg_metaKey_0735ca81e2ec6c72 = function() { return logError(function (arg0) {
        const ret = getObject(arg0).metaKey;
        _assertBoolean(ret);
        return ret;
    }, arguments) };
    imports.wbg.__wbg_shiftKey_cb120edc9c25950d = function() { return logError(function (arg0) {
        const ret = getObject(arg0).shiftKey;
        _assertBoolean(ret);
        return ret;
    }, arguments) };
    imports.wbg.__wbg_clientX_a8eebf094c107e43 = function() { return logError(function (arg0) {
        const ret = getObject(arg0).clientX;
        _assertNum(ret);
        return ret;
    }, arguments) };
    imports.wbg.__wbg_clientY_ffe0a79af8089cd4 = function() { return logError(function (arg0) {
        const ret = getObject(arg0).clientY;
        _assertNum(ret);
        return ret;
    }, arguments) };
    imports.wbg.__wbg_pageX_163dc6047071b51f = function() { return logError(function (arg0) {
        const ret = getObject(arg0).pageX;
        _assertNum(ret);
        return ret;
    }, arguments) };
    imports.wbg.__wbg_pageY_302e6265933ebb59 = function() { return logError(function (arg0) {
        const ret = getObject(arg0).pageY;
        _assertNum(ret);
        return ret;
    }, arguments) };
    imports.wbg.__wbg_screenX_a30d4e116ae70c94 = function() { return logError(function (arg0) {
        const ret = getObject(arg0).screenX;
        _assertNum(ret);
        return ret;
    }, arguments) };
    imports.wbg.__wbg_screenY_8325b64f4724a798 = function() { return logError(function (arg0) {
        const ret = getObject(arg0).screenY;
        _assertNum(ret);
        return ret;
    }, arguments) };
    imports.wbg.__wbg_offsetX_79b2d23b78682ab7 = function() { return logError(function (arg0) {
        const ret = getObject(arg0).offsetX;
        _assertNum(ret);
        return ret;
    }, arguments) };
    imports.wbg.__wbg_offsetY_39cb724403a8302f = function() { return logError(function (arg0) {
        const ret = getObject(arg0).offsetY;
        _assertNum(ret);
        return ret;
    }, arguments) };
    imports.wbg.__wbg_altKey_c9401b041949ea90 = function() { return logError(function (arg0) {
        const ret = getObject(arg0).altKey;
        _assertBoolean(ret);
        return ret;
    }, arguments) };
    imports.wbg.__wbg_ctrlKey_4015247a39aa9410 = function() { return logError(function (arg0) {
        const ret = getObject(arg0).ctrlKey;
        _assertBoolean(ret);
        return ret;
    }, arguments) };
    imports.wbg.__wbg_metaKey_5d680933661ea1ea = function() { return logError(function (arg0) {
        const ret = getObject(arg0).metaKey;
        _assertBoolean(ret);
        return ret;
    }, arguments) };
    imports.wbg.__wbg_shiftKey_6d843f3032fd0366 = function() { return logError(function (arg0) {
        const ret = getObject(arg0).shiftKey;
        _assertBoolean(ret);
        return ret;
    }, arguments) };
    imports.wbg.__wbg_buttons_2cb9e49b40e20105 = function() { return logError(function (arg0) {
        const ret = getObject(arg0).buttons;
        _assertNum(ret);
        return ret;
    }, arguments) };
    imports.wbg.__wbg_button_d8226b772c8cf494 = function() { return logError(function (arg0) {
        const ret = getObject(arg0).button;
        _assertNum(ret);
        return ret;
    }, arguments) };
    imports.wbg.__wbg_altKey_e0ebf3eabcb13e08 = function() { return logError(function (arg0) {
        const ret = getObject(arg0).altKey;
        _assertBoolean(ret);
        return ret;
    }, arguments) };
    imports.wbg.__wbg_ctrlKey_606cbe2c4322ed56 = function() { return logError(function (arg0) {
        const ret = getObject(arg0).ctrlKey;
        _assertBoolean(ret);
        return ret;
    }, arguments) };
    imports.wbg.__wbg_metaKey_3b977a6e61a731d7 = function() { return logError(function (arg0) {
        const ret = getObject(arg0).metaKey;
        _assertBoolean(ret);
        return ret;
    }, arguments) };
    imports.wbg.__wbg_shiftKey_863ca71f9f2722ab = function() { return logError(function (arg0) {
        const ret = getObject(arg0).shiftKey;
        _assertBoolean(ret);
        return ret;
    }, arguments) };
    imports.wbg.__wbg_touches_092e96ce3221acbc = function() { return logError(function (arg0) {
        const ret = getObject(arg0).touches;
        return addHeapObject(ret);
    }, arguments) };
    imports.wbg.__wbg_length_1b6ac4894265d4e6 = function() { return logError(function (arg0) {
        const ret = getObject(arg0).length;
        _assertNum(ret);
        return ret;
    }, arguments) };
    imports.wbg.__wbg_get_4d863ed1d42a2b7d = function() { return logError(function (arg0, arg1) {
        const ret = getObject(arg0)[arg1 >>> 0];
        return isLikeNone(ret) ? 0 : addHeapObject(ret);
    }, arguments) };
    imports.wbg.__wbg_changedTouches_ee3dabea7d95ebf2 = function() { return logError(function (arg0) {
        const ret = getObject(arg0).changedTouches;
        return addHeapObject(ret);
    }, arguments) };
    imports.wbg.__wbg_targetTouches_faffde5127036c13 = function() { return logError(function (arg0) {
        const ret = getObject(arg0).targetTouches;
        return addHeapObject(ret);
    }, arguments) };
    imports.wbg.__wbg_pointerId_93f7e5e10bb641ad = function() { return logError(function (arg0) {
        const ret = getObject(arg0).pointerId;
        _assertNum(ret);
        return ret;
    }, arguments) };
    imports.wbg.__wbg_width_e219d480687cf6e6 = function() { return logError(function (arg0) {
        const ret = getObject(arg0).width;
        _assertNum(ret);
        return ret;
    }, arguments) };
    imports.wbg.__wbg_height_43c0ad624a17f405 = function() { return logError(function (arg0) {
        const ret = getObject(arg0).height;
        _assertNum(ret);
        return ret;
    }, arguments) };
    imports.wbg.__wbg_pressure_ad8dacbd14c9076f = function() { return logError(function (arg0) {
        const ret = getObject(arg0).pressure;
        return ret;
    }, arguments) };
    imports.wbg.__wbg_tangentialPressure_a096181c7325f997 = function() { return logError(function (arg0) {
        const ret = getObject(arg0).tangentialPressure;
        return ret;
    }, arguments) };
    imports.wbg.__wbg_tiltX_d85abdd3d6e11865 = function() { return logError(function (arg0) {
        const ret = getObject(arg0).tiltX;
        _assertNum(ret);
        return ret;
    }, arguments) };
    imports.wbg.__wbg_tiltY_c890264354ac05d2 = function() { return logError(function (arg0) {
        const ret = getObject(arg0).tiltY;
        _assertNum(ret);
        return ret;
    }, arguments) };
    imports.wbg.__wbg_twist_7843b7e5e0e2d69d = function() { return logError(function (arg0) {
        const ret = getObject(arg0).twist;
        _assertNum(ret);
        return ret;
    }, arguments) };
    imports.wbg.__wbg_pointerType_6d91ef0da43639d3 = function() { return logError(function (arg0, arg1) {
        const ret = getObject(arg1).pointerType;
        const ptr1 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len1 = WASM_VECTOR_LEN;
        getDataViewMemory0().setInt32(arg0 + 4 * 1, len1, true);
        getDataViewMemory0().setInt32(arg0 + 4 * 0, ptr1, true);
    }, arguments) };
    imports.wbg.__wbg_isPrimary_2ee404d1f136ff46 = function() { return logError(function (arg0) {
        const ret = getObject(arg0).isPrimary;
        _assertBoolean(ret);
        return ret;
    }, arguments) };
    imports.wbg.__wbg_type_739ef24b64f58229 = function() { return logError(function (arg0, arg1) {
        const ret = getObject(arg1).type;
        const ptr1 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len1 = WASM_VECTOR_LEN;
        getDataViewMemory0().setInt32(arg0 + 4 * 1, len1, true);
        getDataViewMemory0().setInt32(arg0 + 4 * 0, ptr1, true);
    }, arguments) };
    imports.wbg.__wbg_target_b0499015ea29563d = function() { return logError(function (arg0) {
        const ret = getObject(arg0).target;
        return isLikeNone(ret) ? 0 : addHeapObject(ret);
    }, arguments) };
    imports.wbg.__wbg_instanceof_Node_db422d75160b3c20 = function() { return logError(function (arg0) {
        let result;
        try {
            result = getObject(arg0) instanceof Node;
        } catch (_) {
            result = false;
        }
        const ret = result;
        _assertBoolean(ret);
        return ret;
    }, arguments) };
    imports.wbg.__wbg_instanceof_Element_1a81366cc90e70e2 = function() { return logError(function (arg0) {
        let result;
        try {
            result = getObject(arg0) instanceof Element;
        } catch (_) {
            result = false;
        }
        const ret = result;
        _assertBoolean(ret);
        return ret;
    }, arguments) };
    imports.wbg.__wbg_getAttribute_8ac49f4186f4cefd = function() { return logError(function (arg0, arg1, arg2, arg3) {
        const ret = getObject(arg1).getAttribute(getStringFromWasm0(arg2, arg3));
        var ptr1 = isLikeNone(ret) ? 0 : passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        var len1 = WASM_VECTOR_LEN;
        getDataViewMemory0().setInt32(arg0 + 4 * 1, len1, true);
        getDataViewMemory0().setInt32(arg0 + 4 * 0, ptr1, true);
    }, arguments) };
    imports.wbg.__wbg_parentElement_bf013e6093029477 = function() { return logError(function (arg0) {
        const ret = getObject(arg0).parentElement;
        return isLikeNone(ret) ? 0 : addHeapObject(ret);
    }, arguments) };
    imports.wbg.__wbg_bubbles_c48a1056384e852c = function() { return logError(function (arg0) {
        const ret = getObject(arg0).bubbles;
        _assertBoolean(ret);
        return ret;
    }, arguments) };
    imports.wbg.__wbg_preventDefault_eecc4a63e64c4526 = function() { return logError(function (arg0) {
        getObject(arg0).preventDefault();
    }, arguments) };
    imports.wbg.__wbindgen_memory = function() {
        const ret = wasm.memory;
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_updatememory_0fc68f206c0463de = function() { return logError(function (arg0, arg1) {
        getObject(arg0).update_memory(takeObject(arg1));
    }, arguments) };
    imports.wbg.__wbg_run_47308773c14de567 = function() { return logError(function (arg0) {
        getObject(arg0).run();
    }, arguments) };
    imports.wbg.__wbg_getNode_d4db5a1d3bc15eae = function() { return logError(function (arg0, arg1) {
        const ret = getObject(arg0).getNode(arg1 >>> 0);
        return addHeapObject(ret);
    }, arguments) };
    imports.wbg.__wbg_saveTemplate_e7948f8b7f54cfc9 = function() { return logError(function (arg0, arg1, arg2, arg3) {
        var v0 = getArrayJsValueFromWasm0(arg1, arg2).slice();
        wasm.__wbindgen_free(arg1, arg2 * 4, 4);
        getObject(arg0).saveTemplate(v0, arg3);
    }, arguments) };
    imports.wbg.__wbg_createTextNode_3b33c97f8ef3e999 = function() { return logError(function (arg0, arg1, arg2) {
        const ret = getObject(arg0).createTextNode(getStringFromWasm0(arg1, arg2));
        return addHeapObject(ret);
    }, arguments) };
    imports.wbg.__wbg_createElement_e4523490bd0ae51d = function() { return handleError(function (arg0, arg1, arg2) {
        const ret = getObject(arg0).createElement(getStringFromWasm0(arg1, arg2));
        return addHeapObject(ret);
    }, arguments) };
    imports.wbg.__wbg_createElementNS_e51a368ab3a64b37 = function() { return handleError(function (arg0, arg1, arg2, arg3, arg4) {
        const ret = getObject(arg0).createElementNS(arg1 === 0 ? undefined : getStringFromWasm0(arg1, arg2), getStringFromWasm0(arg3, arg4));
        return addHeapObject(ret);
    }, arguments) };
    imports.wbg.__wbg_toggleAttribute_dbc6a90ae90527b3 = function() { return handleError(function (arg0, arg1, arg2) {
        const ret = getObject(arg0).toggleAttribute(getStringFromWasm0(arg1, arg2));
        _assertBoolean(ret);
        return ret;
    }, arguments) };
    imports.wbg.__wbindgen_string_new = function(arg0, arg1) {
        const ret = getStringFromWasm0(arg0, arg1);
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_setAttributeInner_2820d9f570125088 = function() { return logError(function (arg0, arg1, arg2, arg3, arg4, arg5) {
        setAttributeInner(takeObject(arg0), getStringFromWasm0(arg1, arg2), takeObject(arg3), arg4 === 0 ? undefined : getStringFromWasm0(arg4, arg5));
    }, arguments) };
    imports.wbg.__wbg_appendChild_bc4a0deae90a5164 = function() { return handleError(function (arg0, arg1) {
        const ret = getObject(arg0).appendChild(getObject(arg1));
        return addHeapObject(ret);
    }, arguments) };
    imports.wbg.__wbindgen_jsval_loose_eq = function(arg0, arg1) {
        const ret = getObject(arg0) == getObject(arg1);
        _assertBoolean(ret);
        return ret;
    };
    imports.wbg.__wbindgen_boolean_get = function(arg0) {
        const v = getObject(arg0);
        const ret = typeof(v) === 'boolean' ? (v ? 1 : 0) : 2;
        _assertNum(ret);
        return ret;
    };
    imports.wbg.__wbindgen_is_bigint = function(arg0) {
        const ret = typeof(getObject(arg0)) === 'bigint';
        _assertBoolean(ret);
        return ret;
    };
    imports.wbg.__wbindgen_number_get = function(arg0, arg1) {
        const obj = getObject(arg1);
        const ret = typeof(obj) === 'number' ? obj : undefined;
        if (!isLikeNone(ret)) {
            _assertNum(ret);
        }
        getDataViewMemory0().setFloat64(arg0 + 8 * 1, isLikeNone(ret) ? 0 : ret, true);
        getDataViewMemory0().setInt32(arg0 + 4 * 0, !isLikeNone(ret), true);
    };
    imports.wbg.__wbindgen_string_get = function(arg0, arg1) {
        const obj = getObject(arg1);
        const ret = typeof(obj) === 'string' ? obj : undefined;
        var ptr1 = isLikeNone(ret) ? 0 : passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        var len1 = WASM_VECTOR_LEN;
        getDataViewMemory0().setInt32(arg0 + 4 * 1, len1, true);
        getDataViewMemory0().setInt32(arg0 + 4 * 0, ptr1, true);
    };
    imports.wbg.__wbg_isArray_6f3b47f09adb61b5 = function() { return logError(function (arg0) {
        const ret = Array.isArray(getObject(arg0));
        _assertBoolean(ret);
        return ret;
    }, arguments) };
    imports.wbg.__wbg_isSafeInteger_b9dff570f01a9100 = function() { return logError(function (arg0) {
        const ret = Number.isSafeInteger(getObject(arg0));
        _assertBoolean(ret);
        return ret;
    }, arguments) };
    imports.wbg.__wbindgen_is_object = function(arg0) {
        const val = getObject(arg0);
        const ret = typeof(val) === 'object' && val !== null;
        _assertBoolean(ret);
        return ret;
    };
    imports.wbg.__wbg_iterator_695d699a44d6234c = function() { return logError(function () {
        const ret = Symbol.iterator;
        return addHeapObject(ret);
    }, arguments) };
    imports.wbg.__wbindgen_in = function(arg0, arg1) {
        const ret = getObject(arg0) in getObject(arg1);
        _assertBoolean(ret);
        return ret;
    };
    imports.wbg.__wbindgen_bigint_get_as_i64 = function(arg0, arg1) {
        const v = getObject(arg1);
        const ret = typeof(v) === 'bigint' ? v : undefined;
        if (!isLikeNone(ret)) {
            _assertBigInt(ret);
        }
        getDataViewMemory0().setBigInt64(arg0 + 8 * 1, isLikeNone(ret) ? BigInt(0) : ret, true);
        getDataViewMemory0().setInt32(arg0 + 4 * 0, !isLikeNone(ret), true);
    };
    imports.wbg.__wbindgen_bigint_from_i64 = function(arg0) {
        const ret = arg0;
        return addHeapObject(ret);
    };
    imports.wbg.__wbindgen_jsval_eq = function(arg0, arg1) {
        const ret = getObject(arg0) === getObject(arg1);
        _assertBoolean(ret);
        return ret;
    };
    imports.wbg.__wbindgen_bigint_from_u64 = function(arg0) {
        const ret = BigInt.asUintN(64, arg0);
        return addHeapObject(ret);
    };
    imports.wbg.__wbindgen_error_new = function(arg0, arg1) {
        const ret = new Error(getStringFromWasm0(arg0, arg1));
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_length_f217bbbf7e8e4df4 = function() { return logError(function (arg0) {
        const ret = getObject(arg0).length;
        _assertNum(ret);
        return ret;
    }, arguments) };
    imports.wbg.__wbg_get_5419cf6b954aa11d = function() { return logError(function (arg0, arg1) {
        const ret = getObject(arg0)[arg1 >>> 0];
        return addHeapObject(ret);
    }, arguments) };
    imports.wbg.__wbg_next_b06e115d1b01e10b = function() { return handleError(function (arg0) {
        const ret = getObject(arg0).next();
        return addHeapObject(ret);
    }, arguments) };
    imports.wbg.__wbg_done_983b5ffcaec8c583 = function() { return logError(function (arg0) {
        const ret = getObject(arg0).done;
        _assertBoolean(ret);
        return ret;
    }, arguments) };
    imports.wbg.__wbg_value_2ab8a198c834c26a = function() { return logError(function (arg0) {
        const ret = getObject(arg0).value;
        return addHeapObject(ret);
    }, arguments) };
    imports.wbg.__wbg_entries_c02034de337d3ee2 = function() { return logError(function (arg0) {
        const ret = Object.entries(getObject(arg0));
        return addHeapObject(ret);
    }, arguments) };
    imports.wbg.__wbg_length_21a3493916831b15 = function() { return logError(function (arg0) {
        const ret = getObject(arg0).length;
        _assertNum(ret);
        return ret;
    }, arguments) };
    imports.wbg.__wbg_item_e35a9206ab7dd263 = function() { return logError(function (arg0, arg1) {
        const ret = getObject(arg0).item(arg1 >>> 0);
        return isLikeNone(ret) ? 0 : addHeapObject(ret);
    }, arguments) };
    imports.wbg.__wbg_name_e30efb33291e0016 = function() { return logError(function (arg0, arg1) {
        const ret = getObject(arg1).name;
        const ptr1 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len1 = WASM_VECTOR_LEN;
        getDataViewMemory0().setInt32(arg0 + 4 * 1, len1, true);
        getDataViewMemory0().setInt32(arg0 + 4 * 0, ptr1, true);
    }, arguments) };
    imports.wbg.__wbg_setonload_0e9d43ec0cbb3987 = function() { return logError(function (arg0, arg1) {
        getObject(arg0).onload = getObject(arg1);
    }, arguments) };
    imports.wbg.__wbg_readAsArrayBuffer_467dfea5cb42f85c = function() { return handleError(function (arg0, arg1) {
        getObject(arg0).readAsArrayBuffer(getObject(arg1));
    }, arguments) };
    imports.wbg.__wbg_new_fec2611eb9180f95 = function() { return logError(function (arg0) {
        const ret = new Uint8Array(getObject(arg0));
        return addHeapObject(ret);
    }, arguments) };
    imports.wbg.__wbg_result_5cc84600fc64bf35 = function() { return handleError(function (arg0) {
        const ret = getObject(arg0).result;
        return addHeapObject(ret);
    }, arguments) };
    imports.wbg.__wbg_readAsText_abb4898a82a4815a = function() { return handleError(function (arg0, arg1) {
        getObject(arg0).readAsText(getObject(arg1));
    }, arguments) };
    imports.wbg.__wbg_instanceof_DragEvent_d060c9d7e145246e = function() { return logError(function (arg0) {
        let result;
        try {
            result = getObject(arg0) instanceof DragEvent;
        } catch (_) {
            result = false;
        }
        const ret = result;
        _assertBoolean(ret);
        return ret;
    }, arguments) };
    imports.wbg.__wbg_dataTransfer_b898af73237a967c = function() { return logError(function (arg0) {
        const ret = getObject(arg0).dataTransfer;
        return isLikeNone(ret) ? 0 : addHeapObject(ret);
    }, arguments) };
    imports.wbg.__wbg_files_194ca113571d995f = function() { return logError(function (arg0) {
        const ret = getObject(arg0).files;
        return isLikeNone(ret) ? 0 : addHeapObject(ret);
    }, arguments) };
    imports.wbg.__wbg_new_e282c42c5fc7a7b1 = function() { return handleError(function () {
        const ret = new FileReader();
        return addHeapObject(ret);
    }, arguments) };
    imports.wbg.__wbg_instanceof_HtmlInputElement_ee25196edbacced9 = function() { return logError(function (arg0) {
        let result;
        try {
            result = getObject(arg0) instanceof HTMLInputElement;
        } catch (_) {
            result = false;
        }
        const ret = result;
        _assertBoolean(ret);
        return ret;
    }, arguments) };
    imports.wbg.__wbg_files_5738c8732c2fc992 = function() { return logError(function (arg0) {
        const ret = getObject(arg0).files;
        return isLikeNone(ret) ? 0 : addHeapObject(ret);
    }, arguments) };
    imports.wbg.__wbg_type_0b40a977ba28a744 = function() { return logError(function (arg0, arg1) {
        const ret = getObject(arg1).type;
        const ptr1 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len1 = WASM_VECTOR_LEN;
        getDataViewMemory0().setInt32(arg0 + 4 * 1, len1, true);
        getDataViewMemory0().setInt32(arg0 + 4 * 0, ptr1, true);
    }, arguments) };
    imports.wbg.__wbg_checked_5c9846154b6119f6 = function() { return logError(function (arg0) {
        const ret = getObject(arg0).checked;
        _assertBoolean(ret);
        return ret;
    }, arguments) };
    imports.wbg.__wbg_value_0cffd86fb9a5a18d = function() { return logError(function (arg0, arg1) {
        const ret = getObject(arg1).value;
        const ptr1 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len1 = WASM_VECTOR_LEN;
        getDataViewMemory0().setInt32(arg0 + 4 * 1, len1, true);
        getDataViewMemory0().setInt32(arg0 + 4 * 0, ptr1, true);
    }, arguments) };
    imports.wbg.__wbg_instanceof_HtmlTextAreaElement_3d7305919124ce06 = function() { return logError(function (arg0) {
        let result;
        try {
            result = getObject(arg0) instanceof HTMLTextAreaElement;
        } catch (_) {
            result = false;
        }
        const ret = result;
        _assertBoolean(ret);
        return ret;
    }, arguments) };
    imports.wbg.__wbg_value_a8d0480de0da39cf = function() { return logError(function (arg0, arg1) {
        const ret = getObject(arg1).value;
        const ptr1 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len1 = WASM_VECTOR_LEN;
        getDataViewMemory0().setInt32(arg0 + 4 * 1, len1, true);
        getDataViewMemory0().setInt32(arg0 + 4 * 0, ptr1, true);
    }, arguments) };
    imports.wbg.__wbg_instanceof_HtmlSelectElement_66dfc08c717b1515 = function() { return logError(function (arg0) {
        let result;
        try {
            result = getObject(arg0) instanceof HTMLSelectElement;
        } catch (_) {
            result = false;
        }
        const ret = result;
        _assertBoolean(ret);
        return ret;
    }, arguments) };
    imports.wbg.__wbg_value_0b0cebe9335a78ae = function() { return logError(function (arg0, arg1) {
        const ret = getObject(arg1).value;
        const ptr1 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len1 = WASM_VECTOR_LEN;
        getDataViewMemory0().setInt32(arg0 + 4 * 1, len1, true);
        getDataViewMemory0().setInt32(arg0 + 4 * 0, ptr1, true);
    }, arguments) };
    imports.wbg.__wbg_textContent_389dd460500a44bd = function() { return logError(function (arg0, arg1) {
        const ret = getObject(arg1).textContent;
        var ptr1 = isLikeNone(ret) ? 0 : passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        var len1 = WASM_VECTOR_LEN;
        getDataViewMemory0().setInt32(arg0 + 4 * 1, len1, true);
        getDataViewMemory0().setInt32(arg0 + 4 * 0, ptr1, true);
    }, arguments) };
    imports.wbg.__wbg_instanceof_HtmlFormElement_b7d5ed0355176c29 = function() { return logError(function (arg0) {
        let result;
        try {
            result = getObject(arg0) instanceof HTMLFormElement;
        } catch (_) {
            result = false;
        }
        const ret = result;
        _assertBoolean(ret);
        return ret;
    }, arguments) };
    imports.wbg.__wbg_getformdata_42029f06cbdc8637 = function() { return logError(function (arg0) {
        const ret = get_form_data(getObject(arg0));
        return addHeapObject(ret);
    }, arguments) };
    imports.wbg.__wbg_entries_08a0332a9c4be547 = function() { return logError(function (arg0) {
        const ret = getObject(arg0).entries();
        return addHeapObject(ret);
    }, arguments) };
    imports.wbg.__wbg_getselectdata_d0a959dad128957f = function() { return logError(function (arg0, arg1) {
        const ret = get_select_data(getObject(arg1));
        const ptr1 = passArrayJsValueToWasm0(ret, wasm.__wbindgen_malloc);
        const len1 = WASM_VECTOR_LEN;
        getDataViewMemory0().setInt32(arg0 + 4 * 1, len1, true);
        getDataViewMemory0().setInt32(arg0 + 4 * 0, ptr1, true);
    }, arguments) };
    imports.wbg.__wbg_new_034f913e7636e987 = function() { return logError(function () {
        const ret = new Array();
        return addHeapObject(ret);
    }, arguments) };
    imports.wbg.__wbg_set_425e70f7c64ac962 = function() { return logError(function (arg0, arg1, arg2) {
        getObject(arg0)[arg1 >>> 0] = takeObject(arg2);
    }, arguments) };
    imports.wbg.__wbindgen_number_new = function(arg0) {
        const ret = arg0;
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_new_7a87a0376e40533b = function() { return logError(function () {
        const ret = new Map();
        return addHeapObject(ret);
    }, arguments) };
    imports.wbg.__wbg_set_277a63e77c89279f = function() { return logError(function (arg0, arg1, arg2) {
        const ret = getObject(arg0).set(getObject(arg1), getObject(arg2));
        return addHeapObject(ret);
    }, arguments) };
    imports.wbg.__wbindgen_is_string = function(arg0) {
        const ret = typeof(getObject(arg0)) === 'string';
        _assertBoolean(ret);
        return ret;
    };
    imports.wbg.__wbg_set_841ac57cff3d672b = function() { return logError(function (arg0, arg1, arg2) {
        getObject(arg0)[takeObject(arg1)] = takeObject(arg2);
    }, arguments) };
    imports.wbg.__wbg_instanceof_Window_6575cd7f1322f82f = function() { return logError(function (arg0) {
        let result;
        try {
            result = getObject(arg0) instanceof Window;
        } catch (_) {
            result = false;
        }
        const ret = result;
        _assertBoolean(ret);
        return ret;
    }, arguments) };
    imports.wbg.__wbg_document_d7fa2c739c2b191a = function() { return logError(function (arg0) {
        const ret = getObject(arg0).document;
        return isLikeNone(ret) ? 0 : addHeapObject(ret);
    }, arguments) };
    imports.wbg.__wbg_new_c44bca3b6f94b9dc = function() { return logError(function (arg0) {
        const ret = new WebDioxusChannel(JSOwner.__wrap(arg0));
        return addHeapObject(ret);
    }, arguments) };
    imports.wbg.__wbg_weak_cd28ee08d42a73ad = function() { return logError(function (arg0) {
        const ret = getObject(arg0).weak();
        return addHeapObject(ret);
    }, arguments) };
    imports.wbg.__wbg_newwithargs_54f5f31ff1323eb2 = function() { return logError(function (arg0, arg1, arg2, arg3) {
        const ret = new Function(getStringFromWasm0(arg0, arg1), getStringFromWasm0(arg2, arg3));
        return addHeapObject(ret);
    }, arguments) };
    imports.wbg.__wbg_call_3bfa248576352471 = function() { return handleError(function (arg0, arg1, arg2) {
        const ret = getObject(arg0).call(getObject(arg1), getObject(arg2));
        return addHeapObject(ret);
    }, arguments) };
    imports.wbg.__wbg_stringify_eead5648c09faaf8 = function() { return handleError(function (arg0) {
        const ret = JSON.stringify(getObject(arg0));
        return addHeapObject(ret);
    }, arguments) };
    imports.wbg.__wbindgen_is_undefined = function(arg0) {
        const ret = getObject(arg0) === undefined;
        _assertBoolean(ret);
        return ret;
    };
    imports.wbg.__wbg_length_ace210b441c50e19 = function() { return logError(function (arg0) {
        const ret = getObject(arg0).length;
        _assertNum(ret);
        return ret;
    }, arguments) };
    imports.wbg.__wbg_charCodeAt_b8a738ac743eeff4 = function() { return logError(function (arg0, arg1) {
        const ret = getObject(arg0).charCodeAt(arg1 >>> 0);
        return ret;
    }, arguments) };
    imports.wbg.__wbg_rustSend_3b93da3cdc175722 = function() { return logError(function (arg0, arg1) {
        getObject(arg0).rustSend(takeObject(arg1));
    }, arguments) };
    imports.wbg.__wbg_rustRecv_1b6f5c2d10fa7141 = function() { return logError(function (arg0) {
        const ret = getObject(arg0).rustRecv();
        return addHeapObject(ret);
    }, arguments) };
    imports.wbg.__wbg_getElementById_734c4eac4fec5911 = function() { return logError(function (arg0, arg1, arg2) {
        const ret = getObject(arg0).getElementById(getStringFromWasm0(arg1, arg2));
        return isLikeNone(ret) ? 0 : addHeapObject(ret);
    }, arguments) };
    imports.wbg.__wbg_error_53abcd6a461f73d8 = function() { return logError(function (arg0) {
        console.error(getObject(arg0));
    }, arguments) };
    imports.wbg.__wbg_ownerDocument_1ff29e4c967e4d78 = function() { return logError(function (arg0) {
        const ret = getObject(arg0).ownerDocument;
        return isLikeNone(ret) ? 0 : addHeapObject(ret);
    }, arguments) };
    imports.wbg.__wbg_new_0924b9f1bd6dd838 = function() { return logError(function (arg0) {
        const ret = new RawInterpreter(arg0 >>> 0);
        return addHeapObject(ret);
    }, arguments) };
    imports.wbg.__wbg_initialize_ed565fb65dd7bcd0 = function() { return logError(function (arg0, arg1, arg2) {
        getObject(arg0).initialize(takeObject(arg1), getObject(arg2));
    }, arguments) };
    imports.wbg.__wbg_crypto_1d1f22824a6a080c = function() { return logError(function (arg0) {
        const ret = getObject(arg0).crypto;
        return addHeapObject(ret);
    }, arguments) };
    imports.wbg.__wbg_process_4a72847cc503995b = function() { return logError(function (arg0) {
        const ret = getObject(arg0).process;
        return addHeapObject(ret);
    }, arguments) };
    imports.wbg.__wbg_versions_f686565e586dd935 = function() { return logError(function (arg0) {
        const ret = getObject(arg0).versions;
        return addHeapObject(ret);
    }, arguments) };
    imports.wbg.__wbg_node_104a2ff8d6ea03a2 = function() { return logError(function (arg0) {
        const ret = getObject(arg0).node;
        return addHeapObject(ret);
    }, arguments) };
    imports.wbg.__wbg_require_cca90b1a94a0255b = function() { return handleError(function () {
        const ret = module.require;
        return addHeapObject(ret);
    }, arguments) };
    imports.wbg.__wbindgen_is_function = function(arg0) {
        const ret = typeof(getObject(arg0)) === 'function';
        _assertBoolean(ret);
        return ret;
    };
    imports.wbg.__wbg_msCrypto_eb05e62b530a1508 = function() { return logError(function (arg0) {
        const ret = getObject(arg0).msCrypto;
        return addHeapObject(ret);
    }, arguments) };
    imports.wbg.__wbg_newwithlength_76462a666eca145f = function() { return logError(function (arg0) {
        const ret = new Uint8Array(arg0 >>> 0);
        return addHeapObject(ret);
    }, arguments) };
    imports.wbg.__wbg_abort_c57daab47a6c1215 = function() { return logError(function (arg0) {
        getObject(arg0).abort();
    }, arguments) };
    imports.wbg.__wbg_setmethod_ce2da76000b02f6a = function() { return logError(function (arg0, arg1, arg2) {
        getObject(arg0).method = getStringFromWasm0(arg1, arg2);
    }, arguments) };
    imports.wbg.__wbg_new_a9ae04a5200606a5 = function() { return handleError(function () {
        const ret = new Headers();
        return addHeapObject(ret);
    }, arguments) };
    imports.wbg.__wbg_setheaders_f5205d36e423a544 = function() { return logError(function (arg0, arg1) {
        getObject(arg0).headers = getObject(arg1);
    }, arguments) };
    imports.wbg.__wbg_setmode_4919fd636102c586 = function() { return logError(function (arg0, arg1) {
        getObject(arg0).mode = __wbindgen_enum_RequestMode[arg1];
    }, arguments) };
    imports.wbg.__wbg_setcredentials_a4e661320cdb9738 = function() { return logError(function (arg0, arg1) {
        getObject(arg0).credentials = __wbindgen_enum_RequestCredentials[arg1];
    }, arguments) };
    imports.wbg.__wbg_setbody_aa8b691bec428bf4 = function() { return logError(function (arg0, arg1) {
        getObject(arg0).body = getObject(arg1);
    }, arguments) };
    imports.wbg.__wbg_new_75169ae5a9683c55 = function() { return handleError(function () {
        const ret = new AbortController();
        return addHeapObject(ret);
    }, arguments) };
    imports.wbg.__wbg_signal_9acfcec9e7dffc22 = function() { return logError(function (arg0) {
        const ret = getObject(arg0).signal;
        return addHeapObject(ret);
    }, arguments) };
    imports.wbg.__wbg_setsignal_812ccb8269a7fd90 = function() { return logError(function (arg0, arg1) {
        getObject(arg0).signal = getObject(arg1);
    }, arguments) };
    imports.wbg.__wbg_append_8b3e7f74a47ea7d5 = function() { return handleError(function (arg0, arg1, arg2, arg3, arg4) {
        getObject(arg0).append(getStringFromWasm0(arg1, arg2), getStringFromWasm0(arg3, arg4));
    }, arguments) };
    imports.wbg.__wbg_instanceof_Response_3c0e210a57ff751d = function() { return logError(function (arg0) {
        let result;
        try {
            result = getObject(arg0) instanceof Response;
        } catch (_) {
            result = false;
        }
        const ret = result;
        _assertBoolean(ret);
        return ret;
    }, arguments) };
    imports.wbg.__wbg_status_5f4e900d22140a18 = function() { return logError(function (arg0) {
        const ret = getObject(arg0).status;
        _assertNum(ret);
        return ret;
    }, arguments) };
    imports.wbg.__wbg_url_58af972663531d16 = function() { return logError(function (arg0, arg1) {
        const ret = getObject(arg1).url;
        const ptr1 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len1 = WASM_VECTOR_LEN;
        getDataViewMemory0().setInt32(arg0 + 4 * 1, len1, true);
        getDataViewMemory0().setInt32(arg0 + 4 * 0, ptr1, true);
    }, arguments) };
    imports.wbg.__wbg_headers_1b9bf90c73fae600 = function() { return logError(function (arg0) {
        const ret = getObject(arg0).headers;
        return addHeapObject(ret);
    }, arguments) };
    imports.wbg.__wbg_arrayBuffer_144729e09879650e = function() { return handleError(function (arg0) {
        const ret = getObject(arg0).arrayBuffer();
        return addHeapObject(ret);
    }, arguments) };
    imports.wbg.__wbg_length_9254c4bd3b9f23c4 = function() { return logError(function (arg0) {
        const ret = getObject(arg0).length;
        _assertNum(ret);
        return ret;
    }, arguments) };
    imports.wbg.__wbg_get_ef828680c64da212 = function() { return handleError(function (arg0, arg1) {
        const ret = Reflect.get(getObject(arg0), getObject(arg1));
        return addHeapObject(ret);
    }, arguments) };
    imports.wbg.__wbg_call_a9ef466721e824f2 = function() { return handleError(function (arg0, arg1) {
        const ret = getObject(arg0).call(getObject(arg1));
        return addHeapObject(ret);
    }, arguments) };
    imports.wbg.__wbg_next_13b477da1eaa3897 = function() { return logError(function (arg0) {
        const ret = getObject(arg0).next;
        return addHeapObject(ret);
    }, arguments) };
    imports.wbg.__wbg_buffer_ccaed51a635d8a2d = function() { return logError(function (arg0) {
        const ret = getObject(arg0).buffer;
        return addHeapObject(ret);
    }, arguments) };
    imports.wbg.__wbg_set_ec2fcf81bc573fd9 = function() { return logError(function (arg0, arg1, arg2) {
        getObject(arg0).set(getObject(arg1), arg2 >>> 0);
    }, arguments) };
    imports.wbg.__wbg_self_bf91bf94d9e04084 = function() { return handleError(function () {
        const ret = self.self;
        return addHeapObject(ret);
    }, arguments) };
    imports.wbg.__wbg_window_52dd9f07d03fd5f8 = function() { return handleError(function () {
        const ret = window.window;
        return addHeapObject(ret);
    }, arguments) };
    imports.wbg.__wbg_globalThis_05c129bf37fcf1be = function() { return handleError(function () {
        const ret = globalThis.globalThis;
        return addHeapObject(ret);
    }, arguments) };
    imports.wbg.__wbg_global_3eca19bb09e9c484 = function() { return handleError(function () {
        const ret = global.global;
        return addHeapObject(ret);
    }, arguments) };
    imports.wbg.__wbg_newnoargs_1ede4bf2ebbaaf43 = function() { return logError(function (arg0, arg1) {
        const ret = new Function(getStringFromWasm0(arg0, arg1));
        return addHeapObject(ret);
    }, arguments) };
    imports.wbg.__wbg_newwithbyteoffsetandlength_7e3eb787208af730 = function() { return logError(function (arg0, arg1, arg2) {
        const ret = new Uint8Array(getObject(arg0), arg1 >>> 0, arg2 >>> 0);
        return addHeapObject(ret);
    }, arguments) };
    imports.wbg.__wbg_randomFillSync_5c9c955aa56b6049 = function() { return handleError(function (arg0, arg1) {
        getObject(arg0).randomFillSync(takeObject(arg1));
    }, arguments) };
    imports.wbg.__wbg_subarray_975a06f9dbd16995 = function() { return logError(function (arg0, arg1, arg2) {
        const ret = getObject(arg0).subarray(arg1 >>> 0, arg2 >>> 0);
        return addHeapObject(ret);
    }, arguments) };
    imports.wbg.__wbg_getRandomValues_3aa56aa6edec874c = function() { return handleError(function (arg0, arg1) {
        getObject(arg0).getRandomValues(getObject(arg1));
    }, arguments) };
    imports.wbg.__wbg_has_bd717f25f195f23d = function() { return handleError(function (arg0, arg1) {
        const ret = Reflect.has(getObject(arg0), getObject(arg1));
        _assertBoolean(ret);
        return ret;
    }, arguments) };
    imports.wbg.__wbg_fetch_f8d735ba6fe1b719 = function() { return logError(function (arg0) {
        const ret = fetch(getObject(arg0));
        return addHeapObject(ret);
    }, arguments) };
    imports.wbg.__wbg_fetch_1fdc4448ed9eec00 = function() { return logError(function (arg0, arg1) {
        const ret = getObject(arg0).fetch(getObject(arg1));
        return addHeapObject(ret);
    }, arguments) };
    imports.wbg.__wbg_instanceof_Uint8Array_df0761410414ef36 = function() { return logError(function (arg0) {
        let result;
        try {
            result = getObject(arg0) instanceof Uint8Array;
        } catch (_) {
            result = false;
        }
        const ret = result;
        _assertBoolean(ret);
        return ret;
    }, arguments) };
    imports.wbg.__wbg_instanceof_ArrayBuffer_74945570b4a62ec7 = function() { return logError(function (arg0) {
        let result;
        try {
            result = getObject(arg0) instanceof ArrayBuffer;
        } catch (_) {
            result = false;
        }
        const ret = result;
        _assertBoolean(ret);
        return ret;
    }, arguments) };
    imports.wbg.__wbg_String_88810dfeb4021902 = function() { return logError(function (arg0, arg1) {
        const ret = String(getObject(arg1));
        const ptr1 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len1 = WASM_VECTOR_LEN;
        getDataViewMemory0().setInt32(arg0 + 4 * 1, len1, true);
        getDataViewMemory0().setInt32(arg0 + 4 * 0, ptr1, true);
    }, arguments) };
    imports.wbg.__wbindgen_throw = function(arg0, arg1) {
        throw new Error(getStringFromWasm0(arg0, arg1));
    };
    imports.wbg.__wbindgen_debug_string = function(arg0, arg1) {
        const ret = debugString(getObject(arg1));
        const ptr1 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len1 = WASM_VECTOR_LEN;
        getDataViewMemory0().setInt32(arg0 + 4 * 1, len1, true);
        getDataViewMemory0().setInt32(arg0 + 4 * 0, ptr1, true);
    };
    imports.wbg.__wbindgen_cb_drop = function(arg0) {
        const obj = takeObject(arg0).original;
        if (obj.cnt-- == 1) {
            obj.a = 0;
            return true;
        }
        const ret = false;
        _assertBoolean(ret);
        return ret;
    };
    imports.wbg.__wbg_then_4866a7d9f55d8f3e = function() { return logError(function (arg0, arg1, arg2) {
        const ret = getObject(arg0).then(getObject(arg1), getObject(arg2));
        return addHeapObject(ret);
    }, arguments) };
    imports.wbg.__wbg_queueMicrotask_848aa4969108a57e = function() { return logError(function (arg0) {
        const ret = getObject(arg0).queueMicrotask;
        return addHeapObject(ret);
    }, arguments) };
    imports.wbg.__wbg_resolve_0aad7c1484731c99 = function() { return logError(function (arg0) {
        const ret = Promise.resolve(getObject(arg0));
        return addHeapObject(ret);
    }, arguments) };
    imports.wbg.__wbg_then_748f75edfb032440 = function() { return logError(function (arg0, arg1) {
        const ret = getObject(arg0).then(getObject(arg1));
        return addHeapObject(ret);
    }, arguments) };
    imports.wbg.__wbg_queueMicrotask_c5419c06eab41e73 = function() { return logError(function (arg0) {
        queueMicrotask(getObject(arg0));
    }, arguments) };
    imports.wbg.__wbg_newwithstrandinit_4b92c89af0a8e383 = function() { return handleError(function (arg0, arg1, arg2) {
        const ret = new Request(getStringFromWasm0(arg0, arg1), getObject(arg2));
        return addHeapObject(ret);
    }, arguments) };
    imports.wbg.__wbindgen_closure_wrapper757 = function() { return logError(function (arg0, arg1, arg2) {
        const ret = makeMutClosure(arg0, arg1, 31, __wbg_adapter_48);
        return addHeapObject(ret);
    }, arguments) };
    imports.wbg.__wbindgen_closure_wrapper782 = function() { return logError(function (arg0, arg1, arg2) {
        const ret = makeMutClosure(arg0, arg1, 31, __wbg_adapter_51);
        return addHeapObject(ret);
    }, arguments) };
    imports.wbg.__wbindgen_closure_wrapper1858 = function() { return logError(function (arg0, arg1, arg2) {
        const ret = makeMutClosure(arg0, arg1, 31, __wbg_adapter_54);
        return addHeapObject(ret);
    }, arguments) };

    return imports;
}

function __wbg_init_memory(imports, memory) {

}

function __wbg_finalize_init(instance, module) {
    wasm = instance.exports;
    __wbg_init.__wbindgen_wasm_module = module;
    cachedDataViewMemory0 = null;
    cachedUint8ArrayMemory0 = null;


    wasm.__wbindgen_start();
    return wasm;
}

function initSync(module) {
    if (wasm !== undefined) return wasm;


    if (typeof module !== 'undefined') {
        if (Object.getPrototypeOf(module) === Object.prototype) {
            ({module} = module)
        } else {
            console.warn('using deprecated parameters for `initSync()`; pass a single object instead')
        }
    }

    const imports = __wbg_get_imports();

    __wbg_init_memory(imports);

    if (!(module instanceof WebAssembly.Module)) {
        module = new WebAssembly.Module(module);
    }

    const instance = new WebAssembly.Instance(module, imports);

    return __wbg_finalize_init(instance, module);
}

async function __wbg_init(module_or_path) {
    if (wasm !== undefined) return wasm;


    if (typeof module_or_path !== 'undefined') {
        if (Object.getPrototypeOf(module_or_path) === Object.prototype) {
            ({module_or_path} = module_or_path)
        } else {
            console.warn('using deprecated parameters for the initialization function; pass a single object instead')
        }
    }


    const imports = __wbg_get_imports();

    if (typeof module_or_path === 'string' || (typeof Request === 'function' && module_or_path instanceof Request) || (typeof URL === 'function' && module_or_path instanceof URL)) {
        module_or_path = fetch(module_or_path);
    }

    __wbg_init_memory(imports);

    const { instance, module } = await __wbg_load(await module_or_path, imports);

    return __wbg_finalize_init(instance, module);
}

export { initSync };
export default __wbg_init;
