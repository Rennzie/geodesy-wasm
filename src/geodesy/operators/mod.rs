mod senmerc;

use geodesy_rs::authoring::*;

#[rustfmt::skip]
pub const ACCESSORY_OPERATORS: [(&str, OpConstructor); 1] = [
  ("senmerc", OpConstructor(senmerc::new)),
];
