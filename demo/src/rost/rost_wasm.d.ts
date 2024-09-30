/* tslint:disable */
/* eslint-disable */
/**
* @param {string} code
* @returns {WasmResult}
*/
export function compile(code: string): WasmResult;
/**
*/
export class WasmResult {
  free(): void;
/**
* @returns {string}
*/
  get_lexed(): string;
/**
* @returns {string}
*/
  get_parsed(): string;
/**
* @returns {string}
*/
  get_compiled(): string;
/**
* @returns {string}
*/
  get_wasm(): string;
}
