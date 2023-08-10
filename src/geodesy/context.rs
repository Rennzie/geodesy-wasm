use super::coordinate::CoordBuffer;
use super::wasmcontext::WasmContext;
use crate::error::WasmResult;
use geodesy_rs::prelude::*;
use log::debug;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct Ctx {
    context: WasmContext,
    op_handle: OpHandle,
}

#[wasm_bindgen]
impl Ctx {
    // TODO: Rework how the Ctx is initialized. Should check for grids and make sure there is a resource
    #[wasm_bindgen(constructor)]
    pub fn new(
        definition: &str,
        grid_key: &str,
        data_view: Option<js_sys::DataView>,
    ) -> WasmResult<Ctx> {
        let mut context = WasmContext::new();

        if let Some(data_view) = data_view {
            context.set_blob(grid_key, data_view)?;
        }

        let mut geodesy_def = definition.to_owned();
        if definition.contains("+proj=") {
            geodesy_def = parse_proj(definition);
        }
        // TODO: Remove: Only for debugging
        debug!("geodesy_def: {:?}", geodesy_def);

        let op_handle = context.op(geodesy_def.as_str());
        match op_handle {
            Ok(op_handle) => Ok(Self { context, op_handle }),
            Err(e) => Err(JsError::new(&format!("{}", e))),
        }
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
