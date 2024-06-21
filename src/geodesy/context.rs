use super::{coordinate::Coordinates, wasmcontext::WasmContext};
use crate::error::{Error, WasmResult};
use geodesy_rs::{authoring::*, ctx::OpHandle};
use wasm_bindgen::prelude::*;

/// A wrapper around a [geodesy_rs::Context]
/// This is the main entry point for the library.
#[wasm_bindgen]
pub struct Geo {
    context: WasmContext,
    definition: String,
    op_handle: Option<OpHandle>,
}

#[wasm_bindgen]
impl Geo {
    #[wasm_bindgen(constructor)]
    pub fn new(definition: &str) -> WasmResult<Geo> {
        let mut geodesy_def = definition.to_owned();
        if definition.contains("+proj=") {
            geodesy_def = parse_proj(definition)?;
        }

        Ok(Self {
            context: WasmContext::new(),
            definition: geodesy_def.to_string(),
            // We lazily initialize the op handle on first use
            op_handle: None,
        })
    }

    /// A forward transformation of the coordinates in the buffer.
    #[wasm_bindgen]
    pub fn forward(&mut self, operands: &mut Coordinates) -> WasmResult<usize> {
        let handle = self.op_handle()?;
        let converted = self.context.apply(handle, Fwd, operands);

        match converted {
            Ok(c) => Ok(c),
            Err(e) => Err(JsError::new(&format!("{}", e))),
        }
    }

    /// An inverse transformation of the coordinates in the buffer.
    #[wasm_bindgen]
    pub fn inverse(&mut self, operands: &mut Coordinates) -> WasmResult<usize> {
        let handle = self.op_handle()?;
        let converted = self.context.apply(handle, Inv, operands);

        match converted {
            Ok(c) => Ok(c),
            Err(e) => Err(JsError::new(&format!("{}", e))),
        }
    }

    /// A convenience method for testing that a forward and inverse transformation
    #[wasm_bindgen(js_name = roundTrip)]
    pub fn round_trip(&mut self, operands: &mut Coordinates) -> WasmResult<usize> {
        let handle = self.op_handle()?;
        let fwd_count = self.context.apply(handle, Fwd, operands);
        let inv_count = self.context.apply(handle, Inv, operands);

        match (fwd_count, inv_count) {
            (Ok(fc), Ok(ic)) => {
                if fc != ic {
                    return Err(JsError::new(&format!(
                        "Forward and Inverse counts do not match: {} != {}",
                        fc, ic
                    )));
                }
                Ok(fc)
            }
            (Err(fe), _) => Err(JsError::new(&format!("{}", fe))),
            (_, Err(ie)) => Err(JsError::new(&format!("{}", ie))),
        }
    }

    // For lazy initialization of the op handle
    // Primarily so we can load grids after the context is created
    fn op_handle(&mut self) -> Result<OpHandle, Error> {
        match self.op_handle {
            Some(op_handle) => Ok(op_handle),
            None => {
                let op_handle = self.context.op(self.definition.as_str())?;
                self.op_handle = Some(op_handle);
                Ok(op_handle)
            }
        }
    }
}
