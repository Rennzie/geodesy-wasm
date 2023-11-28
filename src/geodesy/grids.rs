use crate::error::{Result, WasmResult};
use geodesy_rs::Grid;
use geodesy_rs::{authoring::BaseGrid, Ntv2Grid};
use js_sys::{DataView, Uint8Array};
use reqwest::Url;
use std::{
    collections::BTreeMap,
    sync::{Arc, Mutex, OnceLock},
};
use wasm_bindgen::prelude::*;

// A single store on the heap for all grids
pub static GRIDS: OnceLock<Mutex<BTreeMap<String, Arc<dyn Grid>>>> = OnceLock::new();

fn init_grids() -> Mutex<BTreeMap<String, Arc<dyn Grid>>> {
    Mutex::new(BTreeMap::<String, Arc<dyn Grid>>::new())
}

/// A synchronous way to register grids for use in the [Geo] class with named [DataView]s.
///
/// The keys used to load the grid MUST be the same
/// as the `grids=<key>` parameter in the definition string.
///
/// Supported Grid Types:
///     - `NTv2` (.gsb)
///     - `Gravsoft`
#[wasm_bindgen(js_name = registerGridSync)]
pub fn register_grid_sync(key: &str, data_view: DataView) -> WasmResult<()> {
    let grid: Vec<u8> = Uint8Array::new(&data_view.buffer()).to_vec();
    add_grid(key, grid)?;

    Ok(())
}

/// [Unstable] Add grids to the [Geo] class with names urls.
/// The URL MUST return a grid file in the supported formats.
///
/// The keys used to load the grid MUST be the same
/// as the `grids=<key>` parameter in the definition string.
///
/// Supported Grid Types:
///     - `NTv2` (.gsb)
///     - `Gravsoft`
#[wasm_bindgen(js_name = registerGrid)]
pub async fn UNSTABLE_register_grid(key: &str, url: &str) -> WasmResult<()> {
    // TODO: Cache the result on IndexDB if available.
    // - Make it possible to add headers when calling `register_grid`

    let url = Url::parse(url)?;
    let response = reqwest::get(url).await?;

    let bytes = response.bytes().await?;
    let byte_array = bytes.as_ref();

    add_grid(key, byte_array.into())?;

    Ok(())
}

fn add_grid(key: &str, grid_bytes: Vec<u8>) -> Result<()> {
    let mut grids = GRIDS.get_or_init(init_grids).lock().unwrap();

    if key.trim().ends_with("gsb") {
        grids.insert(key.to_string(), Arc::new(Ntv2Grid::new(&grid_bytes)?));
    } else {
        grids.insert(key.to_string(), Arc::new(BaseGrid::gravsoft(&grid_bytes)?));
    }
    return Ok(());
}
