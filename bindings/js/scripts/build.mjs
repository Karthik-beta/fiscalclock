// Builds the WebAssembly module and the TypeScript layer into dist/.
import { execFileSync } from "node:child_process";
import { rmSync } from "node:fs";
import { fileURLToPath } from "node:url";

const here = fileURLToPath(new URL("..", import.meta.url));
const run = (cmd, args) => execFileSync(cmd, args, { cwd: here, stdio: "inherit", shell: process.platform === "win32" });

rmSync(new URL("../dist", import.meta.url), { recursive: true, force: true });
run("cargo", ["build", "-p", "fiscalclock-wasm", "--target", "wasm32-unknown-unknown", "--release"]);
run("wasm-bindgen", [
  "../../target/wasm32-unknown-unknown/release/fiscalclock_wasm.wasm",
  "--target", "web",
  "--out-dir", "dist/wasm",
]);
run("npx", ["tsc", "-p", "tsconfig.json"]);
