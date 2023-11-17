use geodesy_rs::prelude::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub enum CoordDimension {
    Two,
    Three,
}

/// A wrapper around the `CoordSetBuffer` struct which allows for a
/// mutable pointer to wasm memory.
#[wasm_bindgen]
pub struct CoordBuffer(#[wasm_bindgen(skip)] pub CoordSetBuffer);

#[wasm_bindgen]
impl CoordBuffer {
    /// Creates a new [CoordBuffer] from a JS array of f64 values.
    /// The array should be flat and contain either 2D or 3D coordinates.
    /// Note: If you are providing angular coordinates, they MUST be in radians AND
    /// it's assumed they are in the order (longitude, latitude, height) OR (easting, northing, height).
    #[wasm_bindgen(constructor)]
    pub fn new(coord_buffer: Vec<f64>, dimension: CoordDimension) -> CoordBuffer {
        CoordBuffer(CoordSetBuffer::new(coord_buffer, dimension))
    }

    /// Maps the buffer from wasm memory to a JS array
    /// consuming the wasm memory on the way out.
    /// i.e we don't need to call `free()` on the pointer.
    #[wasm_bindgen(js_name = toArray)]
    pub fn into_array(self) -> js_sys::Float64Array {
        let array = js_sys::Float64Array::new_with_length(self.0.buffer.len() as u32);

        for (i, v) in self.0.buffer.into_iter().enumerate() {
            array.set_index(i as u32, v);
        }

        array
    }
}

/// A flat buffer of f64 values representing coordinates.
/// Currently supports 2D and 3D coordinate dimensions.
/// Note: If you are providing angular coordinates coordinates, they should be in radians.
pub struct CoordSetBuffer {
    pub buffer: Vec<f64>,
    dimensions: CoordDimension,
}

impl CoordSetBuffer {
    pub fn new(coord_buffer: Vec<f64>, dimensions: CoordDimension) -> CoordSetBuffer {
        CoordSetBuffer {
            buffer: coord_buffer,
            dimensions,
        }
    }

    pub fn with_2d(coord_buffer: Vec<f64>) -> CoordSetBuffer {
        CoordSetBuffer {
            buffer: coord_buffer,
            dimensions: CoordDimension::Two,
        }
    }

    pub fn with_3d(coord_buffer: Vec<f64>) -> CoordSetBuffer {
        CoordSetBuffer {
            buffer: coord_buffer,
            dimensions: CoordDimension::Three,
        }
    }

    pub fn dim(&self) -> usize {
        match self.dimensions {
            CoordDimension::Two => 2,
            CoordDimension::Three => 3,
        }
    }
}

impl CoordinateSet for CoordSetBuffer {
    fn len(&self) -> usize {
        self.buffer.len() / self.dim()
    }

    fn get_coord(&self, index: usize) -> Coor4D {
        let start = index * self.dim();
        let mut result = Coor4D::origin();
        for i in 0..self.dim() {
            result[i] = self.buffer[start + i];
        }

        result
    }

    fn set_coord(&mut self, index: usize, value: &Coor4D) {
        let start = index * self.dim();
        for i in 0..self.dim() {
            self.buffer[start + i] = value.0[i];
        }
    }
}
