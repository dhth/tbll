# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added

- Allow skipping specific columns from input

## [v1.1.0] - Mar 12, 2025

### Added

- Allow selecting specific columns from input

## [v1.0.0] - Mar 07, 2025

### Fixed

- Correctly parsing row items when they have the delimiter character in them

### Removed

- `--row` flag: supplying row items via the flag is no longer supported
- `--read-stdin` flag: reading from stdin is now the default behaviour
- `--num-cols` flag: this is no longer needed/supported

[unreleased]: https://github.com/dhth/tbll/compare/v1.1.0...HEAD
[v1.1.0]: https://github.com/dhth/tbll/compare/v1.0.0...v1.1.0
[v1.0.0]: https://github.com/dhth/tbll/compare/v0.2.3...v1.0.0
