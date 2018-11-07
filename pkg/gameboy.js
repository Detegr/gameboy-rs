/* tslint:disable */
import * as wasm from './gameboy_bg';

/**
* @param {Emulator} arg0
* @returns {void}
*/
export function reset(arg0) {
    return wasm.reset(arg0.ptr);
}

/**
* @param {Emulator} arg0
* @returns {boolean}
*/
export function run_until_redraw(arg0) {
    return (wasm.run_until_redraw(arg0.ptr)) !== 0;
}

let cachegetUint8Memory = null;
function getUint8Memory() {
    if (cachegetUint8Memory === null || cachegetUint8Memory.buffer !== wasm.memory.buffer) {
        cachegetUint8Memory = new Uint8Array(wasm.memory.buffer);
    }
    return cachegetUint8Memory;
}

function getArrayU8FromWasm(ptr, len) {
    return getUint8Memory().subarray(ptr / 1, ptr / 1 + len);
}

let cachedGlobalArgumentPtr = null;
function globalArgumentPtr() {
    if (cachedGlobalArgumentPtr === null) {
        cachedGlobalArgumentPtr = wasm.__wbindgen_global_argument_ptr();
    }
    return cachedGlobalArgumentPtr;
}

let cachegetUint32Memory = null;
function getUint32Memory() {
    if (cachegetUint32Memory === null || cachegetUint32Memory.buffer !== wasm.memory.buffer) {
        cachegetUint32Memory = new Uint32Array(wasm.memory.buffer);
    }
    return cachegetUint32Memory;
}
/**
* @param {Emulator} arg0
* @returns {Uint8Array}
*/
export function display_buffer(arg0) {
    const retptr = globalArgumentPtr();
    wasm.display_buffer(retptr, arg0.ptr);
    const mem = getUint32Memory();
    const rustptr = mem[retptr / 4];
    const rustlen = mem[retptr / 4 + 1];

    const realRet = getArrayU8FromWasm(rustptr, rustlen).slice();
    wasm.__wbindgen_free(rustptr, rustlen * 1);
    return realRet;

}

function passArray8ToWasm(arg) {
    const ptr = wasm.__wbindgen_malloc(arg.length * 1);
    getUint8Memory().set(arg, ptr / 1);
    return [ptr, arg.length];
}
/**
* @param {Emulator} arg0
* @param {Uint8Array} arg1
* @returns {void}
*/
export function load_cartridge_data(arg0, arg1) {
    const [ptr1, len1] = passArray8ToWasm(arg1);
    try {
        return wasm.load_cartridge_data(arg0.ptr, ptr1, len1);

    } finally {
        wasm.__wbindgen_free(ptr1, len1 * 1);

    }

}

/**
* @param {Emulator} arg0
* @returns {boolean}
*/
export function should_redraw_display(arg0) {
    return (wasm.should_redraw_display(arg0.ptr)) !== 0;
}

let cachedTextEncoder = new TextEncoder('utf-8');

function passStringToWasm(arg) {

    const buf = cachedTextEncoder.encode(arg);
    const ptr = wasm.__wbindgen_malloc(buf.length);
    getUint8Memory().set(buf, ptr);
    return [ptr, buf.length];
}
/**
* @param {string} arg0
* @returns {Emulator}
*/
export function get_emulator(arg0) {
    const [ptr0, len0] = passStringToWasm(arg0);
    try {
        return Emulator.__wrap(wasm.get_emulator(ptr0, len0));

    } finally {
        wasm.__wbindgen_free(ptr0, len0 * 1);

    }

}

function freeEmulator(ptr) {

    wasm.__wbg_emulator_free(ptr);
}
/**
*/
export class Emulator {

    static __wrap(ptr) {
        const obj = Object.create(Emulator.prototype);
        obj.ptr = ptr;

        return obj;
    }

    free() {
        const ptr = this.ptr;
        this.ptr = 0;
        freeEmulator(ptr);
    }

}

let cachedTextDecoder = new TextDecoder('utf-8');

function getStringFromWasm(ptr, len) {
    return cachedTextDecoder.decode(getUint8Memory().subarray(ptr, ptr + len));
}

export function __wbindgen_throw(ptr, len) {
    throw new Error(getStringFromWasm(ptr, len));
}

