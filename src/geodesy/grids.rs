use crate::error::WasmResult;
use geodesy_rs::Grid;
use geodesy_rs::{authoring::BaseGrid, Ntv2Grid};
use js_sys::{DataView, Uint8Array};
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
    // IDEA: To get more sophisticated we could
    // -  fetch from the network by identifying if the name is http etc
    //      -- either from the cdn or from a user defined url
    // - from IndexDB at a key/database that we pre-define

    let grid: Vec<u8> = Uint8Array::new(&data_view.buffer()).to_vec();

    let mut grids = GRIDS.get_or_init(init_grids).lock().unwrap();

    // TODO: Pull this into a separate function when we have more ways to get a grid
    if key.trim().ends_with("gsb") {
        grids.insert(key.to_string(), Arc::new(Ntv2Grid::new(&grid)?));
        return Ok(());
    } else {
        grids.insert(key.to_string(), Arc::new(BaseGrid::gravsoft(&grid)?));
    }

    Ok(())
}
