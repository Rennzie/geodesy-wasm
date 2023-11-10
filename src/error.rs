use wasm_bindgen::JsError;
pub type WasmResult<T> = std::result::Result<T, JsError>;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("RGError: {0}")]
    RgError(#[from] geodesy_rs::Error),
}
pub type Result<T> = std::result::Result<T, Error>;
