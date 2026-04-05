# Versioning Policy

## Purpose

This document defines how LumaUI versions the Rust workspace, tags releases, and records change history.

It does not define the authored-language contract version. `docs/LANGUAGE_SPEC.md` has its own version line and change history so language ratification can evolve independently from repository releases.

The goals are:

- keep all crates on one coherent repository version
- make release intent visible before tagging
- distinguish routine fixes from breaking contract changes
- keep changelog entries reviewable and useful to operators and contributors

## Current Model

LumaUI uses a single workspace version defined in the root `Cargo.toml` under `[workspace.package]`.

All published workspace crates inherit that version through `version.workspace = true`.

That means:

- `cli/`, `compiler/`, `parser/`, `semantic/`, `ir/`, and `backend/lvgl_c/` move together
- the repository has one version number for release and changelog purposes
- feature branches do not create separate crate versions

## Separate Language Spec Versioning

The authored-language contract is versioned separately from the workspace.

- Workspace and crate releases are tracked by the root `Cargo.toml` version and `CHANGELOG.md`.
- The language contract is tracked by the `Language Spec Version` declared in `docs/LANGUAGE_SPEC.md`.
- Language-specification changes are recorded in `docs/LANGUAGE_CHANGELOG.md`.

This separation is intentional because the repository can refine compiler implementation, docs, tooling, or release packaging without necessarily changing the language contract, and language-contract work may require explicit revision tracking before a corresponding application release is cut.

## Scheme

LumaUI uses Semantic Versioning with a pre-1.0 discipline.

Format:

- `MAJOR.MINOR.PATCH`

While the project remains below `1.0.0`:

- `0.MINOR.0` is used for meaningful slice expansion, contract reshaping, or intentional breaking changes
- `0.MINOR.PATCH` is used for compatible fixes, documentation corrections, test updates, and non-breaking implementation improvements inside the current slice

Examples:

- `0.1.0` initial public repository baseline
- `0.2.0` ratified first MVP language slice with breaking parser or CLI contract changes permitted
- `0.2.1` compatible bug fix or documentation-only follow-up on the `0.2.x` line

## What Counts As Breaking Before 1.0

Before `1.0.0`, treat the following as minor-version changes rather than patch releases:

- authored-language grammar changes
- CLI command contract changes
- semantic or IR contract changes that affect downstream expectations
- generated C structure or ownership-boundary policy changes
- snapshot-visible output changes that are intentional rather than bug fixes

Patch releases should avoid intentional breaking changes.

## Tags

Release tags should use the form:

- `v0.1.0`
- `v0.2.0`

Do not tag phase branches independently. Phase branches are delivery branches, not release lines.

## Changelog Rules

The canonical changelog lives at the repository root in `CHANGELOG.md`.

The language specification has its own changelog in `docs/LANGUAGE_CHANGELOG.md`.

Use these sections when needed:

- `Added`
- `Changed`
- `Fixed`
- `Deprecated`
- `Removed`
- `Security`

Guidelines:

- keep entries user- and maintainer-visible rather than file-by-file
- group related changes into one entry when they ship together
- record contract changes, not every refactor detail
- update the changelog in the same change that updates the version for a release
- update `docs/LANGUAGE_CHANGELOG.md` when the authored-language contract changes independently of a workspace release

## Release Housekeeping

When cutting a release:

1. Update the root workspace version in `Cargo.toml`.
2. Confirm workspace crates still inherit `version.workspace = true`.
3. Move relevant notes from `Unreleased` in `CHANGELOG.md` into a versioned section.
4. Commit the version and changelog update together.
5. Tag the release as `vX.Y.Z` from the release commit.

If the release also ratifies or revises the authored-language contract, update the `Language Spec Version` in `docs/LANGUAGE_SPEC.md` and move the relevant notes from `docs/LANGUAGE_CHANGELOG.md` in the same change.

## Branch Workflow Interaction

The active brownfield workflow uses `001-brownfield-spec` as an integration branch and flat-named phase branches for implementation.

Versioning and changelog rules apply when changes are prepared for release, not every time a phase branch merges. Phase branches should keep changelog notes current when they materially affect release-facing behavior, but the version number itself should only change when the repository is intentionally being cut toward a release.