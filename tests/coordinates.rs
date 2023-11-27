//! Test suite for the Web and headless browsers.

#![cfg(target_arch = "wasm32")]

extern crate wasm_bindgen_test;
use geodesy_wasm::{error::Error, geodesy::coordinate::Coordinates};

use float_eq::assert_float_eq;
use geodesy_rs::prelude::*;
use wasm_bindgen_test::*;

#[wasm_bindgen_test]
fn coordinates_new() -> Result<(), Error> {
    // Errors if the buffer length is not a multiple of 4
    assert!(Coordinates::new(vec![0.0, 0.0, 0.0]).is_err());

    let london = vec![544748.0, 258372.0, 9.61, 0.0, 100.0, 5000.0, 10.0, 0.0];
    let raw = Coordinates::new(london.clone());
    assert!(raw.is_ok());

    match raw {
        Ok(raw) => {
            assert_eq!(raw.len(), 2);
            assert_eq!(raw.get_coord(0), Coor4D([544748.0, 258372.0, 9.61, 0.0]));
            assert_eq!(raw.get_coord(1), Coor4D([100.0, 5000.0, 10.0, 0.0]));

            // Check that we convert to Float64 correctly
            let float64 = raw.into_array();
            assert_eq!(float64.length(), 8);
            for (i, val) in london.iter().enumerate() {
                assert_float_eq!(float64.get_index(i as u32), *val, abs_all <= 1e-6);
            }
        }
        Err(_) => panic!("Error creating Coordinates from raw buffer"),
    }

    Ok(())
}
