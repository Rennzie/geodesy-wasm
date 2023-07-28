mod context;
// use geodesy_rs::prelude::*;
// use geodesy_rs::Direction;
// use wasm_bindgen::prelude::*;

// // Looks like we need a wasm conversion Direction.
// #[wasm_bindgen]
// pub enum ReprojectDirection {
//     /// `Fwd`: Indicate that a two-way operator, function, or method,
//     /// should run in the *forward* direction.
//     /// Geographic coordinates are projected to cartesian coordinates.
//     Fwd,

//     /// `Inv`: Indicate that a two-way operator, function, or method,
//     /// should run in the *inverse* direction.
//     /// Cartesian coordinates are converted to geographic coordinates.
//     Inv,
// }

// impl From<ReprojectDirection> for Direction {
//     fn from(value: ReprojectDirection) -> Self {
//         match value {
//             ReprojectDirection::Fwd => Self::Fwd,
//             ReprojectDirection::Inv => Self::Inv,
//         }
//     }
// }

// impl From<Direction> for ReprojectDirection {
//     fn from(value: Direction) -> Self {
//         match value {
//             Direction::Fwd => Self::Fwd,
//             Direction::Inv => Self::Inv,
//         }
//     }
// }

// #[wasm_bindgen(js_name = reprojectCoord)]
// pub fn reproject_coord(
//     coord: Coord,
//     definition: &str,
//     direction: ReprojectDirection,
// ) -> Result<Coord, JsValue> {
//     let mut context = Minimal::new();
//     let operation = context.op(definition).unwrap();
//     let coord = Coord::geo(coord.x, coord.y, coord.z, 0.0);
//     let mut data = [coord];
//     context.apply(operation, direction.into(), &mut data);
//     let new_coord = data[0];

//     Ok(Coord {
//         x: new_coord[0],
//         y: new_coord[1],
//         z: new_coord[2],
//     })
// }
