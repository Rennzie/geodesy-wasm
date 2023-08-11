<div align="center">

  <h1><code>Geodesy Wasm</code></h1>

<strong>Wasm bindings for <a href="https://github.com/busstoptaktik/geodesy/tree/0.10">Geodesy</a>.</strong>

</div>

## About

Geodesy is a pure Rust library for doing coordinate transformations and conversion. Geodesy-Wasm is intended as an alternative to [Proj4.js](http://proj4js.org/) for performing coordinate transformations in the browsers.

_Note: Geodesy does not try to be a replacement for PROJ and only contains a fraction of the projects. If (like me) you only need a sub-set, this project may be of interest._

### Roadmap

1. [x] Implement bindings to the geodesy library
2. [x] Write a JS friendly wrapper around the bindings
3. [x] Proj string to geodesy_rs parser so that proj strings can be used with geodesy_wasm - completed by @busstoptaktik in the geodesy crate
4. [ ] NTv2 support
5. [ ] Usage guide and examples
6. [ ] Documentation
7. [ ] Publish v1.0.0

Contributions very much welcome!

## 🚴 Usage

### 🛠️ Build with `yarn build``

```sh
yarn build
```

### 🔬 Test in Headless Browsers with `wasm-pack test`

```sh
wasm-pack test --headless --chrome
```

### 🔧 Developing the JS wrapper

The wrapper is intended to abstract some of the complexities of using a wasm library - like dealing with pointers and manage some memory. The wrapper is written to provide a more familiar API for JS developers.

To develop the wrapper you must first build the wasm library:

```sh
yarn build:wrapper-dev
```

This will ensure that you get all the linting goodness from typescript.

Todo: tests etc

## Documentation

todo

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
