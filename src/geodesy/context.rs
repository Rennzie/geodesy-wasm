use super::{coordinate::CoordBuffer, wasmcontext::WasmContext};
use crate::error::WasmResult;
use geodesy_rs::{authoring::parse_proj, prelude::*};
use wasm_bindgen::prelude::*;

/// A wrapper around a Geodesy Context
/// This is the main entry point for the library.
#[wasm_bindgen]
pub struct Geo {
    context: WasmContext,
    op_handle: OpHandle,
}

#[wasm_bindgen]
impl Geo {
    #[wasm_bindgen(constructor)]
    pub fn new(definition: &str) -> WasmResult<Geo> {
        let mut context = WasmContext::new();

        let mut geodesy_def = definition.to_owned();
        if definition.contains("+proj=") {
            geodesy_def = parse_proj(definition)?;
        }

        // Missing grids will error out here
        let op_handle = context.op(geodesy_def.as_str());
        match op_handle {
            Ok(op_handle) => Ok(Self { context, op_handle }),
            Err(e) => Err(JsError::new(&format!("{}", e))),
        }
    }

    /// A forward transformation of the coordinates in the buffer.
    #[wasm_bindgen]
    pub fn forward(&self, operands: &mut CoordBuffer) -> WasmResult<usize> {
        let converted = self.context.apply(self.op_handle, Fwd, &mut operands.0);

        match converted {
            Ok(c) => Ok(c),
            Err(e) => Err(JsError::new(&format!("{}", e))),
        }
    }

    /// An inverse transformation of the coordinates in the buffer.
    #[wasm_bindgen]
    pub fn inverse(&self, operands: &mut CoordBuffer) -> WasmResult<usize> {
        let converted = self.context.apply(self.op_handle, Inv, &mut operands.0);

        match converted {
            Ok(c) => Ok(c),
            Err(e) => Err(JsError::new(&format!("{}", e))),
        }
    }

    /// A convenience method for testing that a forward and inverse transformation
    #[wasm_bindgen(js_name = roundTrip)]
    pub fn round_trip(&self, operands: &mut CoordBuffer) -> WasmResult<usize> {
        let fwd_count = self.context.apply(self.op_handle, Fwd, &mut operands.0);
        let _inv_count = self.context.apply(self.op_handle, Inv, &mut operands.0);

        match fwd_count {
            Ok(c) => Ok(c),
            Err(e) => Err(JsError::new(&format!("{}", e))),
        }
    }
}
