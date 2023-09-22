import {GeodesyWasm, Geodesy} from '../../pkg/node/index';
import {log, logCoordDiff, logCoordinates} from './utils';
GeodesyWasm.set_panic_hook();
GeodesyWasm.init_console_logger();

async function main() {
  log.white(`
  ==================================
     01 - Gridshift Transformations
  ==================================
 `);

  // Geodesy has native support for Gravsoft gridshift files.
  // Proj4.js and PROJ have support for Ntv2 gridshift files which geodesy-wasm has experimental support for.
  // Follow this (https://github.com/busstoptaktik/geodesy/pull/60) PR to see how thing progress with getting it into Geodesy.

  // One key difference between Rust Geodesy and Geodesy-wasm is that the latter requires us to set all
  // dependencies for a specific transformation when creating a context. As you've seen in 00-basics.ts
  // we had to provide the definition at creation time.
  // The same is true for gridshift file. First we must fetch the grid, then create a context that uses it.

  // In this example we're using a locally stored grid, but in a browser you'd fetch if from a server.
  // That's the only difference between this example and what a browser implementation would look like.

  // First we load the grid. (Or fetch it from a server).
  const file = Bun.file('./js/fixtures/OSTN15_NTv2_OSGBtoETRS.gsb');
  const buf = Buffer.from(await file.arrayBuffer());
  // We need the grid as a DataView.
  const gsb = new DataView(buf.buffer);

  // Next we need a transformation definition that uses the grid.
  // We're using the OSGB grid designed for transforming between EPSG:27700 and WGS84.
  // Remember that this could also be a PROJ pipeline definition.
  const gsbPipelineDefinition = `
  | tmerc inv lat_0=49 lon_0=-2 k_0=0.9996012717 x_0=400000 y_0=-100000 ellps=airy
  | gridshift grids=OSTN15_NTv2_OSGBtoETRS.gsb
  | webmerc lat_0=0 lon_0=0 x_0=0 y_0=0 ellps=WGS84
  `;

  // Now we can create a context that uses the grid.
  // We provide the grid as a key-value pair where the key MUST match that used
  // by the `grids` parameter in the transformation definition.

  const ctx = new Geodesy(gsbPipelineDefinition, {
    'OSTN15_NTv2_OSGBtoETRS.gsb': gsb,
  });

  // And some coordinates to test with.
  // We'll us projected coordinates from central Cambridge in easting northing height order.
  const centralCambridgeCoords = [
    [544748.5367636156, 258372.49178149243, 9.61],
    [544750.8907704452, 258365.94195330486, 9.61],
  ];

  // And so we can validate the transforms, here is the result of doing the same transform
  // with PROJ
  const expected = [
    [13004.3086, 6837202.7637, 9.61],
    [13007.8289, 6837191.9623, 9.61],
  ];

  const forward = ctx.forward(centralCambridgeCoords);
  log.blue('Forward:');
  logCoordinates(forward);
  log.blue('Diff Expected:');
  logCoordDiff(forward, expected);

  // And then back again:
  const reverse = ctx.inverse(forward);
  log.red('Reverse:');
  logCoordinates(reverse);
  log.blue('Diff Original:');
  logCoordDiff(reverse, centralCambridgeCoords);

  // And finally a round trip check for sanity:
  const roundTrip = ctx.roundTrip(centralCambridgeCoords);
  log.yellow('Round Trip:');
  logCoordinates(roundTrip);
}

main();
