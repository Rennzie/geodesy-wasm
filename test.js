/**
 * FOR DEBUGGING PURPOSES ONLY.
 * - Will be removed in favour of actual tests once the library is stable.
 */
const {GeodesyWasm, Geodesy} = require('./pkg/node/index');
GeodesyWasm.set_panic_hook();
GeodesyWasm.init_console_logger();

function colourString(num) {
  if (Math.abs(num) < 0.001) {
    return ['\x1b[32m%s\x1b[0m', num];
  } else {
    return ['\x1b[31m%s\x1b[0m', num];
  }
}

function logCoordinate(coord) {
  console.log(coord[0], coord[1], coord[2]);
}

function logCoordinates(coords) {
  for (let i = 0; i < coords.length; i += 1) {
    logCoordinate(coords[i]);
  }
}

function logCoordDiff(coordsA, coordsB) {
  for (let i = 0; i < coordsA.length; i += 1) {
    let x = colourString(coordsA[i][0] - coordsB[i][0]);
    let y = colourString(coordsA[i][1] - coordsB[i][1]);
    let z = colourString(coordsA[i][2] - coordsB[i][2]);

    console.log(`${x[0]} ${x[0]} ${z[0]}`, x[1], y[1], z[1]);
  }
}

// prettier-ignore
const bngControlCoords = [
  [544748.5367636156, 258372.49178149243, 9.61],
  // 354062.25517 5719890.85218 9.61000 -- KP INV
  [544750.8907704452, 258365.94195330486, 9.61],
]

// ------ Pipeline testing ------
console.log('\nEPSG:27700 TO EPSG:3857 without Gridshift');
console.log('--------------------------------------');

// Expected output generated with `echo <coords> | cct `bngTo3857WithoutGridshift`
// prettier-ignore
const expectedWithoutGsb = [
  [13186.3825, 6837121.6345, 9.61],
// 13186.38248 6837121.63452 9.61 -- KP FWD
  [13189.9031, 6837110.8322, 9.61],
];

// Modifications:
// - tmerc: k changed to k_0
const bngTo3857WithoutGridshift = `
+proj=pipeline
  +step +inv +proj=tmerc +lat_0=49 +lon_0=-2 +k_0=0.9996012717 +x_0=400000 +y_0=-100000 +ellps=airy
  +step +proj=webmerc +lat_0=0 +lon_0=0 +x_0=0 +y_0=0 +ellps=WGS84
  `;
// +proj=pipeline
// +step +inv +proj=tmerc +lat_0=49 +lon_0=-2 +k=0.9996012717 +x_0=400000
//       +y_0=-100000 +ellps=airy
// +step +proj=webmerc +lat_0=0 +lon_0=0 +x_0=0 +y_0=0 +ellps=WGS84

const bngTo3857WithoutGridshiftCtx = new Geodesy(bngTo3857WithoutGridshift);
const withoutGridshiftResult =
  bngTo3857WithoutGridshiftCtx.forward(bngControlCoords);

console.log('Converted Coords');
logCoordinates(withoutGridshiftResult);
console.log('Diff Expected Coords');
logCoordDiff(withoutGridshiftResult, expectedWithoutGsb);

console.log('Round trip');
const roundTripWithout =
  bngTo3857WithoutGridshiftCtx.roundTrip(bngControlCoords);
logCoordinates(roundTripWithout);

bngTo3857WithoutGridshiftCtx.ctx.free();
// ------ Gridshift testing ------

console.log('\n');
console.log('EPSG:27700 TO EPSG:3857 with Gridshift');
console.log('--------------------------------------');

// Expected output generated using cct `echo <coords> | cct +proj=pipeline +step +inv +proj=tmerc +lat_0=49 +lon_0=-2 +k=0.9996012717 +x_0=400000 +y_0=-100000 +ellps=airy +step +proj=hgridshift +grids=./OSTN15_NTv2_OSGBtoETRS.gsb +step +proj=webmerc +lat_0=0 +lon_0=0 +x_0=0 +y_0=0 +ellps=WGS84
// prettier-ignore
const expectedWithGsb = [
  [13004.3086, 6837202.7637, 9.6100],
  [13007.8289, 6837191.9623, 9.6100]
];
// Modifications:
// - tmerc: k changed to k_0 [NOTE: PROJ accepts both k and k_0, RG is stricter and only accepts k_0]
// - hgridshift changed to gridshift
const bngTo3857WithGridshift = `
+proj=pipeline
  +step +inv +proj=tmerc +lat_0=49 +lon_0=-2 +k_0=0.9996012717 +x_0=400000 +y_0=-100000 +ellps=airy
  +step +proj=gridshift +grids=OSTN15_NTv2_OSGBtoETRS.gsb
  +step +proj=webmerc +lat_0=0 +lon_0=0 +x_0=0 +y_0=0 +ellps=WGS84
  `;

const fs = require('fs');
const gridShiftFile = fs.readFileSync('./OSTN15_NTv2_OSGBtoETRS.gsb');
const dataView = new DataView(gridShiftFile.buffer);

