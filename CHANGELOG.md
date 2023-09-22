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

## [Unreleased]

- Move test.js to examples folder [#18](https://github.com/Rennzie/geodesy-wasm/issues/18)
  - Added 00-basic examples
  - Added 01-gridshift examples
- Expose `parse_proj` function to wasm.

### Added

- This CHANGELOG file
- A lightweight JS Wrapper with a friendlier API [#3](https://github.com/Rennzie/geodesy-wasm/issues/3)
  - The `Geodesy` class is intended for most users. Advanced users can still use the `GeodesyWasm` components directly.
- Support for NTv2 Grids [#2](https://github.com/Rennzie/geodesy-wasm/issues/2) via Geodesy [branch](https://github.com/busstoptaktik/geodesy/pull/60)
- The ability to supply a gridshift file via `RawGrids` struct [#2](https://github.com/Rennzie/geodesy-wasm/issues/2)
- Update the README with usage examples and better documentation [#9](https://github.com/Rennzie/geodesy-wasm/issues/9)
- Add tests for the wrapper [#10](https://github.com/Rennzie/geodesy-wasm/issues/10)
  - Replaced yarn with [bun](https://bun.sh/docs/cli/test) in the process

[unreleased]:
