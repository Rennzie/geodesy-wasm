use geodesy_rs::CoordinateSet;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub enum CoordDimension {
    Two,
    Three,
}

// LEARN: This will be a reference to the CoordSetBuffer in wasm memory
// Because it's a struct, it CAN be mutated by rust code - which is useful as an argument to forward and inverse transformation in the Ctx.
// It would be nice to move this to coordinates but that means the inner struct
// MUST be public which errors on wasm_bindgen macro
// LEARN: Adding skip to pub means we don't error but Js Cannot read the buffer directly, which is ok, it's not supposed to
/// A wrapper around the `CoordSetBuffer` struct which allows for a
/// mutable pointer to wasm memory.
#[wasm_bindgen]
pub struct CoordBuffer(#[wasm_bindgen(skip)] pub CoordSetBuffer);

#[wasm_bindgen]
impl CoordBuffer {
    /// Creates a new CoordBuffer from a JS array of f64 values.
    /// The array should be flat and contain either 2D or 3D coordinates.
    /// Note: If you are providing angular coordinates, they should be in radians AND
    /// it's assumed they are in the order (longitude, latitude, height) OR (easting, northing, height) for cartesian coords.
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

        for (i, v) in self.0.buffer.iter().enumerate() {
            array.set_index(i as u32, *v);
        }

        array
    }
}

/// A flat buffer of f64 values representing coordinates.
/// Currently supports 2D and 3D coordinates.
/// Note: If you are providing angular coordinates coordinates, they should be in radians.
pub struct CoordSetBuffer {
    pub buffer: Vec<f64>,
    dimension: CoordDimension,
}

impl CoordSetBuffer {
    pub fn new(coord_buffer: Vec<f64>, dimension: CoordDimension) -> CoordSetBuffer {
        CoordSetBuffer {
            buffer: coord_buffer,
            dimension,
        }
    }
}

impl CoordinateSet for CoordSetBuffer {
    fn len(&self) -> usize {
        match self.dimension {
            CoordDimension::Two => self.buffer.len() / 2,
            CoordDimension::Three => self.buffer.len() / 3,
        }
    }

    fn get_coord(&self, index: usize) -> geodesy_rs::Coor4D {
        let multiplier = match self.dimension {
            CoordDimension::Two => 2,
            CoordDimension::Three => 3,
        };

        let start = index * multiplier;

        let first = self.buffer[start];
        let second = self.buffer[start + 1];

        match self.dimension {
            CoordDimension::Two => geodesy_rs::Coor4D::raw(first, second, 0., 0.),
            CoordDimension::Three => {
                geodesy_rs::Coor4D::raw(first, second, self.buffer[start + 2], 0.)
            }
        }
    }

    fn set_coord(&mut self, index: usize, value: &geodesy_rs::Coor4D) {
        let multiplier = match self.dimension {
            CoordDimension::Two => 2,
            CoordDimension::Three => 3,
        };

        let start = index * multiplier;

        self.buffer[start] = value.0[0];
        self.buffer[start + 1] = value.0[1];

        match self.dimension {
            CoordDimension::Two => {}
            CoordDimension::Three => self.buffer[start + 2] = value.0[2],
        };
    }
}
