# Changelog

All notable changes to this repository should be recorded in this file.

The format is based on Keep a Changelog and the project follows Semantic Versioning as described in `docs/VERSIONING.md`.

## [Unreleased]

## [1.0.0] - 2026-09-09

### Added

- Usable `lumaui` CLI with init, doctor, validate, build and version reporting.
- Completed ratified five-widget compiler pipeline, styles and named callbacks.
- Native release archives, checksums, source-install instructions and firmware examples.
- Cross-platform compiler/package checks and a real LVGL 9.2.2 runtime smoke test.

### Fixed

- Missing generated callback declarations, C++ linkage and snapshot drift.
- Loss of user-owned code during regeneration; malformed regions now block output.
- Acceptance of nested screens, empty projects, invalid symbol prefixes and LVGL versions.
- Coordinate overflow, output-name collisions, hex-shaped id selectors, source-path loss,
  carriage returns in strings, and unbounded parser recursion.


### Added

- Documented repository versioning policy and release housekeeping.
- Added a canonical changelog for future release tracking.

### Changed

- Standardized brownfield phase-branch naming on flat branch names that work cleanly with Git ref rules.

## [0.1.0] - 2026-04-05

### Added

- Established the Rust workspace and compiler-stage crate layout.
- Added repository-wide architecture, roadmap, language, LVGL mapping, and documentation scheme docs.
- Added provisional examples, fixture directories, and initial snapshot structure.
- Added shared diagnostics, configuration, source discovery, IR types, CLI scaffolding, and LVGL C backend scaffolding.

### Notes

- This is a pre-1.0 foundation baseline. The authored language grammar remains provisional and the end-to-end compiler path is not yet complete.