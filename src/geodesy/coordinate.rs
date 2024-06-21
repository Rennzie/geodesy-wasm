use crate::error::WasmResult;
use geodesy_rs::prelude::*;
use wasm_bindgen::prelude::*;

/// A flat array of 4D coordinates.
#[wasm_bindgen]
pub struct Coordinates(Vec<f64>);

#[wasm_bindgen]
impl Coordinates {
    /// Creates [Coordinates] from a JS array of f64 values.
    /// The array MUST be flat and contain sets of 4D coordinates
    /// ordered (longitude, latitude, height, time) OR (easting, northing, height, time).
    /// Angular coordinates are assumed to be in radians.
    /// Returns a pointer to the array in wasm memory.
    #[wasm_bindgen(constructor)]
    pub fn new(buffer: Vec<f64>) -> WasmResult<Coordinates> {
        if buffer.len() % 4 != 0 {
            return Err(JsError::new("Buffer length must be a multiple of 4"));
        }
        Ok(Coordinates(buffer))
    }

    /// Maps the raw buffer values to a [js_sys::Float64Array] and returns it,
    /// Coordinates are ordered (longitude, latitude, height, time) OR (easting, northing, height, time).
    /// Angular coordinates are in radians.
    /// Note: the WASM memory is freed on the way out and therefore no longer usable after this call.
    #[wasm_bindgen(js_name = toArray)]
    pub fn into_array(self) -> js_sys::Float64Array {
        let array = js_sys::Float64Array::new_with_length(self.0.len() as u32);

        for (i, v) in self.0.into_iter().enumerate() {
            array.set_index(i as u32, v);
        }

        array
    }
}

impl CoordinateSet for Coordinates {
    fn len(&self) -> usize {
        self.0.len() / 4
    }

    fn dim(&self) -> usize {
        4
    }

    fn get_coord(&self, index: usize) -> Coor4D {
        let start = index * 4;
        let mut result = Coor4D::origin();
        for i in 0..4 {
            result[i] = self.0[start + i];
        }

        result
    }

    fn set_coord(&mut self, index: usize, value: &Coor4D) {
        let start = index * 4;
        for i in 0..4 {
            self.0[start + i] = value.0[i];
        }
    }
}

// ----- T E S T S ---------------------------------------------------------------------

// Written in tests/coordinates.rs because of the wasm_bindgen_test macro
