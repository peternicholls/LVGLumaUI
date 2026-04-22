# D-001 Generated Output Verification Drift

**Status**: proposed  
**Owner**: GitHub Copilot (draft for developer review)  
**Date**: 2026-04-22  
**Scope**: `001-brownfield-spec` generated-output verification, backend snapshots, CLI build verification

## Implementation Status

Implemented after this brief was drafted:

1. `cli/tests/end_to_end.rs` now compares frontend-generated `.c` and `.h` output against the canonical backend snapshots after prefix normalization.
2. `backend/lvgl_c/tests/generation.rs` now reports more actionable snapshot-drift failures.
3. `tests/README.md` and `specs/001-brownfield-spec/quickstart.md` now document snapshot-maintenance workflow explicitly.

Still open:

1. decide whether snapshot regeneration should get a dedicated helper command
2. decide whether emitter formatting helpers are justified for long-term churn reduction

## Decision Summary

The repository hit a generated-output verification drift between the LVGL C emitter, the committed backend snapshot, and the weaker frontend build assertions.

The immediate defect was fixed by updating the committed snapshot to match the current deterministic emitter output. That repair was necessary but not sufficient, so the frontend verification and snapshot-maintenance workflow were also tightened afterward.

The recommended follow-up is:

1. keep the backend snapshot as the canonical exact-text contract
2. strengthen the CLI end-to-end build test to compare normalized generated output exactly, not by substring presence
3. document snapshot refresh as an explicit maintenance workflow
4. centralize emitter formatting helpers if output churn continues

## Problem Statement

The first failing check was the backend snapshot assertion in `backend/lvgl_c/tests/generation.rs`.

The generator in `backend/lvgl_c/src/lib.rs` emitted deterministic output, but the committed golden file in `tests/snapshots/minimal_screen.c` no longer matched the exact text being generated.

This was not a semantic compiler bug. It was a contract drift in the generated text surface.

## Evidence

### Backend Emitter

`backend/lvgl_c/src/lib.rs` currently emits source lines with explicit string formatting through `SourceEmitter::line(...)`.

Relevant characteristics:

- four-space indentation is embedded directly in emitted lines such as `"    lv_obj_t *{root_var} = ..."`
- long calls such as `lv_obj_add_event_cb(...)` are emitted as single lines
- emitted function names are derived from `project.symbol_prefix`

### Canonical Backend Snapshot

`backend/lvgl_c/tests/generation.rs` treats `tests/snapshots/minimal_screen.c` and `tests/snapshots/minimal_screen.h` as the exact generated-output contract for a synthetic IR project using symbol prefix `lumaui_`.

That test is strict and byte-for-byte and now reports actionable remediation when drift is intentional.

### Frontend Build Verification

`cli/tests/end_to_end.rs` exercises the full parse -> semantic -> backend path by building `examples/minimal` and now compares the generated `.c` and `.h` contents exactly against the canonical backend snapshots after intentional symbol-prefix normalization.

### Example Output Prefix Difference

The backend snapshot test uses `Project::new("minimal", "lumaui_")` in `backend/lvgl_c/tests/generation.rs`.

The real example project uses `symbol_prefix = "minimal_"` in `examples/minimal/lumaui.toml`.

That means the canonical backend snapshot and the real frontend-generated example are intentionally not byte-identical even when both are correct.

## What Drifted

Two related drifts were present.

### 1. Snapshot Formatting Drift

The committed `tests/snapshots/minimal_screen.c` had drifted from the emitter output in small but exact-text ways:

- indentation width differed
- `lv_obj_add_event_cb(...)` wrapping differed

Because the backend test is exact, that alone was enough to fail `cargo test`.

### 2. Verification-Strength Drift

The repository had one strong exact-text check and one weak smoke-level check:

- backend snapshot test: exact
- CLI end-to-end build test: substring-only

That means frontend-to-backend output drift can survive as long as the generated file still contains a few expected strings.

## Immediate Fix Applied

The committed snapshot in `tests/snapshots/minimal_screen.c` was updated to match the current deterministic emitter output from `backend/lvgl_c/src/lib.rs`.

After that change:

- `cargo test` passed
- `./scripts/lumaui-phase-check.sh --require-build` passed
- the normative example built successfully under `examples/minimal/generated/ui/`

This was the correct immediate fix because the generator behavior was stable and deterministic; the stale artifact was the snapshot.

## Options Considered

### Option A: Update Snapshot Only

Keep the current test structure and refresh snapshots whenever emitter text changes.

Pros:

- smallest immediate change
- preserves current backend snapshot contract
- no new helper logic required

Cons:

- frontend path remains under-verified
- future output drift can still escape exact comparison in CLI tests
- snapshot maintenance remains reactive

### Option B: Make CLI End-to-End Build Checks Exact After Prefix Normalization

Keep the backend snapshot canonical, but normalize the real example output before comparing it to the committed snapshot.

Pros:

- catches exact frontend-to-backend drift
- keeps one canonical backend golden file
- preserves project-specific symbol-prefix behavior in the example

Cons:

- requires a small normalization helper in the CLI integration test
- slightly raises maintenance cost for generated-output tests

### Option C: Change Backend Snapshot Prefix to Match the Example Prefix

Use `minimal_` in the backend synthetic IR snapshot test so the snapshot and the example output can match directly.

Pros:

- removes one normalization step
- makes the backend snapshot look closer to the real example output

Cons:

- couples backend-local verification to example-project naming
- makes the backend contract less backend-local and more fixture-config dependent

