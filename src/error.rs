use wasm_bindgen::JsError;
pub type WasmResult<T> = std::result::Result<T, JsError>;
