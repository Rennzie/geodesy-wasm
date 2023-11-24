//! Test suite for the Web and headless browsers.

#![cfg(target_arch = "wasm32")]

extern crate wasm_bindgen_test;
use geodesy_wasm::{error::Error, geodesy::coordinate::Coordinates};

use float_eq::assert_float_eq;
use geodesy_rs::prelude::*;
use wasm_bindgen_test::*;

#[wasm_bindgen_test]
fn coordinates_from_raw() -> Result<(), Error> {
    // Errors if the buffer length is not a multiple of 4
    assert!(Coordinates::from_raw(vec![0.0, 0.0, 0.0]).is_err());

    let london = vec![544748.0, 258372.0, 9.61, 0.0, 100.0, 5000.0, 10.0, 0.0];
    let raw = Coordinates::from_raw(london.clone());
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

#[wasm_bindgen_test]
fn coordinates_from_cart() -> Result<(), Error> {
    // Errors if the buffer length is not a multiple of 4
    assert!(Coordinates::from_cart(vec![0.0, 0.0, 0.0]).is_err());

    let london = vec![544748.0, 258372.0, 9.61, 0.0, 100.0, 5000.0, 10.0, 0.0];

    let cart = Coordinates::from_cart(london.clone());
    assert!(cart.is_ok());

    match cart {
        Ok(cart) => {
            assert_eq!(cart.len(), 2);
            assert_eq!(cart.get_coord(0), Coor4D([544748.0, 258372.0, 9.61, 0.0]));
            assert_eq!(cart.get_coord(1), Coor4D([100.0, 5000.0, 10.0, 0.0]));

            // Check that we convert to Float64 correctly
            let float64 = cart.into_array();
            assert_eq!(float64.length(), 8);
            for (i, val) in london.iter().enumerate() {
                assert_float_eq!(float64.get_index(i as u32), *val, abs_all <= 1e-6);
            }
        }
        Err(_) => panic!("Error creating Coordinates from raw buffer"),
    }

    Ok(())
}

#[wasm_bindgen_test]
fn coordinates_from_geo() -> Result<(), Error> {
    // Errors if the buffer length is not a multiple of 4
    assert!(Coordinates::from_geo(vec![0.0, 0.0, 0.0]).is_err());

    let london = vec![-0.09, 51.505, 10.0, 0.0, 0.1, 50.0, 10.0, 0.0];
    let geo = Coordinates::from_geo(london.clone());
    assert!(geo.is_ok());

    match geo {
        Ok(geo) => {
            assert_eq!(geo.len(), 2);

            assert_float_eq!(geo.get_coord(0)[0], 0.898931831, abs_all <= 1e-6);
            assert_float_eq!(geo.get_coord(0)[1], -0.001570796, abs_all <= 1e-6);
            assert_float_eq!(geo.get_coord(0)[2], 10.0, abs_all <= 1e-6);
            assert_float_eq!(geo.get_coord(0)[3], 0.0, abs_all <= 1e-6);

            assert_float_eq!(geo.get_coord(1)[0], 0.872665, abs_all <= 1e-6);
            assert_float_eq!(geo.get_coord(1)[1], 0.00174533, abs_all <= 1e-6);
            assert_float_eq!(geo.get_coord(1)[2], 10.0, abs_all <= 1e-6);
            assert_float_eq!(geo.get_coord(1)[3], 0.0, abs_all <= 1e-6);

            // Check that we convert to Float64 correctly
            let float64 = geo.into_geo_array();
            assert_eq!(float64.length(), 8);
            for (i, val) in london.iter().enumerate() {
                assert_float_eq!(float64.get_index(i as u32), *val, abs_all <= 1e-6);
            }
        }
        Err(_) => panic!("Error creating Coordinates from raw buffer"),
    }

    Ok(())
}

#[wasm_bindgen_test]
fn coordinates_from_gis() -> Result<(), Error> {
    // Errors if the buffer length is not a multiple of 4
    assert!(Coordinates::from_geo(vec![0.0, 0.0, 0.0]).is_err());

    let london = vec![51.505, -0.09, 10.0, 0.0, 50.0, 0.1, 10.0, 0.0];

    let gis = Coordinates::from_gis(london.clone());
    assert!(gis.is_ok());

    match gis {
        Ok(gis) => {
            assert_eq!(gis.len(), 2);

            assert_float_eq!(gis.get_coord(0)[0], 0.898931831, abs_all <= 1e-6);
            assert_float_eq!(gis.get_coord(0)[1], -0.001570796, abs_all <= 1e-6);
            assert_float_eq!(gis.get_coord(0)[2], 10.0, abs_all <= 1e-6);
            assert_float_eq!(gis.get_coord(0)[3], 0.0, abs_all <= 1e-6);

            assert_float_eq!(gis.get_coord(1)[0], 0.872665, abs_all <= 1e-6);
            assert_float_eq!(gis.get_coord(1)[1], 0.00174533, abs_all <= 1e-6);
            assert_float_eq!(gis.get_coord(1)[2], 10.0, abs_all <= 1e-6);
            assert_float_eq!(gis.get_coord(1)[3], 0.0, abs_all <= 1e-6);

            // Check that we convert to Float64 correctly
            let float64 = gis.into_gis_array();
            for (i, val) in london.iter().enumerate() {
                assert_float_eq!(float64.get_index(i as u32), *val, abs_all <= 1e-6);
            }
        }
        Err(_) => panic!("Error creating Coordinates from raw buffer"),
    }

    Ok(())
}
