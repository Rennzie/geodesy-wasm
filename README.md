<div align="center">

  <h1><code>Geodesy Wasm</code></h1>

<strong>Wasm bindings for <a href="https://github.com/busstoptaktik/geodesy">Rust Geodesy</a>.</strong>

</div>

## About

Geodesy is a pure Rust library for doing coordinate transformations and conversion. Geodesy-Wasm is, as the name suggests, wasm bindings for Rust Geodesy. It is intended as an alternative to [Proj4.js](http://proj4js.org/) for performing coordinate transformations in the browser. Today there is no alternative to Proj4.js and while for many projects this isn't an issue - it becomes challenging if you require more advanced transformations that only PROJ offers - notably via the pipeline operator. Rust Geodesy has very similar pipeline support to PROJ - the differences are easily worked around.

_Note: Please see the [warnings](https://github.com/busstoptaktik/geodesy?tab=readme-ov-file#concrete) in the Geodesy create regarding stability and architectural goals. In short Geodesy isn't a replacement for PROJ and only contains a fraction of the projects. However you if only need a sub-set of what PROJ has to offer this project may be of interest._

### Roadmap

1. [x] Implement bindings to the geodesy library
2. [x] Write a JS friendly wrapper around the bindings
3. [x] Proj string to geodesy_rs parser so that proj strings can be used with geodesy_wasm - completed by @busstoptaktik in the geodesy crate
4. [x] NTv2 support
   1. Being added in the geodesy crate [here](https://github.com/busstoptaktik/geodesy/pull/60) but is already being used in geodesy-wasm in place of gravsoft grid support.
5. [ ] Usage guide and examples
6. [ ] Documentation
7. [ ] Publish v1.0.0

Contributions very much welcome!

## 🚴 Usage

### 📦 Install

```sh
npm install geodesy-wasm
```

### 📝 Examples

#### Basic Usage

```js
import {Geodesy} from 'geodesy-wasm';

const definition = 'utm zone=32';

// Control coordinates in Degrees
let copenhagen = [55.0, 12.0];

// We're working with angular coordinates which MUST be convert to radians first.
copenhagen = copenhagen.map(x => x * (Math.PI / 180));

// Create a context for the given definition
const ctx = new Geodesy(definition);

// Geodesy expects an array of coordinates
const result = ctx.forward([copenhagen]);

console.log(result);
// [[ 6080642.1129675 1886936.9691340544 ]]

// And the inverse
const resultInv = ctx.inverse(result);

// The result is in radians which we'd need to convert back to degrees
console.log(resultInv);
// [[ 0.9599310885968819 0.2094395102393213 ]]

// 3D coordinates are also supported
const result3D = ctx.forward([[copenhagen[0], copenhagen[1], 100]]);
console.log(result3D);
// [[ 6080642.1129675, 1886936.9691340544, 100 ]]
```

#### Using a PROJ string

Geodesy wasm includes a [parser](https://github.com/busstoptaktik/geodesy/blob/c1c604c298bea4a80a5ce43276a3816898a10038/src/token/mod.rs#L169) to convert PROJ strings to its native format. However users must be aware of the [operator](https://github.com/busstoptaktik/geodesy/blob/main/ruminations/002-rumination.md) limitations of Rust Geodesy. It is not battle hardened and small differences like only using a `k` in the `tmerc` instead of `k_0` will result in incorrect transforms.

```js
import {Geodesy} from 'geodesy-wasm';
const definition = '+proj=utm +zone=32';

// Control coordinates in Degrees
let copenhagen = [55.0, 12.0];
// We're working with angular coordinates as input so we MUST convert to radians first.
copenhagen = copenhagen.map(x => x * (Math.PI / 180));

// Create a context for the given definition
const ctx = new Geodesy(definition);

// Geodesy expects and array of coordinates so we wrap our single point in an array
const result = ctx.forward([copenhagen]);

console.log(result);
// [ [ 6080642.1129675 1886936.9691340544 ] ]
```

#### Using a pipeline

```js
import {Geodesy} from 'geodesy-wasm';

const controlsCoordinates = [
  [13186.3825, 6837121.6345, 9.61],
  [13189.9031, 6837110.8322, 9.61],
];

// A pipeline using PROJ syntax converting BNG coordinates to Webmercator
const definition = `
+proj=pipeline
  +step +inv +proj=tmerc +lat_0=49 +lon_0=-2 +k_0=0.9996012717 +x_0=400000 +y_0=-100000 +ellps=airy
  +step +proj=webmerc +lat_0=0 +lon_0=0 +x_0=0 +y_0=0 +ellps=WGS84
  `;

const ctx = new Geodesy(definition);
const resultFwd = ctx.forward(controlsCoordinates);

console.log(resultFwd);
// [[ 13186.38247766841 6837121.634523826 9.61 ], [ 13189.903112418948 6837110.83220927 9.61 ]],

// For testing a `roundTrip` method is provided
const resultRoundTrip = ctx.roundTrip(controlsCoordinates);

for (let i = 0; i < resultRoundTrip.length; i++) {
  for (let j = 0; j < resultRoundTrip[i].length; j++) {
    console.assert(resultRoundTrip[i][j] === 0 + Math.E);
  }
}
```

#### Using a gridshift file

Currently only NTv2 (`.gsb`) grids are supported.

_Note: Natively Rust Geodesy only supports gravsoft grids however Geodesy Wasm uses a [branch](https://github.com/busstoptaktik/geodesy/pull/60) where NTv2 support is being considered. Hopefully in the near future both formats and more will be supported natively in Rust Geodesy_

```js
import {Geodesy} from 'geodesy-wasm';

// In some web environment using a component.

// We need a gridshift as a Dataview before building the context
const buffer = await fetch('<some-CDN-url>/OSTN15_NTv2_OSGBtoETRS.gsb').then(
  res => res.arrayBuffer(),
);
const dataView = new DataView(buf);

const controlsCoordinates = [
  [13186.3825, 6837121.6345, 9.61],
  [13189.9031, 6837110.8322, 9.61],
];

// NOTE: We need to modify the gridshift step to work with Rust Geodesy
// `+proj=hgridshift` is replaced by `+proj=gridshift`
const definition = `
  +proj=pipeline
    +step +inv +proj=tmerc +lat_0=49 +lon_0=-2 +k_0=0.9996012717 +x_0=400000 +y_0=-100000 +ellps=airy
    +step +proj=gridshift +grids=OSTN15_NTv2_OSGBtoETRS.gsb
    +step +proj=webmerc +lat_0=0 +lon_0=0 +x_0=0 +y_0=0 +ellps=WGS84
    `;

// From here everything is the same as the pipeline example
const ctx = new Geodesy(bngTo3857WithGridshift, {
  'OSTN15_NTv2_OSGBtoETRS.gsb': dataView,
});

const resultFwd = ctx.forward(controlsCoordinates);

console.log(resultFwd);
// [[ 13004.309008754391 6837202.757094237 9.61 ], [ 13007.829360281388 6837191.955741842 9.61 ]]
```

## Development

For convenience all scripts can be run with `yarn <script>`. Make sure all the javascript related dependencies are installed with `yarn install`. Rust dependencies are managed by [cargo](https://doc.rust-lang.org/cargo/) and don't require and explicit install step.

### 🛠️ Build the bindings

```sh
yarn build

# Or for a specific target run. Only `node` and `bundler` targets are supported
TARGET=node ENV=debug yarn build
```

### 🔧 Developing the JS wrapper

The wrapper is intended to abstract some of the complexities of using a wasm library - like dealing with pointers and managing WASM memory. It's also written to provide a more familiar API for JS developers - see [examples](#📝-examples).

To develop the wrapper you must first build the wasm bindings:

```sh
yarn build:wrapper-dev
```

This will ensure that you get all the linting goodness from typescript while making changes to the wrapper.

Todo: tests etc

### 🔬 Testing during alpha development

Nothing fancy here, just comparing output.

- Build the project with

```sh
TARGET=node ENV=debug npm run build
```

- Run the test script with

```sh
node ./test.js
```

---

## License

Geodesy-Wasm: Copyright (c) 2023 by Sean Rennie.

Licensed under either of

- Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or <http://www.apache.org/licenses/LICENSE-2.0>)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or <http://opensource.org/licenses/MIT>)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally
submitted for inclusion in the work by you, as defined in the Apache-2.0
license, shall be dual licensed as above, without any additional terms or
conditions.
