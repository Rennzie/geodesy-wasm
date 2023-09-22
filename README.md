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
5. [x] Usage guide and examples
6. [ ] Documentation
7. [ ] Publish v1.0.0

Contributions very much welcome!

## 🚴 Usage

### 📦 Install

```sh
npm install geodesy-wasm
```

### 📝 Examples

See the [examples](./examples) folder for more examples.
They can be run with `bun run examples -n <example name (excluding extension)>`.

The examples are written to be run with Node or Bun, however with the exception of loading a gridshift they will translate directly to the browser.

```bash
bun run examples -n 00-basic
```

You can also run them directly with `ts-node`:

```bash
ts-node examples/js/00-basic.ts
```

#### Using ESM Modules in ObservableHQ

See [this notebook](https://observablehq.com/d/3ff9d9b8f0b5168a) for an example of using Geodesy-Wasm in ObservableHQ.

## Development

For convenience all scripts can be run with `bun <script>`. Make sure all the javascript related dependencies are installed with `bun install`. Rust dependencies are managed by [cargo](https://doc.rust-lang.org/cargo/) and don't require and explicit install step.

### 🛠️ Build the bindings

```sh
bun build

# Or for a specific target run. Only `node` and `bundler` targets are supported
TARGET=node ENV=debug bun build
```

### 🔧 Developing the JS wrapper

The wrapper is intended to abstract some of the complexities of using a wasm library - like dealing with pointers and managing WASM memory. It's also written to provide a more familiar API for JS developers - see [examples](#📝-examples).

**Note: This project uses [Bun](https://bun.sh/) for building and testing of the Javascript wrapper. Follow the instructions on the website to install it.**

To develop the wrapper you must first build the wasm bindings:

```sh
bun build:wrapper-dev
```

This will ensure that you get all the linting goodness from typescript while making changes to the wrapper.

Todo: tests etc

### 🔬 Testing

#### Wasm bindings

Written in rust so we use cargo

```bash
cargo test
```

#### JS wrapper

First we need to build the bindings:

```sh
bun build:wrapper-dev
```

Then we can run the tests:

```sh
bun test:wrapper-dev
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
