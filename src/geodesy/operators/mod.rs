use geodesy_rs::authoring::*;

mod noop;
mod unitconvert;
pub mod units;

#[rustfmt::skip]
pub const ACCESSORY_OPERATORS: [(&str, OpConstructor); 2] = [
  ("unitconvert", OpConstructor(unitconvert::new)),
  // As far as I can tell the `longlat` operator is a no-op.
  // - https://proj.org/en/9.3/operations/conversions/latlon.html
  // - https://github.com/OSGeo/PROJ/blob/2040e685f5ab9c2958b7b611f5aaafee21fed82f/src/projections/latlong.cpp#L94
  // My assumption is it's for historical reasons, so in geodesy-wasm it's just a noop.
  ("longlat", OpConstructor(noop::new))
];
