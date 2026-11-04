// Entry point for browsers and bundlers: fetches the WebAssembly module
// relative to this file before any export can be used.
import init from "../dist/wasm/fiscalclock_wasm.js";

await init({ module_or_path: new URL("./wasm/fiscalclock_wasm_bg.wasm", import.meta.url) });

export * from "./index.js";
