use crate::error::{Error, Result};
use js_sys::DataView;

const SEC_TO_DEG: f64 = 0.0002777778;

// Both overview and sub grid headers have 11 fields of 16 bytes each.
const HEADER_SIZE: usize = 11 * 16;
const NODE_SIZE: usize = 16;

// Buffer offsets for the NTv2 subgrid header
const SUBGRID_NLAT: usize = 88; // (f64)
const SUBGRID_SLAT: usize = 72; // (f64)
const SUBGRID_ELON: usize = 104; // (f64)
const SUBGRID_WLON: usize = 120; // (f64)
const SUBGRID_DLAT: usize = 136; // (f64)
const SUBGRID_DLON: usize = 152; // (f64)
const SUBGRID_GSCOUNT: usize = 168; // (i32) grid node count

// Buffer offsets for the NTv2 grid nodes
const NODE_LAT_CORRN: usize = 0; // (f32) correction to the latitude at this node point (secs)
const NODE_LON_CORRN: usize = 4; // (f32) correction to the longitude at this node point (secs)

/// Read a NTv2 grid from a `js_sys::DataView` into the Gravsoft format native to Rust Geodesy.
/// This NTv2 grid reader is based on the following documents:
/// - https://web.archive.org/web/20140127204822if_/http://www.mgs.gov.on.ca:80/stdprodconsume/groups/content/@mgs/@iandit/documents/resourcelist/stel02_047447.pdf
/// - http://mimaka.com/help/gs/html/004_NTV2%20Data%20Format.htm
/// - https://github.com/Esri/ntv2-file-routines/blob/master/README.md
///
/// And inspired by existing implementations in
/// - https://github.com/proj4js/proj4js/blob/master/lib/nadgrid.js
/// - https://github.com/3liz/proj4rs/blob/main/src/nadgrids/grid.rs
pub fn parse_ntv2_to_gravsoft(view: DataView) -> Result<Vec<u8>> {
    let is_le = view.get_int32_endian(8, true) == 11;

    let num_of_fields = view.get_int32_endian(8, is_le);
    let magic = read_string(&view, 0, 8);
    if num_of_fields != 11 && magic != "NUM_OREC".to_string() {
        return Err(Error::Ntv2InvalidGridFormat("Wrong header").into());
    }

    let num_sub_grids = view.get_int32_endian(40, is_le) as usize;
    if num_sub_grids != 1 {
        // Multi grid support is out of scope for geodesy-wasm
        return Err(Error::Ntv2Unsupported("Contains more than one subgrid"));
    }

    let gs_type = read_string(&view, 56, 8);
    if gs_type != "SECONDS".to_string() {
        return Err(Error::Ntv2Unsupported("Not in seconds"));
    }

    let (header, _, cols) = read_subgrid_header(&view, HEADER_SIZE, is_le)?;
    let grid = read_subgrid_grid(&view, HEADER_SIZE, is_le, cols)?;

    into_gravsoft_bin(header, grid)
}

fn read_subgrid_header(
    view: &DataView,
    offset: usize,
    is_le: bool,
) -> Result<(Vec<f64>, usize, usize)> {
    let lat_0 = view.get_float64_endian(offset + SUBGRID_NLAT, is_le); // Latitude of the first (typically northernmost) row of the grid
    let lat_1 = view.get_float64_endian(offset + SUBGRID_SLAT, is_le); // Latitude of the last (typically southernmost) row of the grid

    let lon_0 = view.get_float64_endian(offset + SUBGRID_WLON, is_le); // Longitude of the first (typically westernmost) column of each row
    let lon_1 = view.get_float64_endian(offset + SUBGRID_ELON, is_le); // Longitude of the last (typically easternmost) column of each row

    let dlat = view.get_float64_endian(offset + SUBGRID_DLAT, is_le); // Signed distance between two consecutive rows
    let dlon = view.get_float64_endian(offset + SUBGRID_DLON, is_le); // Signed distance between two consecutive columns

    // As defined by https://web.archive.org/web/20140127204822if_/http://www.mgs.gov.on.ca:80/stdprodconsume/groups/content/@mgs/@iandit/documents/resourcelist/stel02_047447.pdf (pg 30)
    let rows = (((lat_1 - lat_0) / dlat).abs() + 1.0).floor() as usize;
    let cols = (((lon_0 - lon_1) / dlon).abs() + 1.0).floor() as usize;
    let num_nodes = view.get_int32_endian(offset + SUBGRID_GSCOUNT, is_le) as usize;

    if num_nodes != (rows * cols) {
        return Err(Error::Ntv2InvalidGridFormat(
            "Number of nodes does not match the grid size",
        ));
    }

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

    Ok((header, rows, cols))
}

// NTv2 nodes are order from SE to NW! Here we break them up into rows and columns
// reversing the order so we end up with a grid ordered from NW to SE.
//         NW=(r₀:c₀)      NE=(r₀:cₙ)           SW=(rₙ:c₀)      SE=(rₙ:cₙ)
//      [[[lat₀, lon₀]...[latₙ, lonₙ]]ᵣ₀,..., [[lat₀, lon₀]...[latₙ, lonₙ]]ᵣₙ]
//
// lat/lon values are in seconds-of-arc as is expected by the Gravsoft reader in Rust Geodesy
fn read_subgrid_grid(
    view: &DataView,
    offset: usize,
    is_le: bool,
    cols: usize,
) -> Result<Vec<Vec<[f64; 2]>>> {
    let num_nodes = view.get_int32_endian(offset + SUBGRID_GSCOUNT, is_le) as usize;

    let grid_start_offset = offset + HEADER_SIZE;
    let grid_end_offset = grid_start_offset + num_nodes * NODE_SIZE;

    if grid_end_offset > view.byte_length() {
        return Err(Error::Ntv2InvalidGridFormat("Too Short"));
    }

    let grid = (0..num_nodes)
        .map(|i| {
            let offset = grid_start_offset + i * NODE_SIZE;
            let lat_corr = view.get_float32_endian(offset + NODE_LAT_CORRN, is_le);
            // Swap signs because we're reversing the grid. Makes it work with Rust Geodesy grid interpolation
            let lon_corr = -view.get_float32_endian(offset + NODE_LON_CORRN, is_le);
            [lat_corr.into(), lon_corr.into()]
        })
        .rev() // Because only the insane work SE to NW!
        .collect::<Vec<[f64; 2]>>()
        .chunks(cols) // Chunk into rows
        .map(|chunk| chunk.to_vec())
        .collect::<Vec<Vec<[f64; 2]>>>();

    Ok(grid)
}

/// Converts the `NTv2Grid` into a pseudo binary Gravsoft compatible with Rust Geodesy
/// The Gravsoft parser in geodesy_rs expects a text buffer made up of rows with f64 values delimited by spaces.
/// Rows are ordered North to South and East to West.
/// The first row contains the header values which should be in degrees
/// The subsequent rows contain the grid values which should be in seconds-of-arc.
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

fn read_string(view: &DataView, offset: usize, length: usize) -> String {
    let mut string = String::with_capacity(length);
    for i in 0..length {
        let char_code = view.get_uint8(offset + i);
        string.push(char_code as char);
    }

    string.trim().to_string()
}
