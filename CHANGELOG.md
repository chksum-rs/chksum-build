# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Changed

- Disabled code coverage for internal `info` module.

## [0.0.1] - 2022-12-14

### Added

- Added tests with MSRV toolchain in Rust workflow.

### Changed

- Replaced branch checks state with workflow status for branch.

### Fixed

- Fixed coverage job in Rust workflow.
- Fixed lifetimes for constant `str`s.
- Fixed MSRV to `1.58.0`.
- Fixed Cargo build script commands (misspelled `rustup` instead of `rustc`).

### Removed

- Removed `strip` option for release profile.
- Removed tests with `beta` toolchain in Rust workflow.

## [0.0.0] - 2022-11-27

### Added

- Initial release.

[Unreleased]: https://github.com/ferric-bytes/chksum-build/compare/v0.0.1...HEAD
[0.0.1]: https://github.com/ferric-bytes/chksum-build/compare/v0.0.0...v0.0.1
[0.0.0]: https://github.com/ferric-bytes/chksum-build/releases/tag/v0.0.0
