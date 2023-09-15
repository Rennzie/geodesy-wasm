use js_sys::DataView;
use std::collections::btree_map::IntoIter;
use std::collections::BTreeMap;
use std::iter::Map;
use wasm_bindgen::prelude::*;
type Blob = Vec<u8>;
use crate::error::WasmResult;

/// A helper class for loading gridshift files into the `Ctx` class.
/// The class is consumed in the `Ctx` new method to ensure we don't double the memory usage.
/// It is therefore not safe to use this class after the `Ctx` has been created.
#[wasm_bindgen]
pub struct RawGrids(BTreeMap<String, Blob>);

#[wasm_bindgen]
impl RawGrids {
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
    pub fn add(&mut self, key: &str, data_view: DataView) -> WasmResult<()> {
        let mut result = Vec::with_capacity(data_view.byte_length() as usize);

        for i in 0..data_view.byte_length() {
            let byte = data_view.get_uint8(i);
            result.push(byte);
        }
        self.0.insert(key.to_owned(), result);
        Ok(())
    }
}

impl IntoIterator for RawGrids {
    type Item = (String, Blob);
    type IntoIter = Map<IntoIter<String, Blob>, fn((String, Blob)) -> (String, Blob)>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter().map(|(k, v)| (k, v))
    }
}
