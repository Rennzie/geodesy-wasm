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
const SEC_TO_MIN: f64 = 0.0166666667;

// Both overview and sub grid headers have 11 fields of 16 bytes each.
const NTV2_HEADER_SIZE: usize = 11 * 16;
const NTV2_NODE_SIZE: usize = 16;

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

/// Read a NTv2 grid from a `js_sys::DataView` into Gravsoft binary native to Rust Geodesy.
pub fn parse_ntv2_to_gravsoft_bin(view: &DataView) -> Result<Vec<u8>> {
    let is_le = view.get_int32_endian(8, true) == 11;

    // TODO: read the header magic (string) and compare it to NTV2_MAGIC
    let num_of_fields = view.get_int32_endian(8, is_le);
    if num_of_fields != 11 {
        return Err(Error::InvalidNtv2GridFormat("Wrong header").into());
    }

    let num_sub_grids = view.get_int32_endian(40, is_le) as usize;

    if num_sub_grids != 1 {
        // Multi grid support is out of scope for geodesy-wasm
        return Err(Error::UnsupportedNtv2("Contains more than one subgrid"));
    }

    let (header, grid) = read_ntv2_subgrid(view, NTV2_HEADER_SIZE, is_le)?;
    into_gravsoft_bin(header, grid)
}

/// Converts the `NTv2Grid` into binary gravsoft compatible with Rust Geodesy
/// The gravsoft reader in geodesy_rs expects a text buffer made up of rows with f64 values delimited by spaces.
/// Rows are ordered North to South and East to West.
/// The first row contains the header values which should be in degrees
/// The subsequent rows contain the grid values which should be in seconds or arc.
/// See [geodesy_rs::grid::gravsoft_grid_reader](https://github.com/Rennzie/geodesy/blob/49384de0c70135fceac6f00ca367d171a1a8fe2e/src/grid/mod.rs#L208)
fn into_gravsoft_bin(header: Vec<f64>, grid: Vec<Vec<[f64; 2]>>) -> Result<Vec<u8>> {
    let mut gravsoft = String::with_capacity(header.len() * 10 + grid.len() * grid[0].len() * 10);
    for value in header {
        gravsoft.push_str(&value.to_string());
        gravsoft.push(' ');
    }
    gravsoft.push('\n');
    for row in &grid {
        for col in row {
            // We expect values to be ordered [lat, lon]
            for value in col {
                gravsoft.push_str(&value.to_string());
                gravsoft.push(' ');
            }
        }
        gravsoft.push('\n');
    }

    Ok(gravsoft.into_bytes())
}

fn read_ntv2_subgrid(
    view: &DataView,
    offset: usize,
    is_le: bool,
) -> Result<(Vec<f64>, Vec<Vec<[f64; 2]>>)> {
    let lat_0 = view.get_float64_endian(offset + NTV2_SUBGRID_NLAT, is_le); // Latitude of the first (typically northernmost) row of the grid
    let lat_1 = view.get_float64_endian(offset + NTV2_SUBGRID_SLAT, is_le); // Latitude of the last (typically southernmost) row of the grid

    let lon_0 = view.get_float64_endian(offset + NTV2_SUBGRID_WLON, is_le); // Longitude of the first (typically westernmost) column of each row
    let lon_1 = view.get_float64_endian(offset + NTV2_SUBGRID_ELON, is_le); // Longitude of the last (typically easternmost) column of each row

    let dlat = view.get_float64_endian(offset + NTV2_SUBGRID_DLAT, is_le); // Signed distance between two consecutive rows
    let dlon = view.get_float64_endian(offset + NTV2_SUBGRID_DLON, is_le); // Signed distance between two consecutive columns

    let num_nodes = view.get_int32_endian(offset + NTV2_SUBGRID_GSCOUNT, is_le) as usize;

    let grid_start_offset = offset + NTV2_HEADER_SIZE;
    let grid_end_offset = grid_start_offset + num_nodes * NTV2_NODE_SIZE;

    if grid_end_offset > view.byte_length() {
        return Err(Error::InvalidNtv2GridFormat("Invalid Grid: Too Short"));
    }

    // As defined by https://web.archive.org/web/20140127204822if_/http://www.mgs.gov.on.ca:80/stdprodconsume/groups/content/@mgs/@iandit/documents/resourcelist/stel02_047447.pdf (pg 30)
    let rows = (((lat_1 - lat_0) / dlat).abs() + 1.0).floor() as usize;
    let cols = (((lon_0 - lon_1) / dlon).abs() + 1.0).floor() as usize;

    if num_nodes != (rows * cols) {
        return Err(Error::InvalidNtv2GridFormat(
            "Invalid Grid: Number of nodes does not match the grid size",
        ));
    }

    // NTv2 nodes are order from SE to NW! Here we break them up into rows and columns
    // reversing the order so we end up with a grid ordered from NW to SE.
    //         NW=(r₀:c₀)      NE=(r₀:cₙ)           SW=(rₙ:c₀)      SE=(rₙ:cₙ)
    //      [[[lat₀, lon₀]...[latₙ, lonₙ]]ᵣ₀,..., [[lat₀, lon₀]...[latₙ, lonₙ]]ᵣₙ]
    //
    // lat/lon values are in minutes-of-arc as is expected by the Gravsoft reader in Rust Geodesy
    // [RG Says it's minutes of arc but it's dividing by 3600 so it's actually expecting seconds of arc?!](https://github.com/busstoptaktik/geodesy/blob/3bb3beec2593ad9c35ef3c7e6d92383e6c9828ed/src/grid/mod.rs#L182-L192)
    let grid = (0..num_nodes)
        .map(|i| {
            let offset = grid_start_offset + i * NTV2_NODE_SIZE;
            let lat_corr = view.get_float32_endian(offset + NTV2_NODE_LAT_CORRN, is_le);
            let lon_corr = view.get_float32_endian(offset + NTV2_NODE_LON_CORRN, is_le);
            [lat_corr.into(), lon_corr.into()]
        })
        .rev() // Because only the insane work SE to NW!
        .collect::<Vec<[f64; 2]>>()
        .chunks(cols) // Chunk into rows
        .map(|chunk| chunk.to_vec())
        .collect::<Vec<Vec<[f64; 2]>>>();

    let mut header = Vec::<f64>::new();
    header.push(lat_1 * SEC_TO_DEG);
    header.push(lat_0 * SEC_TO_DEG);
    // The Canadian makers of NTv2 have flipped the signs on East(-) and West(+),
    // probably because all of Canada is west of Greenwich.
    // By convention East is negative and West is positive so we flip them here
    header.push(-lon_0 * SEC_TO_DEG);
    header.push(-lon_1 * SEC_TO_DEG);
    header.push(dlat * SEC_TO_DEG);
    header.push(dlon * SEC_TO_DEG);

    Ok((header, grid))
}
