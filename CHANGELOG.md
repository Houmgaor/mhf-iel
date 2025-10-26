# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/).

## [Unreleased]

### Added

- Cross-platform build support for Windows from Linux using cargo-xwin.
- Instructions for Linux-based build.

### Changed

- Switched from nightly to stable Rust toolchain.
- Replaced unstable `Box::new_zeroed()` with stable `Box::new(std::mem::zeroed())`.
- Improved build process documentation.

### Fixed

- Build warnings for stabilized features.

### Removed

- Nightly Rust requirement.
- Feature attributes for `generic_arg_infer` and `new_uninit`, now stable.

## [0.4] - 2023-10-29

### Changed in 0.4

- Rewrote the project in Rust.
- Improved library usability.
- Allow receiving `&str` instead of `String`.
- Derived `Default` on main config struct.

### Added in 0.4

- Support for MHF F5 version.
- JKR environment variable for F5.
- Unique mutex names handling.
- Old DLL handling support.
- LICENSE file.

### Fixed in 0.4

- Mutex handling issues.
- F5 split implementation.

## [0.3] - 2022-10-23

### Added in 0.3

- Extra validation.

## [0.2] - 2022-10-22

### Added in 0.2

- Simple Python GUI for testing.

### Changed in 0.2

- Updated Python GUI.

### Fixed in 0.2

- Correct flag setting on new characters.

## [0.1] - 2022-10-22

### Added in 0.1

- Initial release.
- README documentation.
