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
console.time("Create Ctx");
const epsg7405toWebmercCtx = new geodesy.Ctx(bngTo3857WithoutGridshift, "gsb");
console.timeEnd("Create Ctx");

console.time("Create CoordBuffer");
const flatCoordPtr = new geodesy.CoordBuffer(
  bngControlCoords,
  geodesy.CoordDimension.Three
);
console.timeEnd("Create CoordBuffer");

console.time("forward");
epsg7405toWebmercCtx.forward(flatCoordPtr);
console.timeEnd("forward");
epsg7405toWebmercCtx.free();

console.time("toArray");
const jsArray7405toWebmerc = flatCoordPtr.toArray();
console.timeEnd("toArray");

console.log("Converted Coords");
for (let i = 0; i < jsArray7405toWebmerc.length; i += 3) {
  console.log(
    jsArray7405toWebmerc[i],
    jsArray7405toWebmerc[i + 1],
    jsArray7405toWebmerc[i + 2]
  );
}
console.log("Diff Expected Coords");
logCoordDiff(jsArray7405toWebmerc, expectedBngTo3857OutputWithoutGridshift);

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
  +step +proj=webmerc +lat_0=0 +lon_0=0 +x_0=0 +y_0=0 +ellps=WGS84`;

const fs = require("fs");

const gridShiftFile = fs.readFileSync(
  "/Users/sean/Documents/Project-test-data/gridshifts/OSTN15_NTv2_OSGBtoETRS.gsb"
);
const dataView = new DataView(gridShiftFile.buffer);

console.time("Create Ctx with GSB");
const epsg7405toGridCtx = new geodesy.Ctx(
  bngTo3857WithGridshift,
  "OSTN15_NTv2_OSGBtoETRS.gsb",
  dataView
);
console.timeEnd("Create Ctx with GSB");

const flatCoord7405Ptrv2 = new geodesy.CoordBuffer(
  bngControlCoords,
  geodesy.CoordDimension.Three
);

console.time("forward");
epsg7405toGridCtx.forward(flatCoord7405Ptrv2);
console.timeEnd("forward");

console.time("toArray");
const jsArray7405toWebmercV2 = flatCoord7405Ptrv2.toArray();
console.timeEnd("toArray");

console.log("Converted Coords");
for (let i = 0; i < jsArray7405toWebmercV2.length; i += 3) {
  console.log(
    jsArray7405toWebmercV2[i],
    jsArray7405toWebmercV2[i + 1],
    jsArray7405toWebmercV2[i + 2]
  );
}
console.log("Diff Expected Coords");
logCoordDiff(jsArray7405toWebmercV2, expectedBngTo3857OutputWithGridshift);
