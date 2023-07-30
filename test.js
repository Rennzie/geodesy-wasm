const geodesy = require("./pkg/node/index");
geodesy.set_panic_hook();

const CONTROL_POINTS = {
  // {x: lon, y: lat, z: height} as expected by proj convention
  // eslint-disable-next-line @typescript-eslint/naming-convention
  EPSG_4326: { x: -0.09, y: 51.505, z: 30 },

  // eslint-disable-next-line @typescript-eslint/naming-convention
  EPSG_7405: { x: 532545.5951859689, y: 180234.91854853655, z: 30 },

  // {x: easting, y: northing, z: height}
  // eslint-disable-next-line @typescript-eslint/naming-convention
  EPSG_3857: { x: -10018.754171394621, y: 6711113.243704713, z: 30 },

  // {x, y, z}
  // eslint-disable-next-line @typescript-eslint/naming-convention
  EPSG_4978: {
    x: 3978226.9243723676,
    y: -6248.989379549275,
    z: 4968732.216466796,
  },
};

// ------ Array testing ------

// Flat array of 3D coordinates
// prettier-ignore
// const flatCoordArray = [
//   532545.5951859689, 180234.91854853655, 30,
//   531721.7451008538, 185220.42514814128, 30,
//   530884.804067163, 190762.03280916758, 30,
// ];

function degToRad(deg) {
  return (deg * Math.PI) / 180;
}

console.time("flatCoordArray");
// prettier-ignore
const flatCoordArray = new Float64Array([
     degToRad(-0.09), degToRad(51.505), 30,
     degToRad(-0.1), degToRad(51.55), 30,
     degToRad(-0.11), degToRad(51.6), 30,
]);
console.timeEnd("flatCoordArray");

// prettier-ignore
const flatCoord3857 = [
   -10018.754171394621, 6711113.243704713, 30,
   -11131.949079327358, 6719165.106882159, 30,
   -12245.143987260093, 6728120.968144314, 30,
]

console.time("Create CoordBuffer");
// Create a CoordBuffer from a flat array of Float64 in javascript.
// In a pointer to the CoordBuffer in Wasm memory
const coordBufferPtr = new geodesy.CoordBuffer(
  flatCoordArray,
  geodesy.CoordDimension.Three
);
console.timeEnd("Create CoordBuffer");

console.log("Original coord buffer:", coordBufferPtr);

console.time("Create Ctx");
const webmercCtx = new geodesy.Ctx(
  "webmerc +lat_0=0 +lon_0=0 +x_0=0 +y_0=0 +ellps=WGS84"
);
console.timeEnd("Create Ctx");

console.time("forward");
webmercCtx.forward(coordBufferPtr);
console.timeEnd("forward");
// Parse the buffer back into a JS array
console.log("Reprojected coord buffer:");

console.time("toArray");
const jsArray = coordBufferPtr.toArray();
console.timeEnd("toArray");
console.log(jsArray);
// LEARN: toArray consumes the coord buffer so we don't need to free it
// coordBuffer.free();
// console.log(coordBuffer);

webmercCtx.free();

// ------ Pipeline testing ------
// EPSG:7405 to EPSG:3857 without Gridshift

// prettier-ignore
const flatCoord7405 = [
   532545.5951859689, 180234.91854853655, 30,
   531721.7451008538, 185220.42514814128, 30,
   530884.804067163, 190762.03280916758, 30,
];

// +proj=pipeline
//   +step +inv +proj=tmerc +lat_0=49 +lon_0=-2 +k=0.9996012717 +x_0=400000
//         +y_0=-100000 +ellps=airy
//   +step +proj=push +v_3
//   +step +proj=cart +ellps=airy
//   +step +proj=helmert +x=446.448 +y=-125.157 +z=542.06 +rx=0.15 +ry=0.247
//         +rz=0.842 +s=-20.489 +convention=position_vector
//   +step +inv +proj=cart +ellps=WGS84
//   +step +proj=pop +v_3
//   +step +proj=webmerc +lat_0=0 +lon_0=0 +x_0=0 +y_0=0 +ellps=WGS84

const epsg7405toWebmercPipeline = `
 | tmerc inv lat_0=49 lon_0=-2 k_0=0.9996012717 x_0=400000 y_0=-100000 ellps=airy
 | push v_3
 | cart ellps=airy
 | helmert x=446.448 y=-125.157 z=542.06 rx=0.15 ry=0.247
       rz=0.842 s=-20.489 convention=position_vector
 | cart inv ellps=WGS84
 | pop v_3
 | webmerc lat_0=0 lon_0=0 x_0=0 y_0=0 ellps=WGS84
 `;

console.time("Create Ctx");
const epsg7405toWebmercCtx = new geodesy.Ctx(epsg7405toWebmercPipeline);
console.timeEnd("Create Ctx");
console.log("epsg7405toWebmercCtx:", epsg7405toWebmercCtx);

console.time("Create CoordBuffer");
const flatCoord7405Ptr = new geodesy.CoordBuffer(
  flatCoord7405,
  geodesy.CoordDimension.Three
);
console.timeEnd("Create CoordBuffer");

console.time("forward");
epsg7405toWebmercCtx.forward(flatCoord7405Ptr);
console.timeEnd("forward");

console.time("toArray");
const jsArray7405toWebmerc = flatCoord7405Ptr.toArray();
console.timeEnd("toArray");

console.log("jsArray7405toWebmerc:", jsArray7405toWebmerc);
