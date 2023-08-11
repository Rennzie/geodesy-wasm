/**
 * FOR DEBUGGING PURPOSES ONLY.
 * - Will be removed in favour of actual tests once the library is stable.
 */
const geodesy = require("./pkg/node/index");
geodesy.set_panic_hook();
geodesy.init_console_logger();

function logNumber(num) {
  if (Math.abs(num) < 0.001) {
    console.log("\x1b[32m%s\x1b[0m", num);
  } else {
    console.log("\x1b[31m%s\x1b[0m", num);
  }
}

function logCoordDiff(coordsA, coordsB) {
  for (let i = 0; i < coordsA.length; i += 3) {
    logNumber(coordsA[i] - coordsB[i]);
    logNumber(coordsA[i + 1] - coordsB[i + 1]);
    logNumber(coordsA[i + 2] - coordsB[i + 2]);
  }
}

class Geod {
  ctx;
  constructor(definition, gridKey = "gsb", dataView) {
    console.time("Create Ctx");
    this.ctx = new geodesy.Ctx(definition, gridKey, dataView);
    console.timeEnd("Create Ctx");
  }

  forward(coords) {
    console.time("Create CoordBuffer");
    const coordBufPtr = new geodesy.CoordBuffer(
      coords,
      geodesy.CoordDimension.Three
    );
    console.timeEnd("Create CoordBuffer");

    console.time("forward");
    this.ctx.forward(coordBufPtr);
    console.timeEnd("forward");

    console.time("toArray");
    const resArray = coordBufPtr.toArray();
    console.timeEnd("toArray");
    return resArray;
  }
}

// prettier-ignore
const bngControlCoords = [
  544748.5367636156, 258372.49178149243, 9.61,
  544750.8907704452, 258365.94195330486, 9.61,
]

// ------ Pipeline testing ------
// EPSG:27700 to EPSG:3857 without Gridshift

// Expected output generated with `echo <coords> | cct `bngTo3857WithoutGridshift`
// prettier-ignore
const expectedBngTo3857OutputWithoutGridshift = [
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

console.log("EPSG:7405 TO EPSG:3857 without Gridshift");
console.log("--------------------------------------");

const bngTo3857WithoutGridshiftCtx = new Geod(bngTo3857WithoutGridshift, "gsb");
const withoutGridshiftResult =
  bngTo3857WithoutGridshiftCtx.forward(bngControlCoords);

bngTo3857WithoutGridshiftCtx.ctx.free();

console.log("Converted Coords");
for (let i = 0; i < withoutGridshiftResult.length; i += 3) {
  console.log(
    withoutGridshiftResult[i],
    withoutGridshiftResult[i + 1],
    withoutGridshiftResult[i + 2]
  );
}
console.log("Diff Expected Coords");
logCoordDiff(withoutGridshiftResult, expectedBngTo3857OutputWithoutGridshift);

// ------ Gridshift testing ------

// Created with cct
// prettier-ignore
const expectedBngTo3857OutputWithGridshift = [
  13004.3086, 6837202.7637, 9.6100,
  13007.8289, 6837191.9623, 9.6100
];

console.log("\n");
console.log("EPSG:7405 TO EPSG:3857 with Gridshift");
console.log("--------------------------------------");

// Modifications:
// - tmerc: k changed to k_0
// - hgridshift changed to gridshift
const bngTo3857WithGridshift = `
+proj=pipeline
  +step +inv +proj=tmerc +lat_0=49 +lon_0=-2 +k_0=0.9996012717 +x_0=400000 +y_0=-100000 +ellps=airy
  +step +proj=gridshift +grids=OSTN15_NTv2_OSGBtoETRS.gsb
  +step +proj=webmerc +lat_0=0 +lon_0=0 +x_0=0 +y_0=0 +ellps=WGS84
  `;

const fs = require("fs");

const gridShiftFile = fs.readFileSync("./OSTN15_NTv2_OSGBtoETRS.gsb");
const dataView = new DataView(gridShiftFile.buffer);

console.time("Create Ctx with GSB");
const bngTo3857WithGridshiftCtx = new Geod(
  bngTo3857WithGridshift,
  "OSTN15_NTv2_OSGBtoETRS.gsb",
  dataView
);
const withGridshiftResult = bngTo3857WithGridshiftCtx.forward(bngControlCoords);

console.log("Converted Coords");
for (let i = 0; i < withGridshiftResult.length; i += 3) {
  console.log(
    withGridshiftResult[i],
    withGridshiftResult[i + 1],
    withGridshiftResult[i + 2]
  );
}
console.log("Diff Expected Coords");
logCoordDiff(withGridshiftResult, expectedBngTo3857OutputWithGridshift);
