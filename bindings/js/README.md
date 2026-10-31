# JavaScript binding

This folder will hold the `fiscalclock` npm package: the Rust core compiled to WebAssembly with wasm-bindgen, plus a TypeScript layer that converts between JavaScript values and the core's JSON API. It accepts ISO `YYYY-MM-DD` strings and `Temporal.PlainDate` values and rejects JavaScript `Date` objects, because a `Date` is an instant whose calendar day depends on the time zone. No calendar logic lives here; see [ADR-0002](https://github.com/Karthik-beta/fiscalclock/wiki/ADR-0002-single-rust-core-with-thin-bindings).
