use wasm_bindgen::JsError;
pub type WasmResult<T> = std::result::Result<T, JsError>;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("RGError: {0}")]
    RgError(#[from] geodesy_rs::Error),

    #[error("MissingGrid: {0}")]
    MissingGrid(String),

    #[error("Invalid: {0}")]
    Invalid(String),
}
pub type Result<T> = std::result::Result<T, Error>;
