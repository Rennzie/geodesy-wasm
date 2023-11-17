use crate::error::WasmResult;
use geodesy_rs::{Grid, Ntv2Grid};
use js_sys::{DataView, Uint8Array};
use std::{
    collections::{btree_map::IntoIter, BTreeMap},
    iter::Map,
    sync::Arc,
};
use wasm_bindgen::prelude::*;

/// A helper class for loading gridshift files into the [Geo] class.
/// The class is consumed in the [Geo#new()] method to ensure we don't double the memory usage.
/// It's therefore not safe to use this class after [Geo] has been constructed.
#[wasm_bindgen]
pub struct GridLoader(BTreeMap<String, Arc<dyn Grid>>);

#[wasm_bindgen]
impl GridLoader {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Self(BTreeMap::new())
    }

    /// The keys used to load the grid MUST be the same
    /// as the grid=<key> parameter in the definition string.
    ///
    /// Supported Grid Types:
    ///     - `NTv2` (.gsb)
    #[wasm_bindgen]
    pub fn load(&mut self, key: &str, data_view: DataView) -> WasmResult<()> {
        let array = Uint8Array::new(&data_view.buffer());
        let buf: Vec<u8> = array.to_vec();

        // TODO: Recognise other types of grids and create them here.
        // IDEA: To get more sophisticated we could
        // -  fetch from the network by identifying if the name is http etc
        //      -- either from the cdn or from a user defined url
        // - from IndexDB at a key/database that we pre-define

        let grid = Ntv2Grid::new(&buf)?;
        self.0.insert(key.to_owned(), Arc::new(grid));
        Ok(())
    }
}

impl IntoIterator for GridLoader {
    type Item = (String, Arc<dyn Grid>);
    type IntoIter = Map<
        IntoIter<String, Arc<dyn Grid>>,
        fn((String, Arc<dyn Grid>)) -> (String, Arc<dyn Grid>),
    >;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter().map(|(k, v)| (k, v))
    }
}
