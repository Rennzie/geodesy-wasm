// This NTv2 grid reader is based on the following documents:
// - https://web.archive.org/web/20140127204822if_/http://www.mgs.gov.on.ca:80/stdprodconsume/groups/content/@mgs/@iandit/documents/resourcelist/stel02_047447.pdf
// - http://mimaka.com/help/gs/html/004_NTV2%20Data%20Format.htm
// - https://github.com/Esri/ntv2-file-routines/blob/master/README.md
//
// And inspired by existing implementations in
// - https://github.com/proj4js/proj4js/blob/master/lib/nadgrid.js
// - https://github.com/3liz/proj4rs/blob/main/src/nadgrids/grid.rs
use crate::error::{Error, Result};
use js_sys::DataView;

const SEC_TO_DEG: f64 = 0.0002777778;

// Both overview and sub grid headers have 11 fields of 16 bytes each.
const NTV2_HEADER_SIZE: usize = 11 * 16;
const NTV2_NODE_SIZE: usize = 16;

const ERR_INVALID_HEADER: &str = "Wrong header";
const ERR_INVALID_GRID_TO_SHORT: &str = "Invalid Grid: To Short";

// Buffer offsets for the NTv2 subgrid header
const NTV2_SUBGRID_NLAT: usize = 88; // (f64) upper latitude (in seconds)
const NTV2_SUBGRID_SLAT: usize = 72; // (f64) lower latitude (in seconds)
const NTV2_SUBGRID_ELON: usize = 104; // (f64) lower longitude (in seconds)
const NTV2_SUBGRID_WLON: usize = 120; // (f64) upper longitude (in seconds)
const NTV2_SUBGRID_DLAT: usize = 136; // (f64) Latitude interval (in seconds)
const NTV2_SUBGRID_DLON: usize = 152; // (f64) Longitude interval (in seconds)
const NTV2_SUBGRID_GSCOUNT: usize = 168; // (i32) grid node count

const NTV2_NODE_LAT_CORRN: usize = 0; // (f32) correction to the latitude at this node point (secs)
const NTV2_NODE_LON_CORRN: usize = 4; // (f32) correction to the longitude at this node point (secs)

/// Read a NTv2 grid from a `js_sys::DataView`.
pub fn parse_ntv2_to_gravsoft_bin(view: DataView) -> Result<Vec<u8>> {
    let is_le = view.get_int32_endian(8, true) == 11;

    // TODO: read the header magic (string) and compare it to NTV2_MAGIC
    let num_of_fields = view.get_int32_endian(8, is_le);
    if num_of_fields != 11 {
        return Err(Error::InvalidNtv2GridFormat(ERR_INVALID_HEADER).into());
    }

    let num_sub_grids = view.get_int32_endian(40, is_le) as usize;

    if num_sub_grids != 1 {
        // Multi grid support is out of scope for geodesy-wasm
        return Err(Error::UnsupportedNtv2("Contains more than one subgrid"));
    }

    let (header, grid) = read_ntv2_subgrid(view, NTV2_HEADER_SIZE, is_le)?;
    into_gravsoft_bin(header, grid)
}

// The gravsoft reader in geodesy_rs expects a text buffer from a list of f64 rows
/// Converts the `NTv2Grid` into binary gravsoft compatible with Rust Geodesy
fn into_gravsoft_bin(header: Vec<f64>, grid: Vec<f64>) -> Result<Vec<u8>> {
    let gravsoft_header = header
        .iter()
        .map(|value| format!("{}\n", value))
        .collect::<String>();

    let grav_soft_grid = grid
        .iter()
        .map(|value| format!("{}\n", value))
        .collect::<String>();

    let gravsoft = gravsoft_header + &grav_soft_grid;
    Ok(gravsoft.into_bytes())
}

fn read_ntv2_subgrid(view: DataView, offset: usize, is_le: bool) -> Result<(Vec<f64>, Vec<f64>)> {
    let lat_0 = view.get_float64_endian(offset + NTV2_SUBGRID_NLAT, is_le) * SEC_TO_DEG; // Latitude of the first (typically northernmost) row of the grid
    let lat_1 = view.get_float64_endian(offset + NTV2_SUBGRID_SLAT, is_le) * SEC_TO_DEG; // Latitude of the last (typically southernmost) row of the grid

    // The Canadian makers of NTv2 have flipped the signs on East(-) and West(+),
    // probably because all of Canada is west of Greenwich.
    // By common convention East is negative and West is positive so we flip them here
    let lon_0 = -view.get_float64_endian(offset + NTV2_SUBGRID_WLON, is_le) * SEC_TO_DEG; // Longitude of the first (typically westernmost) column of each row
    let lon_1 = -view.get_float64_endian(offset + NTV2_SUBGRID_ELON, is_le) * SEC_TO_DEG; // Longitude of the last (typically easternmost) column of each row

    let dlat = view.get_float64_endian(offset + NTV2_SUBGRID_DLAT, is_le) * SEC_TO_DEG; // Signed distance between two consecutive rows
    let dlon = view.get_float64_endian(offset + NTV2_SUBGRID_DLON, is_le) * SEC_TO_DEG; // Signed distance between two consecutive columns

    let num_nodes = view.get_int32_endian(offset + NTV2_SUBGRID_GSCOUNT, is_le) as usize;

    let grid_start_offset = offset + NTV2_HEADER_SIZE;
    let grid_end_offset = grid_start_offset + num_nodes * NTV2_NODE_SIZE;

    if grid_end_offset > view.byte_length() {
        return Err(Error::InvalidNtv2GridFormat(ERR_INVALID_GRID_TO_SHORT).into());
    }

    let rows = ((lat_1 - lat_0) / -dlat + 1.5).floor() as usize;
    let cols = ((lon_1 - lon_0) / dlon + 1.5).floor() as usize;

    let mut header = Vec::<f64>::new();
    header.push(lat_1);
    header.push(lat_0);
    header.push(lon_0);
    header.push(lon_1);
    header.push(dlat);
    header.push(dlon);

    let mut grid = Vec::<f64>::new(); // An interleaved vector of node values ordered [lat₀, lon₀...latₙ, lonₙ]

    // Gravsoft grids are North West in the top left corner,
    // opposite to NTv2 which starts in the South £ast corner.
    for r in (0..rows).rev() {
        for c in (0..cols).rev() {
            let offset = grid_start_offset + (c * r) * NTV2_NODE_SIZE;
            let lat_correction = view.get_float64_endian(offset + NTV2_NODE_LAT_CORRN, is_le);
            // And again we flip the longitude sign so East is positive and West is negative.
            let lon_correction = -view.get_float64_endian(offset + NTV2_NODE_LON_CORRN, is_le);

            // TODO: What is the value expected by gravsoft, just the correction or is it the node lat/lon + the correction?
            grid.push(lat_correction * SEC_TO_DEG);
            grid.push(lon_correction * SEC_TO_DEG);
        }
    }

    Ok((header, grid))
}
