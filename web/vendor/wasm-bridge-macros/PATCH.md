# Local compatibility patch

Source: AmbientRun/wasm-bridge commit `574192363a8448e86604142a331501e71b2cc023`, `crates/wasm-bridge-macros` (MIT; see LICENSE).

Make inherited Cargo metadata explicit. Accept variable token whitespace when removing native-only `ComponentType` assertions from generated browser bindings. Newer Rust token formatting no longer matches the original literal `const _ : () =` pattern.
