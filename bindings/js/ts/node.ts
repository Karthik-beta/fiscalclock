// Entry point for Node.js: loads the WebAssembly module synchronously from disk.
import { readFileSync } from "node:fs";
import { initSync } from "../dist/wasm/fiscalclock_wasm.js";

initSync({ module: readFileSync(new URL("./wasm/fiscalclock_wasm_bg.wasm", import.meta.url)) });

export * from "./index.js";
