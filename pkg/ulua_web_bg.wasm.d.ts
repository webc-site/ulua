/* tslint:disable */
/* eslint-disable */
export const memory: WebAssembly.Memory;
export const __wbg_runresult_free: (a: number, b: number) => void;
export const check: (a: number, b: number) => [number, number];
export const run: (a: number, b: number) => number;
export const runresult_error: (a: number) => [number, number];
export const runresult_output: (a: number) => [number, number];
export const wasm_start: () => void;
export const sysconf: (a: number) => number;
export const mprotect: (a: number, b: number, c: number) => number;
export const munmap: (a: number, b: number) => number;
export const mmap: (a: number, b: number, c: number, d: number, e: number, f: bigint) => number;
export const time: (a: number) => bigint;
export const localtime_r: (a: number, b: number) => number;
export const gmtime_r: (a: number, b: number) => number;
export const clock: () => number;
export const free: (a: number) => void;
export const malloc: (a: number) => number;
export const realloc: (a: number, b: number) => number;
export const strchr: (a: number, b: number) => number;
export const __wbindgen_free: (a: number, b: number, c: number) => void;
export const __wbindgen_malloc: (a: number, b: number) => number;
export const __wbindgen_realloc: (a: number, b: number, c: number, d: number) => number;
export const __wbindgen_externrefs: WebAssembly.Table;
export const __wbindgen_start: () => void;
