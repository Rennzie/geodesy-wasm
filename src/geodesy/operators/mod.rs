use geodesy_rs::authoring::*;

mod unitconvert;
mod units;

#[rustfmt::skip]
pub const ACCESSORY_OPERATORS: [(&str, OpConstructor); 1] = [
  ("unitconvert", OpConstructor(unitconvert::new))
];
