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

// ------ Pipeline testing ------
// EPSG:7405 to EPSG:3857 without Gridshift

// prettier-ignore
const flatCoord7405 = [
  532545.5951859689, 180234.91854853655, 30,
  531721.7451008538, 185220.42514814128, 30,
  530884.804067163, 190762.03280916758, 30,
];

// prettier-ignore
const expectedCoord3857 = [
  -10018.754171394621, 6711113.243704713, 30,
  -11131.949079327358, 6719165.106882159, 30,
  -12245.143987260091, 6728120.968144314, 30,
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
console.log("EPSG:7405 TO EPSG:3857 without Gridshift");
console.log("--------------------------------------");
console.time("Create Ctx");
const epsg7405toWebmercCtx = new geodesy.Ctx(epsg7405toWebmercPipeline, "gsb");
console.timeEnd("Create Ctx");

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

console.log("Converted Coords");
for (let i = 0; i < jsArray7405toWebmerc.length; i += 3) {
  console.log(
    jsArray7405toWebmerc[i],
    jsArray7405toWebmerc[i + 1],
    jsArray7405toWebmerc[i + 2]
  );
}
console.log("Diff Expected Coords");
for (let i = 0; i < jsArray7405toWebmerc.length; i += 3) {
  console.log(
    jsArray7405toWebmerc[i] - expectedCoord3857[i],
    jsArray7405toWebmerc[i + 1] - expectedCoord3857[i + 1],
    jsArray7405toWebmerc[i + 2] - expectedCoord3857[i + 2]
  );
}

// ------ Gridshift testing ------
// +proj=pipeline
//   +step +inv +proj=tmerc +lat_0=49 +lon_0=-2 +k=0.9996012717 +x_0=400000
//         +y_0=-100000 +ellps=airy
//   +step +proj=hgridshift +grids=uk_os_OSTN15_NTv2_OSGBtoETRS.tif
//   +step +proj=webmerc +lat_0=0 +lon_0=0 +x_0=0 +y_0=0 +ellps=WGS84

console.log("\n");
console.log("EPSG:7405 TO EPSG:3857 with Gridshift");
console.log("--------------------------------------");

const epsg745Grid = `
   | tmerc inv lat_0=49 lon_0=-2 k_0=0.9996012717 x_0=400000 y_0=-100000 ellps=airy
   | gridshift grids=uk_os_OSTN15_NTv2_OSGBtoETRS.gsb
   | webmerc lat_0=0 lon_0=0 x_0=0 y_0=0 ellps=WGS84
   `;

const fs = require("fs");

const gridShiftFile = fs.readFileSync(
  "/Users/sean/Documents/Project-test-data/gridshifts/OSTN15_NTv2_OSGBtoETRS.gsb"
);
const dataView = new DataView(gridShiftFile.buffer);
geodesy.readGrid(dataView);

console.time("Create Ctx with GSB");
const epsg7405toGridCtx = new geodesy.Ctx(
  epsg745Grid,
  "uk_os_OSTN15_NTv2_OSGBtoETRS.gsb",
  dataView
);
console.timeEnd("Create Ctx with GSB");

const flatCoord7405Ptrv2 = new geodesy.CoordBuffer(
  flatCoord7405,
  geodesy.CoordDimension.Three
);

epsg7405toGridCtx.forward(flatCoord7405Ptrv2);
console.time("forward");
epsg7405toWebmercCtx.forward(flatCoord7405Ptrv2);
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
for (let i = 0; i < jsArray7405toWebmercV2.length; i += 3) {
  console.log(
    jsArray7405toWebmercV2[i] - expectedCoord3857[i],
    jsArray7405toWebmercV2[i + 1] - expectedCoord3857[i + 1],
    jsArray7405toWebmercV2[i + 2] - expectedCoord3857[i + 2]
  );
}
