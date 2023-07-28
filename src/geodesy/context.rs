use geodesy_rs::prelude::{Coordinate, OpHandle};
use geodesy_rs::Coord as RgCoord;
use geodesy_rs::*;
use wasm_bindgen::prelude::*;

// TODO: Find a way to use the geodesy_rs Coord type instead of a custom coord
#[wasm_bindgen]
pub struct Coord {
    x: f64,
    y: f64,
    z: Option<f64>,
}

#[wasm_bindgen]
impl Coord {
    #[wasm_bindgen(constructor)]
    pub fn new(x: f64, y: f64, z: Option<f64>) -> Self {
        Self { x, y, z }
    }
}

#[wasm_bindgen]
pub struct Ctx {
    context: Minimal,
    op_handle: OpHandle,
}

#[wasm_bindgen]
impl Ctx {
    #[wasm_bindgen(constructor)]
    pub fn new(&mut self, definition: &str) -> Self {
        let mut context = Minimal::new();

        let op_handle = context.op(definition).unwrap();
        Self {
            context: Minimal::new(),
            op_handle,
        }
    }

    #[wasm_bindgen(js_name = fromProjPipeline)]
    pub fn from_proj_pipeline(&mut self, _pipeline: &str) -> Result<Ctx, JsValue> {
        // Requires a proj_string to geodesy_rs conversion lexer.
        todo!()
    }

    // operands is a list of (x, y, z) or (x, y) values in javascript.
    // Goals should be to avoid copying data between wasm and javascript.
    // The returned value is the number of coordinates processed.
    // The operands are mutated in place.
    #[wasm_bindgen] // todo: make operands a vector of Coord.
    pub fn forward(self, operands: &mut Coord) -> Result<Coord, JsValue> {
        let mut coords = [RgCoord::gis(
            operands.x,
            operands.y,
            operands.z.unwrap_or(0.0),
            0.0,
        )];
        self.context
            .apply(self.op_handle, Direction::Fwd, &mut coords)
            .unwrap();

        let coord = coords[0];
        let z = if let Some(z) = operands.z {
            Some(z)
        } else {
            None
        };
        Ok(Coord::new(coord[0], coord[1], z))
    }

    #[wasm_bindgen]
    pub fn inverse(self, _operands: &mut Coord) -> Result<usize, JsValue> {
        todo!()
    }
}
