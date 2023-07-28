<div align="center">

  <h1><code>Geodesy Wasm</code></h1>

<strong>Kick started with using <a href="https://github.com/rustwasm/wasm-pack">wasm-pack</a>.</strong>

</div>

## About

Lightweight wasm bindings to the [geodesy](https://github.com/busstoptaktik/geodesy/tree/0.10) library.
_Note: geodesy does not try to be a replacement for PROJ and only contains a fraction of the projects. If (like me) you only need a sub-set, this project may be of interest._

### Roadmap

1. [ ] Implement bindings to the geodesy library
2. [ ] Write a familiar interface for PROJ users who may want to use this project on the web
3. [ ] Proj string to geodesy_rs parser so that proj strings can be used with geodesy_wasm
4. [ ] Usage guide and examples
5. [ ] Documentation
6. [ ] Publish somewhere useful

Contributions very much welcome!

## 🚴 Usage

### 🛠️ Build with `wasm-pack build`

```sh
wasm-pack build
```

### 🔬 Test in Headless Browsers with `wasm-pack test`

```sh
wasm-pack test --headless --chrome
```

### 🎁 Publish to NPM with `wasm-pack publish`

```sh
wasm-pack publish
```

## License

Licensed under either of

- Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or <http://www.apache.org/licenses/LICENSE-2.0>)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or <http://opensource.org/licenses/MIT>)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally
submitted for inclusion in the work by you, as defined in the Apache-2.0
license, shall be dual licensed as above, without any additional terms or
conditions.
