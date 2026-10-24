# Local compatibility patch

Source: AmbientRun/wasm-bridge commit `574192363a8448e86604142a331501e71b2cc023`, `crates/wasm-bridge-js` (MIT; see LICENSE).

Changes: make inherited Cargo metadata explicit and point sibling dependencies to the original Git source, locked by the browser workspace. Add the ABI slot constraint required by wasm-bindgen 0.2.89 to `ToJsValue::ReturnAbi`. Without this bound, generic closures returning `Result<R::ReturnAbi, JsValue>` fail to implement `WasmClosure`.

This copy is patched only into the Ambient browser workspace. It preserves the conversion behavior and guest API.