### Option D: Add Emitter Formatting Helpers

Refactor `backend/lvgl_c/src/lib.rs` so indentation and wrapping policy are centralized instead of embedded across many string literals.

Pros:

- reduces accidental formatting churn
- makes output policy clearer
- lowers future snapshot drift risk

Cons:

- larger code change than the current defect requires
- not necessary to restore correctness immediately

## Recommended Decision

Adopt Option B now, and keep Option D as follow-up hardening if emitter churn continues.

Specifically:

1. retain the backend snapshot in `tests/snapshots/` as the exact generated-output contract
2. strengthen `cli/tests/end_to_end.rs` so the frontend-generated `.c` and `.h` are compared exactly against normalized golden artifacts
3. document snapshot maintenance explicitly in `tests/README.md` and `specs/001-brownfield-spec/quickstart.md`
4. only refactor emitter formatting helpers if output-format churn becomes recurring noise

## Specific Fixes

### Fix 1: Strengthen CLI End-to-End Generated Output Checks

Status: implemented

File: `cli/tests/end_to_end.rs`

Replace the substring-only assertions in `build_minimal_example_matches_snapshot_for_home_screen` with exact comparisons after prefix normalization.

Implementation detail:

- read `tests/snapshots/minimal_screen.c`
- read `tests/snapshots/minimal_screen.h`
- read `examples/minimal/generated/ui/screens/home_gen.c`
- read `examples/minimal/generated/ui/screens/home_gen.h`
- normalize `minimal_` to `lumaui_` in the example-generated output before comparison
- assert equality for both files

Recommended helper shape:

```rust
fn normalize_generated_prefix(input: &str) -> String {
    input.replace("minimal_", "lumaui_")
}
```

Then compare:

```rust
assert_eq!(normalize_generated_prefix(&generated_source), expected_source);
assert_eq!(normalize_generated_prefix(&generated_header), expected_header);
```

### Fix 2: Cover the Header Artifact in the CLI Test

Status: implemented

File: `cli/tests/end_to_end.rs`

The current frontend build test only sanity-checks the generated `.c` file. Add exact comparison for the generated `.h` as well.

Reason:

- the backend contract includes both `.c` and `.h`
- header drift should be caught in the full pipeline too

### Fix 3: Improve Snapshot Maintenance Documentation

Status: implemented

Files:

- `tests/README.md`
- `specs/001-brownfield-spec/quickstart.md`

Add explicit guidance that:

- backend snapshots are normative exact-text contracts for generated output
- intentional emitter text changes must update committed snapshots in the same change
- the standard verification bundle must be rerun after any snapshot refresh

Suggested wording for `tests/README.md`:

> Generated-output snapshots are contract artifacts, not loose examples. If `backend/lvgl_c/src/lib.rs` changes emitted text intentionally, update `tests/snapshots/` in the same change and rerun the full verification bundle.

### Fix 4: Improve Backend Snapshot Failure Messaging

Status: implemented

File: `backend/lvgl_c/tests/generation.rs`

Make the assertion message more actionable.

Current:

- `"source drifted"`

Recommended:

- `"backend source snapshot drifted; update tests/snapshots/minimal_screen.c if the emitter change is intentional"`

This does not change behavior, but it shortens diagnosis time.

### Fix 5: Centralize Formatting Policy If Churn Continues

Status: deferred unless emitter-format churn becomes recurring maintenance noise

File: `backend/lvgl_c/src/lib.rs`

If generated-text churn continues, refactor `SourceEmitter` to own formatting policy through helpers such as:

- `indented_line(level, text)`
- `emit_call(name, args)`
- `emit_wrapped_call(name, args, max_width)`

This is not required to fix the current defect. It is a maintenance hardening step.

## Relevant Practices And Constraints

- the repository treats generated output as a reviewable product surface
- snapshots are already part of the declared contract in `tests/README.md`
- deterministic output matters more than clever formatting logic
- frontend and backend verification should both protect the same output contract where practical

## Implementation Developments Or Evidence

Verified after the snapshot update and follow-up hardening:

- `cargo test` passes
- `./scripts/lumaui-phase-check.sh --require-build` passes
- `lumaui build examples/minimal` writes deterministic output under `examples/minimal/generated/ui/`
- backend snapshots and frontend-generated `examples/minimal` output now stay aligned through exact normalized comparison in the CLI integration test

Observed generated symbols:

- backend snapshot path: `lumaui_screen_home_create`, `lumaui_event_open_settings`
- real example path: `minimal_screen_home_create`, `minimal_event_open_settings`

## Risks And Tradeoffs

### Risk If Only The Snapshot Is Updated

The immediate failure is gone, but frontend exactness remains under-tested.

### Risk If CLI Tests Become Exact Without Normalization

The tests would fail for expected project-level prefix differences that are intentional, not defects.

### Risk If Prefixes Are Unified Everywhere

Backend-local verification becomes more tightly coupled to example config.

### Tradeoff Chosen

Normalize prefix differences in the CLI integration test and keep the backend snapshot canonical.

## Open Questions

1. Should the canonical backend snapshot always use `lumaui_`, or should the repository standardize on the normative example prefix `minimal_`?
2. Should snapshot regeneration get a dedicated helper command or stay as a documented manual workflow?
3. Is current emitter formatting stable enough to leave inline, or is a formatting-helper refactor already justified?

## Developer Sign-Off

Decision: __________________________________________

Approved by: ______________________________________

Date: ______________________________________________

Notes: _____________________________________________