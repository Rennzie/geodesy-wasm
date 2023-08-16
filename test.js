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

function logCoordDiff(coordsA, coordsB) {
  for (let i = 0; i < coordsA.length; i += 3) {
    let x = colourString(coordsA[i] - coordsB[i]);
    let y = colourString(coordsA[i + 1] - coordsB[i + 1]);
    let z = colourString(coordsA[i + 2] - coordsB[i + 2]);

    console.log(`${x[0]} ${x[0]} ${z[0]}`, x[1], y[1], z[1]);
  }
}

// prettier-ignore
const bngControlCoords = [
  [544748.5367636156, 258372.49178149243, 9.61],
  [544750.8907704452, 258365.94195330486, 9.61],
]

// ------ Pipeline testing ------
console.log('EPSG:27700 TO EPSG:3857 without Gridshift');
console.log('--------------------------------------');

// Expected output generated with `echo <coords> | cct `bngTo3857WithoutGridshift`
// prettier-ignore
const expectedWithoutGsb = [
  13186.3825, 6837121.6345, 9.61,
  13189.9031, 6837110.8322, 9.61,
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
const withoutGridshiftResult = bngTo3857WithoutGridshiftCtx
  .forward(bngControlCoords)
  .flat();

bngTo3857WithoutGridshiftCtx.ctx.free();

console.log('Converted Coords');
for (let i = 0; i < withoutGridshiftResult.length; i += 3) {
  console.log(
    withoutGridshiftResult[i],
    withoutGridshiftResult[i + 1],
    withoutGridshiftResult[i + 2],
  );
}
console.log('Diff Expected Coords');
logCoordDiff(withoutGridshiftResult, expectedWithoutGsb);

// ------ Gridshift testing ------

console.log('\n');
console.log('EPSG:27700 TO EPSG:3857 with Gridshift');
console.log('--------------------------------------');

// Expected output generated using cct `echo <coords> | cct +proj=pipeline +step +inv +proj=tmerc +lat_0=49 +lon_0=-2 +k=0.9996012717 +x_0=400000 +y_0=-100000 +ellps=airy +step +proj=hgridshift +grids=./OSTN15_NTv2_OSGBtoETRS.gsb +step +proj=webmerc +lat_0=0 +lon_0=0 +x_0=0 +y_0=0 +ellps=WGS84
// prettier-ignore
const expectedWithGsb = [
  13004.3086, 6837202.7637, 9.6100,
  13007.8289, 6837191.9623, 9.6100
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
const withGridshiftResult = bngTo3857WithGridshiftCtx
  .forward(bngControlCoords)
  .flat();

console.log('Converted Coords');
for (let i = 0; i < withGridshiftResult.length; i += 3) {
  console.log(
    withGridshiftResult[i],
    withGridshiftResult[i + 1],
    withGridshiftResult[i + 2],
  );
}
console.log('Diff Expected Coords');
logCoordDiff(withGridshiftResult, expectedWithGsb);
