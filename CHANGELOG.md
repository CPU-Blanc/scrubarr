# Changelog

All notable changes to this project will be documented in this file.

This project adheres to [Semantic Versioning](https://semver.org).

<!--
Note: In this file, do not use the hard wrap in the middle of a sentence for compatibility with GitHub comment style markdown rendering.
-->

## [Unreleased]

## [0.2.0] - 2024-11-23
### Breaking
- `SCRUBARR_SONARR_BASE_URL` environmental variable renamed to `SCRUBARR_SONARR_BASE_PATH`

### Added
- `.env` file support
- Commandline argument support

### Security
- Marked API key header as sensitive

## [0.1.2] - 2024-11-22
### Fixed
- Bulk delete actions failing

## [0.1.1] - 2024-11-21
### Fixed
- Multiple queued items for the same series causing duplicate series refreshes

## [0.1.0] - 2024-11-20

Initial release

[Unreleased]: https://github.com/CPU-Blanc/scrubarr/compare/v0.2.0...HEAD
[0.2.0]: https://github.com/CPU-Blanc/scrubarr/compare/v0.1.2...v0.2.0
[0.1.2]: https://github.com/CPU-Blanc/scrubarr/compare/v0.1.1...v0.1.2
[0.1.1]: https://github.com/CPU-Blanc/scrubarr/compare/v0.1.0...v0.1.1
[0.1.0]: https://github.com/CPU-Blanc/scrubarr/tree/v0.1.0