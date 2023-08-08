use super::coordinate::CoordBuffer;
use crate::error::WasmResult;
use geodesy_rs::prelude::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct Ctx {
    context: Minimal,
    op_handle: OpHandle,
}

#[wasm_bindgen]
impl Ctx {
    #[wasm_bindgen(constructor)]
    pub fn new(definition: &str) -> WasmResult<Ctx> {
        let mut context = Minimal::new();

        let op_handle = context.op(definition);
        match op_handle {
            Ok(op_handle) => Ok(Self { context, op_handle }),
            Err(e) => Err(JsError::new(&format!("{}", e))),
        }
    }

    #[wasm_bindgen(js_name = fromProjPipeline)]
    pub fn from_proj_pipeline(&mut self, _pipeline: &str) -> WasmResult<Ctx> {
        // Requires a proj_string to geodesy_rs conversion lexer.
        todo!()
    }

    /// A forward transformation of the coordinates in the buffer.
    #[wasm_bindgen]
    pub fn forward(&self, operands: &mut CoordBuffer) -> WasmResult<usize> {
        let converted = self
            .context
            .apply(self.op_handle, Direction::Fwd, &mut operands.0);

        match converted {
            Ok(c) => Ok(c),
            Err(e) => Err(JsError::new(&format!("{}", e))),
        }
    }

    /// An inverse transformation of the coordinates in the buffer.
    #[wasm_bindgen]
    pub fn inverse(&self, operands: &mut CoordBuffer) -> WasmResult<usize> {
        let converted = self
            .context
            .apply(self.op_handle, Direction::Inv, &mut operands.0);

        match converted {
            Ok(c) => Ok(c),
            Err(e) => Err(JsError::new(&format!("{}", e))),
        }
    }
}
