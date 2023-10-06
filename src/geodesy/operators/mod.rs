use geodesy_rs::authoring::*;

mod longlat;
mod unitconvert;
pub mod units;

#[rustfmt::skip]
pub const ACCESSORY_OPERATORS: [(&str, OpConstructor); 2] = [
  ("unitconvert", OpConstructor(unitconvert::new)),
  ("longlat", OpConstructor(longlat::new))
];
