# Language Specification Changelog

All notable ratified or draft-tracked changes to the authored-language specification should be recorded in this file.

This changelog is separate from the repository release changelog in `CHANGELOG.md` because the authored-language contract has its own version line and may change on a different cadence from workspace or CLI releases.

## [Unreleased]

## [LS-0.2.0] - 2026-04-22

### Added

- Ratified the first MVP slice: markup grammar (`Screen`, `Column`, `Row`, `Text`, `Button`), `id` / `class` / `text` / `onPress` attributes, and the style grammar covering class- and id-selectors with the `padding`, `background-color`, `text-color`, `width`, and `height` properties.
- Ratified the deterministic style application rule (last-rule-wins, no specificity cascade) and the named event-reference rule (`onPress` only).

### Changed

- Promoted `docs/LANGUAGE_SPEC.md` from provisional baseline to first-slice ratified contract.
- Documented the explicit rejection of bindings (`bind="…"`) for the first slice.

## [LS-0.1.0] - 2026-04-05

### Added

- Baseline provisional language specification for the pre-ratification phase.
- Phase policy, required capabilities, deferred grammar areas, and stage-ownership constraints for future ratification work.

### Notes

- `LS-0.1.0` is a provisional language-contract baseline, not a claim that the authored grammar is ratified or feature-complete.