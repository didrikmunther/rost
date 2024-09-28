
import * as wasm from "./rost_wasm_bg.wasm";
import { __wbg_set_wasm } from "./rost_wasm_bg.js";
__wbg_set_wasm(wasm);
export * from "./rost_wasm_bg.js";
