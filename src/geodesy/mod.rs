use crate::utils::log;
use geodesy_rs::prelude::*;
use wasm_bindgen::prelude::*;

mod context;
mod coordinate;
mod ntv2_grid;
mod wasmcontext;

// TODO: remove this - it's just for testing
#[wasm_bindgen (js_name = readGrid)]
pub fn read_grid(data_view: js_sys::DataView) -> crate::error::WasmResult<()> {
    let grav_bin = ntv2_grid::parse_ntv2_to_gravsoft_bin(&data_view)?;
    let rg_grid = geodesy_rs::context_authoring::Grid::gravsoft(grav_bin.as_slice())?;

    let position = geodesy_rs::Coor4D::geo(51.505, -0.09, 0.0, 0.0);
    let contains_point = rg_grid.contains(position);
    log(format!("contains_point: {contains_point}")
        .to_string()
        .as_str());

    let interpolated = rg_grid.interpolation(&position, None).to_geo();
    let lon = interpolated.0[0];
    let lat = interpolated.0[1];
    log(format!("lat: {lat} lon: {lon}").to_string().as_str());

    Ok(())
}
