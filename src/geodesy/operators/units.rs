// Units are taken from PROJ https://github.com/OSGeo/PROJ/blob/master/src/units.cpp

use std::collections::HashMap;

pub struct Unit {
    pub factor: &'static str,
    pub description: &'static str,
    pub to_meters: f64,
}

/// Represents a set of linear units and their conversion to meters.
const LINEAR_UNITS: [(&str, Unit); 21] = [
    (
        "km",
        Unit {
            factor: "1000",
            description: "Kilometer",
            to_meters: 1000.0,
        },
    ),
    (
        "m",
        Unit {
            factor: "1",
            description: "Meter",
            to_meters: 1.0,
        },
    ),
    (
        "dm",
        Unit {
            factor: "1/10",
            description: "Decimeter",
            to_meters: 0.1,
        },
    ),
    (
        "cm",
        Unit {
            factor: "1/100",
            description: "Centimeter",
            to_meters: 0.01,
        },
    ),
    (
        "mm",
        Unit {
            factor: "1/1000",
            description: "Millimeter",
            to_meters: 0.001,
        },
    ),
    (
        "kmi",
        Unit {
            factor: "1852",
            description: "International Nautical Mile",
            to_meters: 1852.0,
        },
    ),
    (
        "in",
        Unit {
            factor: "0.0254",
            description: "International Inch",
            to_meters: 0.0254,
        },
    ),
    (
        "ft",
        Unit {
            factor: "0.3048",
            description: "International Foot",
            to_meters: 0.3048,
        },
    ),
    (
        "yd",
        Unit {
            factor: "0.9144",
            description: "International Yard",
            to_meters: 0.9144,
        },
    ),
    (
        "mi",
        Unit {
            factor: "1609.344",
            description: "International Statute Mile",
            to_meters: 1609.344,
        },
    ),
    (
        "fath",
        Unit {
            factor: "1.8288",
            description: "International Fathom",
            to_meters: 1.8288,
        },
    ),
    (
        "ch",
        Unit {
            factor: "20.1168",
            description: "International Chain",
            to_meters: 20.1168,
        },
    ),
    (
        "link",
        Unit {
            factor: "0.201168",
            description: "International Link",
            to_meters: 0.201168,
        },
    ),
    (
        "us-in",
        Unit {
            factor: "1/39.37",
            description: "U.S. Surveyor's Inch",
            to_meters: 100.0 / 3937.0,
        },
    ),
    (
        "us-ft",
        Unit {
            factor: "0.304800609601219",
            description: "U.S. Surveyor's Foot",
            to_meters: 1200.0 / 3937.0,
        },
    ),
    (
        "us-yd",
        Unit {
            factor: "0.914401828803658",
            description: "U.S. Surveyor's Yard",
            to_meters: 3600.0 / 3937.0,
        },
    ),
    (
        "us-ch",
        Unit {
            factor: "20.11684023368047",
            description: "U.S. Surveyor's Chain",
            to_meters: 79200.0 / 3937.0,
        },
    ),
    (
        "us-mi",
        Unit {
            factor: "1609.347218694437",
            description: "U.S. Surveyor's Statute Mile",
            to_meters: 6336000.0 / 3937.0,
        },
    ),
    (
        "ind-yd",
        Unit {
            factor: "0.91439523",
            description: "Indian Yard",
            to_meters: 0.91439523,
        },
    ),
    (
        "ind-ft",
        Unit {
            factor: "0.30479841",
            description: "Indian Foot",
            to_meters: 0.30479841,
        },
    ),
    (
        "ind-ch",
        Unit {
            factor: "20.11669506",
            description: "Indian Chain",
            to_meters: 20.11669506,
        },
    ),
];

// Returns a map of linear units and their conversion to meters.
// # Example
// ```
// use geodesy_wasm::geodesy::operators::units::linear_units_map;
// let map = linear_units_map();
// assert_eq!(map["km"].to_meters, 1000.0);
// ```
pub fn linear_units_map() -> HashMap<&'static str, &'static Unit> {
    LINEAR_UNITS
        .iter()
        .map(|&(name, ref unit)| (name, unit))
        .collect()
}