const bngTo3857WithGridshiftCtx = new Geodesy(bngTo3857WithGridshift, {
  'OSTN15_NTv2_OSGBtoETRS.gsb': dataView,
});
const withGridshiftResult = bngTo3857WithGridshiftCtx.forward(bngControlCoords);

console.log('Converted Coords');
const display = [...withGridshiftResult];
logCoordinates(display);

console.log('Diff Expected Coords');
logCoordDiff(display, expectedWithGsb);

console.log('Round trip');
const roundTrip = bngTo3857WithGridshiftCtx.roundTrip(bngControlCoords);
logCoordinates(roundTrip);

// ------ Gridshift testing Snake grid ------
// console.log('\n');
// console.log('HS2 Snakegrid TO EPSG:3857 with Gridshift');
// console.log('--------------------------------------');

// const snakeGrid = fs.readFileSync(
//   '/Users/sean/Documents/Project-test-data/gridshifts/HS2TN15_NTv2.gsb',
// );

// const hs2To3857Definition = `
// +proj=pipeline
//   +step +inv +proj=tmerc +lat_0=52.3 +lon_0=-1.5 +k_0=1 +x_0=198873.0046 +y_0=375064.3871 +ellps=WGS84
//   +step +proj=gridshift +grids=HS2TN15_NTv2.gsb
//   +step +proj=webmerc +lat_0=0 +lon_0=0 +x_0=0 +y_0=0 +ellps=WGS84
// `;

// const hs2SnakeTo3857Ctx = new Geodesy(hs2To3857Definition, {
//   'HS2TN15_NTv2.gsb': new DataView(snakeGrid.buffer),
// });

// const hs2SnakeControlCoords = [
//   [291924.45778025826, 287915.5278503553, 55.215899925299276],
// ];

// const hs2SnakeTo3857Result = hs2SnakeTo3857Ctx.forward(hs2SnakeControlCoords);

// console.log('Converted Coords');
// logCoordinates(hs2SnakeTo3857Result);

// const hs2SnakeTo3857Expected = [
//   [-14877.967314163603, 6714892.8385218205, 88.74904554140113],
// ];

// console.log('Diff Expected Coords');
// logCoordDiff(hs2SnakeTo3857Result, hs2SnakeTo3857Expected);
console.log('========================================');
console.log('================ END ===================');
console.log('========================================');

// const definition = `+proj=pipeline
//       +step +inv +proj=lcc +lat_0=50.85 +lon_0=-3.25 +lat_1=50.3 +lat_2=51.45 +x_0=372382.8292 +y_0=217764.7796 +ellps=GRS80
//       +step +proj=gridshift +grids=TN15-ETRS89-to-RBEPP12-IRF.gsb +ellps=GRS80
//       +step +proj=webmerc +lat_0=0 +lon_0=0 +x_0=0 +y_0=0 +ellps=WGS84`;
const DEG_TO_RAD = Math.PI / 180;
const RAD_TO_DEG = 180 / Math.PI;
const controlPoint = [355436.5844194045, 187050.05227720237]; //.map(v => v * DEG_TO_RAD);

const irf = [-3.4892308415257496, 50.57363537406664].map(v => v * DEG_TO_RAD);

const definition = `+proj=pipeline
+step +inv +proj=gridshift +ellps=GRS80 +grids=TN15-ETRS89-to-RBEPP12-IRF.gsb
`;
// +step +inv +proj=lcc +lat_0=50.85 +lon_0=-3.25 +lat_1=50.3 +lat_2=51.45 +x_0=372382.8292 +y_0=217764.7796 +ellps=GRS80 +units=m
// +step +proj=merc +a=6378137 +b=6378137 +lat_ts=0.0 +lon_0=0.0 +x_0=0.0 +y_0=0 +k=1.0 +units=m +no_defs

const newGsbFile = fs.readFileSync(
  '/Users/sean/Documents/Project-test-data/gridshifts/TN15-ETRS89-to-RBEPP12-IRF.gsb',
);

const newDataView = new DataView(newGsbFile.buffer);

const ctx = new Geodesy(definition, {
  'TN15-ETRS89-to-RBEPP12-IRF.gsb': newDataView,
});

const result = ctx.forward([irf]);
logCoordinates(result.map(o => o.map(v => v * RAD_TO_DEG)));

// `proj=pipeline
//     step inv proj=lcc lat_0=50.85 lon_0=-3.25 lat_1=50.3 lat_2=51.45 x_0=372382.8292 y_0=217764.7796 ellps=GRS80 units=m no_defs type=crs
//     step proj=longlat ellps=GRS80 nadgrids=/Users/sean/Documents/Project-test-data/gridshifts/TN15-ETRS89-to-RBEPP12-IRF.gsb no_defs
//     step proj=merc a=6378137 b=6378137 lat_ts=0.0 lon_0=0.0 x_0=0.0 y_0=0 k=1.0 units=m no_defs argc=32 pargc=30`
