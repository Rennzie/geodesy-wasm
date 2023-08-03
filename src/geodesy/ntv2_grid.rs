// This NTv2 grid reader is based on the following documents:
// - https://web.archive.org/web/20140127204822if_/http://www.mgs.gov.on.ca:80/stdprodconsume/groups/content/@mgs/@iandit/documents/resourcelist/stel02_047447.pdf
// - http://mimaka.com/help/gs/html/004_NTV2%20Data%20Format.htm
// - https://github.com/Esri/ntv2-file-routines/blob/master/README.md
//
// And inspired by existing implementations in
// - https://github.com/proj4js/proj4js/blob/master/lib/nadgrid.js
// - https://github.com/3liz/proj4rs/blob/main/src/nadgrids/grid.rs

use crate::error::{Error, Result};
use geodesy_rs::context_authoring::Grid as RgGrid;
use js_sys::DataView;

const SEC_TO_RAD: f64 = 4.848_136_811_095_36e-6;

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

pub struct NTv2Grid {
    // Angular values are in radians. NTv2 values are in seconds of arc.
    lat_0: f64, // Latitude of the first (typically northernmost) row of the grid
    lat_1: f64, // Latitude of the last (typically southernmost) row of the grid
    lon_0: f64, // Longitude of the first (typically westernmost) column of each row
    lon_1: f64, // Longitude of the last (typically easternmost) column of each row
    dlat: f64,  // Signed distance between two consecutive rows
    dlon: f64,  // Signed distance between two consecutive columns
    bands: usize,
    offset: Option<usize>, // typically 0, but may be any number for externally stored grids
    grid: Vec<f32>,
}

impl NTv2Grid {
    /// Read a NTv2 grid from a `js_sys::DataView`.
    pub fn from_data_view(view: DataView) -> Result<NTv2Grid> {
        let is_le = view.get_int32_endian(8, true) == 11;

        // TODO: read the header magic (string) and compare it to NTV2_MAGIC
        let num_of_fields = view.get_int32_endian(8, is_le);
        if num_of_fields != 11 {
            return Err(Error::InvalidNtv2GridFormat(ERR_INVALID_HEADER).into());
        }

        let num_sub_grids = view.get_int32_endian(40, is_le) as usize;

        if num_sub_grids != 1 {
            // I'm not sure what we do if there are multiple subgrids.
            return Err(Error::UnsupportedNtv2("Contains more than one subgrid"));
        }

        read_ntv2_subgrid(view, NTV2_HEADER_SIZE, is_le)
    }

    /// Converts the `NTv2Grid` into the more useful Rust Geodesy `Grid` while consuming itself.
    pub fn into_grid(self) -> Result<RgGrid> {
        let header = [
            self.lat_1,
            self.lat_0,
            self.lon_0,
            self.lon_1,
            self.dlat,
            self.dlon,
            self.bands as f64,
        ];

        let rg_grid = RgGrid::plain(&header, Some(&self.grid), self.offset)?;
        Ok(rg_grid)
    }
}

fn read_ntv2_subgrid(view: DataView, offset: usize, is_le: bool) -> Result<NTv2Grid> {
    let lat_0 = view.get_float64_endian(offset + NTV2_SUBGRID_NLAT, is_le) * SEC_TO_RAD;
    let lat_1 = view.get_float64_endian(offset + NTV2_SUBGRID_SLAT, is_le) * SEC_TO_RAD;

    // Because NTv2 is made by Canadians they've flipped the signs on East(-) and West(+) -
    // probably because they're always have everything west of Greenwich.
    // Commonly East is negative and West is positive so we flip them here
    let lon_0 = -view.get_float64_endian(offset + NTV2_SUBGRID_WLON, is_le) * SEC_TO_RAD;
    let lon_1 = -view.get_float64_endian(offset + NTV2_SUBGRID_ELON, is_le) * SEC_TO_RAD;

    let dlat = view.get_float64_endian(offset + NTV2_SUBGRID_DLAT, is_le) * SEC_TO_RAD;
    let dlon = view.get_float64_endian(offset + NTV2_SUBGRID_DLON, is_le) * SEC_TO_RAD;

    let num_nodes = view.get_int32_endian(offset + NTV2_SUBGRID_GSCOUNT, is_le) as usize;

    let grid_start_offset = offset + NTV2_HEADER_SIZE;
    let grid_end_offset = grid_start_offset + num_nodes * NTV2_NODE_SIZE;

    if grid_end_offset > view.byte_length() {
        return Err(Error::InvalidNtv2GridFormat(ERR_INVALID_GRID_TO_SHORT).into());
    }

    let grid = (0..num_nodes)
        .map(|i| {
            let offset = grid_start_offset + i * NTV2_NODE_SIZE;
            let lat_correction = view.get_float32_endian(offset + NTV2_NODE_LAT_CORRN, is_le);
            // And again we flip the longitude sign so East is positive and West is negative.
            let lon_correction = -view.get_float32_endian(offset + NTV2_NODE_LON_CORRN, is_le);
            [
                lon_correction * SEC_TO_RAD as f32,
                lat_correction * SEC_TO_RAD as f32,
            ]
        })
        .flatten()
        .collect::<Vec<f32>>();

    let rows = ((lat_1 - lat_0) / -dlat + 1.5).floor() as usize;
    let cols = ((lon_1 - lon_0) / dlon + 1.5).floor() as usize;
    let bands = grid.len() / (rows * cols);

    Ok(NTv2Grid {
        lat_0,
        lat_1,
        lon_0,
        lon_1,
        dlat,
        dlon,
        bands,
        offset: None,
        grid,
    })
}
