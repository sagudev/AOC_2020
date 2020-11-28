/* tslint:disable */
/* eslint-disable */
/**
* this is called from js
* data is split by newline
* @param {number} day
* @param {string} s
* @returns {any[]}
*/
export function js_mach(day: number, s: string): any[];
/**
* List of all days
*/
export enum Days {
  Dayx,
  Day5,
}

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
  readonly memory: WebAssembly.Memory;
  readonly js_mach: (a: number, b: number, c: number, d: number) => void;
  readonly __wbindgen_malloc: (a: number) => number;
  readonly __wbindgen_realloc: (a: number, b: number, c: number) => number;
  readonly __wbindgen_free: (a: number, b: number) => void;
}

/**
* If `module_or_path` is {RequestInfo} or {URL}, makes a request and
* for everything else, calls `WebAssembly.instantiate` directly.
*
* @param {InitInput | Promise<InitInput>} module_or_path
*
* @returns {Promise<InitOutput>}
*/
export default function init (module_or_path?: InitInput | Promise<InitInput>): Promise<InitOutput>;
        