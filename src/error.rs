use wasm_bindgen::JsError;
pub type WasmResult<T> = std::result::Result<T, JsError>;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Invalid Ntv2 grid format: {0}")]
    InvalidNtv2GridFormat(&'static str),
    #[error("Unsupported NTv2 grid: {0} ")]
    UnsupportedNtv2(&'static str),

    #[error("RGError: {0}")]
    RgError(#[from] geodesy_rs::Error),
}
pub type Result<T> = std::result::Result<T, Error>;
