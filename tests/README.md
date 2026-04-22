# Tests

This directory stores shared fixtures and snapshots used by crate-local tests.

- `fixtures/` contains small authored-source examples and other test inputs.
- `snapshots/` contains expected generated C output for deterministic backend tests.

## Fixture Categories

Every fixture in `tests/fixtures/` (and every fixture under `examples/`) MUST be classified into exactly one of the categories below. Classification governs which fixtures may participate in snapshot regression and how their CLI behavior is asserted.

### `normative`

A fixture that the currently ratified language slice fully supports.

- Filename or README label MUST identify it as normative.
- `lumaui validate` MUST succeed.
- `lumaui build` MUST succeed and emit deterministic output.
- Generated artifacts MAY participate in snapshot regression (`tests/snapshots/`).
- CLI logging output for these fixtures is part of the contract.

### `expected-fail`

A fixture that intentionally exceeds the ratified slice and MUST be rejected.

- Filename or README label MUST identify it as expected-fail.
- `lumaui validate` MUST report a deterministic, source-located diagnostic for every offending construct.
- `lumaui build` MUST NOT produce output for these fixtures.
- Diagnostics ARE part of the contract; their text and ordering are asserted.
- Expected-fail fixtures MUST NOT participate in backend snapshot regression.

### `aspirational`

A fixture that documents future direction but stays ahead of the ratified slice.

- README MUST mark it as aspirational; the fixture itself MUST be excluded from CLI smoke runs.
- Aspirational fixtures MUST NOT participate in snapshot regression.
- Aspirational fixtures MUST NOT be referenced by the standard verification bundle (`scripts/lumaui-phase-check.sh`).

## Naming Convention

Fixture filenames follow `<topic>_<category-hint>.<ext>` where the category hint is implied by the filename:

- `minimal_*.lui` / `minimal_*.lus` — normative
- `unsupported_*.{lui,lus}`, `binding_reference.lui`, and `duplicate_ids.lui` — expected-fail
- Anything under `examples/dashboard/` — aspirational (until the slice expands)

If a fixture's category changes, its filename, README label, and test wiring MUST all change in the same edit.

## Snapshot Participation

`tests/snapshots/` contains expected generated C for normative fixtures only. Snapshot files are checked in verbatim and reviewed on every backend change. Expected-fail and aspirational fixtures MUST NOT have snapshot files.

Generated-output snapshots are contract artifacts, not loose examples. If `backend/lvgl_c/src/lib.rs` changes emitted text intentionally, update `tests/snapshots/` in the same change and rerun the full verification bundle.

## Logging Capture

Operator-visible CLI logging (`doctor`, `validate`, `build`) is captured per category:

- normative — full success log is asserted
- expected-fail — failure diagnostics and exit status are asserted
- aspirational — no CLI assertion; the fixture is documentation only
