const geodesy = require("./pkg/node/index");
geodesy.set_panic_hook();

const coord = new geodesy.WasmCoord(59, 18, 0);
const out = geodesy.reprojectCoord(
  coord,
  "utm",
  geodesy.ReprojectDirection.Fwd
);
console.log(`Reprojected coord: ${out.x} ${out.y} ${out.z}`);
