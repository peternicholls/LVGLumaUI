# Changelog

All notable changes to this repository should be recorded in this file.

The format is based on Keep a Changelog and the project follows Semantic Versioning with prerelease tagging as described in `docs/VERSIONING.md`.

## [Unreleased]

## [1.0.0-beta.1] - 2026-04-22

### Added

- Shipped the first end-to-end LumaUI compiler pipeline across parser, semantic analysis, IR lowering, deterministic LVGL C backend emission, and CLI orchestration.
- Added normative and expected-fail fixtures, canonical backend snapshots, frontend exact-output regression coverage, and the standard `scripts/lumaui-phase-check.sh` verification bundle.
- Added repository workflow guardrails, release-facing verification guidance, and decision material covering language ratification and generated-output drift.

### Changed

- Ratified the first authored-language slice as `LS-0.2.0`, including the MVP widget set, selector surface, style properties, deterministic style application rules, and explicit binding deferral.
- Promoted `examples/minimal` to the normative end-to-end fixture, marked `examples/dashboard` aspirational, and aligned docs, fixtures, and examples around that support boundary.
- Wired `lumaui validate`, `lumaui build`, `lumaui doctor`, and starter project generation through the real compiler pipeline with deterministic, stage-scoped operator output.
- Moved the workspace release line from the initial `0.1.0` foundation baseline to the `1.0.0-beta.x` prerelease track.

### Fixed

- Closed backend/frontend generated-output drift by tightening snapshot maintenance rules, restoring the canonical minimal screen snapshot, and aligning LVGL button emission with the LVGL 9 `lv_button_create` API.

### Notes

- This is the first beta release for the repository's coherent end-to-end compiler slice. Broader widgets, richer style properties, bindings, assets, and preview integration remain intentionally deferred.

## [0.1.0] - 2026-04-05

### Added

- Established the Rust workspace and compiler-stage crate layout.
- Added repository-wide architecture, roadmap, language, LVGL mapping, and documentation scheme docs.
- Added provisional examples, fixture directories, and initial snapshot structure.
- Added shared diagnostics, configuration, source discovery, IR types, CLI scaffolding, and LVGL C backend scaffolding.

### Notes

- This is a pre-1.0 foundation baseline. The authored language grammar remains provisional and the end-to-end compiler path is not yet complete.