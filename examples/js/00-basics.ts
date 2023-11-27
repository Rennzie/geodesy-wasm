import {
  GeodesyWasm,
  Geodesy,
  CoordTuple2D,
  CoordTuple3D,
} from '../../pkg/node/index';
import {log, logCoordDiff, logCoordinates} from './utils';
GeodesyWasm.set_panic_hook();
GeodesyWasm.init_console_logger();

const geodesyRsLink = `${'\u001b]8;;https://github.com/busstoptaktik/geodesy/blob/main/examples/00-transformations.rs\u0007'}00-transformations.rs${'\u001b]8;;\u0007'}`;

function main() {
  log.white(`
   ==================================
      00 - Basic Transformations
   ==================================
   This example tries to replicate that of Rust Geodesy  ${geodesyRsLink} example in a geodesy-wasm native way.
  `);

  // First we create a context and by providing a transformation definition.
  // In this case a simple transformation between WGS84 and UTM zone 32.
  // We'll be using angular values in degrees as input so we must include a unit convert in the definition.
  let ctx = new Geodesy('utm zone=32');

  // We'll need some coordinates to test with.
  // Geodesy-wasm can take both projected and geographic coordinates.
  // Coordinates MUST be in GIS ordering convention - latitude before longitude.
  // Or in the case of projected coordinates - easting before northing.

  let cph: CoordTuple2D | CoordTuple3D = [59.0, 18.0];
  let osl: CoordTuple2D | CoordTuple3D = [60.0, 10.0];
  let sth: CoordTuple2D | CoordTuple3D = [59.0, 18.0];
  let hel: CoordTuple2D | CoordTuple3D = [60.0, 25.0];

  // For speed and to ease the cost of transferring data between JS and WASM geodesy-wasm operates
  // on arrays of coordinates. So we need to wrap our coordinates in an array.
  let nordics: (CoordTuple2D | CoordTuple3D)[] = [cph, osl, sth, hel];
  log.blue('----- Basic Transform WGS84 to UTM Zone 32 and back -----');
  log.green('\nOriginal (Nordic Capitals WGS84):');
  logCoordinates(nordics);

  // We're using geographic coordinates that need to be converted to radians first.
  // I have plans to add a convenience function for this - https://github.com/Rennzie/geodesy-wasm/issues/19 -
  // but in the mean time it must be handled manually by the user.
  nordics = nordics.map(c =>
    c.map(c => (c * Math.PI) / 180.0),
  ) as CoordTuple2D[];

  // NOTE: geodesy-wasm supports both 2D and 3D coordinates but not in the same transformation.
  // But it's not possible to mix 2D and 3D coordinates in the same transformation.
  // Doing:
  // const nordics =  [cph, osl, sth, hel, [59.0, 18.0, 0.0]];
  // Would result in an error.

  // Now we can transform our coordinates.
  // First forward:
  let forward = ctx.forward(nordics);
  log.blue('Forward (Nordic Capitals UTM Zone 32):');
  logCoordinates(forward);

  // And then back again:
  let reverse = ctx.inverse(forward);
  log.red('Reverse RAD:');
  logCoordinates(reverse);
  log.white(
    'The result is also in radians and need converting back to degrees:',
  );
  log.blue('Reverse DEG:');
  logCoordinates(
    reverse.map(c => c.map(c => (c * 180.0) / Math.PI)) as CoordTuple2D[],
  );

  // For convenience and testing there is a round trip method.
  // It returns the difference between the original and the round trip result.
  let roundTrip = ctx.roundTrip(nordics);
  log.yellow('Round Trip:');
  logCoordinates(roundTrip);

  // I've not entirely worked out how to free the wasm memory when we don't need the context any more.
  // Below is a hack to do that if you want to avoid leaking wasm memory.
  ctx['ctx'].free();

  log.blue('----- -------------------------------------- -----');

  log.yellow(
    '\n----- Advanced transformations with pipelines and PROJ pipelines -----',
  );

  // Now a slightly more complex case: Transforming the coordinates,
  // which we consider given in WGS84, back to the older ED50 datum.
  // The EPSG:1134 method handles that through a 3 parameter Helmert
  // transformation. But since the Helmert transformation works on
  // cartesian coordinates, rather than geographic, we need to add
  // pre- and post-processing steps, taking us from geographical
  // coordinates to cartesian, and back. Hence, we need a pipeline
  // of 3 steps:

  let pipeline =
    'cart ellps=intl | helmert x=-87 y=-96 z=-120 | cart inv=true ellps=GRS80';

  // Pipelines are first class citizen in Rust Geodesy which is great. It allows
  // us to avoid the manual conversions to radians and back again.
  // We'll do this by updating our pipeline with a `unitconvert` step.

  pipeline =
    'unitconvert xy_in=deg | cart ellps=intl | helmert x=-87 y=-96 z=-120 | cart inv=true ellps=GRS80 | unitconvert xy_out=deg';

  ctx = new Geodesy(pipeline);

  // Using our same nordics coordinates but this time in 3D
  cph = [59.0, 18.0, 10];
  osl = [60.0, 10.0, 10];
  sth = [59.0, 18.0, 10];
  hel = [60.0, 25.0, 10];
  nordics = [cph, osl, sth, hel];

  log.green('\nOriginal (Nordic Capitals WGS84):');
  logCoordinates([cph, osl, sth, hel]);

  // Since the forward transformation goes *from* ed50 to wgs84, we use
  // the inverse method to take us the other way, back in time to ED50
  const nordicsEd30Rg = ctx.inverse(nordics);

  log.green('Inverse to get (Nordic Capitals ED50):');
  logCoordinates(nordicsEd30Rg);

  ctx['ctx'].free();

  // If you're coming from PROJ you'll notice that the string above is a little different. Rust geodesy
  // has the advantage of being built brand new and can remove much of the boilerplate that PROJ string have.
  // Fortunately, we can also use PROJ string in Rust Geodesy, as it comes with a PROJ string parser.
  // There are caveats and limitations but for the most part it works well.
  // For our pipeline above we can use the equivalent PROJ string below:

  pipeline = `
      +proj=pipeline
        +step +proj=unitconvert +xy_in=deg
        +step +proj=cart +ellps=intl
        +step +proj=helmert +x=-87 +y=-96 +z=-120
        +step +proj=cart +inv=true +ellps=GRS80
        +step +proj=unitconvert +xy_out=deg
        `;

  ctx = new Geodesy(pipeline);

  // And we can use the same nordics coordinates as before.
  const nordicsEd30Proj = ctx.inverse(nordics);

  log.green('\nDiff PROJ string vs Rust Geodesy String:');
  logCoordDiff(nordicsEd30Rg, nordicsEd30Proj);

  // Geodesy wasm includes a [parser](https://github.com/busstoptaktik/geodesy/blob/c1c604c298bea4a80a5ce43276a3816898a10038/src/token/mod.rs#L169)
  // to convert PROJ strings to its native format which is re-exported by geodesy-wasm here.
  // However users must be aware of the [operator](https://github.com/busstoptaktik/geodesy/blob/main/ruminations/002-rumination.md) limitations of Rust Geodesy.
  // It is not battle hardened and small differences like only using a `k` in the `tmerc` instead of `k_0` will result in incorrect transforms.

  const parsed = GeodesyWasm.parseProj(pipeline);
  log.blue('\nParsed PROJ string:');
  log.yellow(parsed);

  log.yellow('----- ----------------------------------------------- -----');
}

main();
