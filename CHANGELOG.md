# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

<!--
### Added - for new features.
### Changed - for changes in existing functionality.
### Deprecated - for soon-to-be removed features.
### Removed - for now removed features.
### Fixed - for any bug fixes.
### Security - in case of vulnerabilities.
-->

<!-- ## [Unreleased] -->

## [Unreleased]

### Added

- Attribution for `OSTN15_NTv1_OSGBtoETRS.gsb` [#32](https://github.com/Rennzie/geodesy-wasm/issues/32)
- Support for 2D, 3D and 4D coordinates as tuples ([number, number]) or objects ({x, y, z, t}) to the wrapper
- Unstable support for loading a grid from the network
- Support for the `axisswap` operator via [Geodesy:#84](https://github.com/busstoptaktik/geodesy/pull/84)

### Changed

- Grids are stored in globally allocated heap memory [#34](https://github.com/Rennzie/geodesy-wasm/issues/34)
- Lazily initialise the operator so grids can be registered after creation
- Grids are loaded via standalone helper functions [#39](https://github.com/Rennzie/geodesy-wasm/issues/39)
- Renamed `noop` operator to `senmerc` in anticipation of adding a Sensat Mercator operator
- How Coordinates work [#19](https://github.com/Rennzie/geodesy-wasm/issues/19):
  - Rename `CoordBuffer` to `Coordinates`
  - Remove superfluous wrapping of the underlying buffer
  - Removed the requirement for specifying the coordinate dimension - they are always 4D.

### Removed

- The `unitconvert` and `longlat` operators which have been implemented upstream in [Geodesy:#80](https://github.com/busstoptaktik/geodesy/pull/80)

## [0.5.0] - 2023-11-17

### Added

- Support for NTv2 Grids with multiple subgrids via [Geodesy:#74](https://github.com/busstoptaktik/geodesy/pull/74)
- General clean ups and re-org i.e rename `Ctx` -> `Geo`

## [0.5.0-beta.[2->6]] - 2023-11-16

Iteration to get teh CI publishing working

## [0.5.0-beta.1] - 2023-11-14

### Added

CI Script for publishing to Github npm registry

## [0.4.1] - 2023-11-11

### Fixed

- Gridshift interpolation via RG [#75](https://github.com/busstoptaktik/geodesy/pull/75)

## [0.4.0] - 2023-11-10

### Added

- Support `hgridshift` via `tidyProjString` helper which replaces `hgridshift` with `gridshift` [#23](https://github.com/Rennzie/geodesy-wasm/issues/23)
- Update Rust Geodesy commit to use `ntv2` and multi grid support
  - Now supporting `@optional` and `@null` grids
- `longlat` operator and aliases [#24](https://github.com/Rennzie/geodesy-wasm/issues/24)

## [0.3.0] - 2023-10-05

### Added

- `unitconvert` operator [#21](https://github.com/Rennzie/geodesy-wasm/issues/21)

### Fixed

- The dates in this change log. Blindly following Github Copilot is a mistake

## [0.2.0] - 2023-09-23

- Move test.js to examples folder [#18](https://github.com/Rennzie/geodesy-wasm/issues/18)
  - Added 00-basic examples
  - Added 01-gridshift examples
- Expose `parse_proj` function to wasm.
- Added performance benchmarks vs proj4.js [#11](https://github.com/Rennzie/geodesy-wasm/issues/11)
  - Improved performance to JS Wrapper code

## [0.2.0-beta2] - 2021-09-23

### Added

- ESM Build [#16](https://github.com/Rennzie/geodesy-wasm/issues/16)
- Example usage in ObservableHQ

## [0.2.0-beta1] - 2021-09-23

### Added

- This CHANGELOG file
- A lightweight JS Wrapper with a friendlier API [#3](https://github.com/Rennzie/geodesy-wasm/issues/3)
  - The `Geodesy` class is intended for most users. Advanced users can still use the `GeodesyWasm` components directly.
- Support for NTv2 Grids [#2](https://github.com/Rennzie/geodesy-wasm/issues/2) via Geodesy [branch](https://github.com/busstoptaktik/geodesy/pull/60)
- The ability to supply a gridshift file via `RawGrids` struct [#2](https://github.com/Rennzie/geodesy-wasm/issues/2)
- Update the README with usage examples and better documentation [#9](https://github.com/Rennzie/geodesy-wasm/issues/9)
- Add tests for the wrapper [#10](https://github.com/Rennzie/geodesy-wasm/issues/10)
  - Replaced yarn with [bun](https://bun.sh/docs/cli/test) in the process
