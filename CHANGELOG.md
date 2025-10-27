# Changelog

All notable changes to this project will be documented in this file.

This project adheres to [Semantic Versioning](https://semver.org).

<!--
Note: In this file, do not use the hard wrap in the middle of a sentence for compatibility with GitHub comment style markdown rendering.
-->

## [Unreleased]

## [0.5.4-beta.1] - 2025-10-27
### Changed
- Items in the queue can be superseded by others without needing to have completed downloading

### Chore
- Updated dependencies

## [0.5.3] - 2025-03-19
### Fixed
- TBA queue detection & depreciated downloads detection. Sonarr had reverted some changes upstream that broke these once again

## [0.5.2] - 2025-03-15
### Fixed
- Removal of queued items that are of lower quality than the file on disk

## [0.5.1] - 2025-02-15
### Fixed
- TBA queue detection

## [0.5.0] - 2024-12-24
### Removed
- All CLI configuration arguments - Use the configuration file or environmental vars in future

### Added
- `--generate-config` CLI command - Generates a configuration file using either the current env vars, or defaults
- Removal of queued items if a higher scored episode is already in the queue

## [0.4.0] - 2024-12-11
### Removed
- `SCRUBARR_SONARR_PORT` & `SCRUBARR_OMIT_PORT` options and CLI arguments

### Deprecated
- All CLI arguments - Use the configuration file or environmental vars in future. Passed CLI arguments (excluding the API key) will 
  be automatically added into a newly generated configuration file on first start up for this minor version

### Added
- Configuration file support - This will default to the XDG specification location for the host OS, or can be overridden with the `X_SCRUBARR_CONFIG`
env var. For Docker builds, this will be `/config/settings.json`
- Multi-instance support - define instances in `settings.json` or `SCRUBARR_SONARR_[int]_[variable]` (ie `SCRUBARR_SONARR_1_URL`)

### Changed
- `SCRUBARR_SONARR_BASE_PATH` (`SCRUBARR_SONARR_1_BASE_PATH`) renamed to `SCRUBARR_SONARR_1_BASE`

## [0.3.1] - 2024-11-30
### Fixed
- Incorrect queue filter being enforced, resulting in the queue not being processed

## [0.3.0] - 2024-11-27
### Deprecated
- `SCRUBARR_SONARR_PORT` & `SCRUBARR_OMIT_PORT` options - Port is now parsed as part of the `SCRUBARR_SONARR_URL` variable

### Added
- `SCRUBARR_VERBOSE` env var & cli argument - Provides verbose http logs at the `trace` level

### Changed
- Minimum interval time is now 300 seconds

### Fixed
- Various Sonarr API schema issues - Missing optionals and incorrect types

## [0.2.1] - 2024-11-24
### Fixed
- `SCRUBARR_SONARR_BASE_PATH` env variable not working

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

[Unreleased]: https://github.com/CPU-Blanc/scrubarr/compare/v0.5.4-beta.1...HEAD
[0.5.4-beta.1]: https://github.com/CPU-Blanc/scrubarr/compare/v0.5.3...v0.5.4-beta.1
[0.5.3]: https://github.com/CPU-Blanc/scrubarr/compare/v0.5.2...v0.5.3
[0.5.2]: https://github.com/CPU-Blanc/scrubarr/compare/v0.5.1...v0.5.2
[0.5.1]: https://github.com/CPU-Blanc/scrubarr/compare/v0.5.0...v0.5.1
[0.5.0]: https://github.com/CPU-Blanc/scrubarr/compare/v0.4.0...v0.5.0
[0.4.0]: https://github.com/CPU-Blanc/scrubarr/compare/v0.3.1...v0.4.0
[0.3.1]: https://github.com/CPU-Blanc/scrubarr/compare/v0.3.0...v0.3.1
[0.3.0]: https://github.com/CPU-Blanc/scrubarr/compare/v0.2.1...v0.3.0
[0.2.1]: https://github.com/CPU-Blanc/scrubarr/compare/v0.2.0...v0.2.1
[0.2.0]: https://github.com/CPU-Blanc/scrubarr/compare/v0.1.2...v0.2.0
[0.1.2]: https://github.com/CPU-Blanc/scrubarr/compare/v0.1.1...v0.1.2
[0.1.1]: https://github.com/CPU-Blanc/scrubarr/compare/v0.1.0...v0.1.1
[0.1.0]: https://github.com/CPU-Blanc/scrubarr/tree/v0.1.0
