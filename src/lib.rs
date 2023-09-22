pub mod error;
pub mod geodesy;
pub mod utils;

// --- Re-exported from geodesy_rs ---
use crate::error::WasmResult;
use geodesy_rs::authoring::parse_proj;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(js_name = parseProj)]
pub fn parse_proj_wasm(definition: &str) -> WasmResult<String> {
    match parse_proj(definition) {
        Ok(geodesy_def) => Ok(geodesy_def),
        Err(e) => Err(JsError::from(e)),
    }
}
