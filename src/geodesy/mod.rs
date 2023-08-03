use geodesy_rs::prelude::AngularUnits;
use wasm_bindgen::prelude::*;

use crate::utils::log;
mod context;
mod coordinate;
mod ntv2_grid;

#[wasm_bindgen (js_name = readGrid)]
pub fn read_grid(data_view: js_sys::DataView) -> crate::error::WasmResult<()> {
    let raw_grid = ntv2_grid::NTv2Grid::from_data_view(data_view)?;
    let rg_grid = raw_grid.into_grid()?;

    let position = geodesy_rs::Coor4D::geo(51.505, -0.09, 0.0, 0.0);
    let contains_point = rg_grid.contains(position)?;
    log(format!("contains_point: {contains_point}")
        .to_string()
        .as_str());

    let interpolated = rg_grid.interpolation(&position, None).to_geo();
    let lon = interpolated.0[0];
    let lat = interpolated.0[1];
    log(format!("lat: {lat} lon: {lon}").to_string().as_str());

    Ok(())
}
