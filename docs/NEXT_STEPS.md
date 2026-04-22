# Next Steps

## Purpose

This document is the execution-oriented companion to `TASKS.md`.

- `PRD.md` explains what LumaUI is and why it should exist.
- `ARCHITECTURE.md` explains how the compiler is structured.
- `TASKS.md` defines the full phased roadmap and acceptance criteria.
- `NEXT_STEPS.md` explains what to do next, in order.

## Current Status

Current active phase: `Phase 6: Polish`

Completed in the current repository state:

- root documentation pack
- Rust workspace layout
- compiler stage crates
- ratified first slice (`LS-0.2.0`) — see `docs/LANGUAGE_SPEC.md`
- real lexer, recursive-descent parser, semantic validation + lowering, IR with applied styles, LVGL 9.x backend with style/event emission and ownership-region markers
- end-to-end CLI: `init`, `validate`, `build`, `doctor` all wired through the pipeline; `build` writes deterministic output to disk
- frontend-driven CLI integration tests with normalized exact snapshot comparison, parser/lexer unit tests, semantic validation tests, IR tests, backend snapshot test
- `scripts/lumaui-phase-check.sh --require-build` passes against `examples/minimal`

Not complete yet:

- broader style and widget surface beyond the ratified slice (requires decision briefs + `MINOR` bumps)
- preview / runtime integration
- bindings (explicitly deferred)
- asset pipeline

## Pre-Phase-2 Review Check

- Treat `001-brownfield-spec` as the integration branch for the active brownfield slice, not the branch for direct phase implementation work.
- Create one flat-named phase branch per phase from the current tip of `001-brownfield-spec`, open each phase PR back into `001-brownfield-spec`, and start the next phase branch only after the previous PR merges.
- For any code-bearing step, write or update the failing test, fixture, snapshot, or command assertion first.
- Keep helpers and modules single-purpose so stage isolation remains easy to review.
- When `doctor`, `validate`, or `build` output changes, treat logging and diagnostics as explicit contract behavior and verify them deliberately.
- When generated LVGL output changes intentionally, update the committed snapshots in the same change and rerun the full verification bundle so backend and frontend checks stay aligned.
- Update the relevant docs, example labels, and operator guidance in the same change when support status or behavior changes.
- Keep wording concise and consistent so ratified support, deferred work, and aspirational examples remain easy to audit.
- Before major stage choices are treated as settled, produce discussion-ready supporting material with options, pros/cons, relevant practices, implementation developments, risks, and open questions.
- Defer the final decision on those choices until the developer explicitly signs off.

## Execution Order

Work should proceed in this order. Do not skip ahead unless a dependency is already satisfied.

### Step 1: Ratify the MVP Language Surface

Goal:
Define the smallest possible authored language that can support one real end-to-end example.

Decisions required:

- exact MVP markup grammar
- exact MVP style grammar
- supported widgets for the first compiler slice
- supported property list for the first compiler slice
- event reference syntax
- binding reference policy for MVP

Recommended scope for the first real slice:

- `Screen`
- `Column`
- `Row`
- `Text`
- `Button`
- class selectors
- id selectors
- text literals
- width and height
- padding
- background color
- text color

Explicitly defer if they slow down parser completion:

- `Grid`
- `Image`
- `Card`
- font assets
- bindings beyond symbolic references
- margin shorthand complexity

Exit criteria:

- `LANGUAGE_SPEC.md` is updated from provisional to MVP-ratified for the implemented subset
- there is no ambiguity about what the parser must accept or reject

### Step 2: Implement the First Real Parser

Goal:
Replace the provisional parser path with a real parser for the ratified subset.

Required work:

- parse markup source into a real AST
- parse style source into a real AST
- report clear syntax diagnostics with spans
- add failure-case tests for malformed input

Recommended target fixture:

- `examples/minimal`

Exit criteria:

- `lumaui validate examples/minimal` parses real syntax
- parser tests cover both valid and invalid cases

### Step 3: Implement Semantic Validation and Lowering

Goal:
Turn parsed syntax into a backend-ready typed model.

Required work:

- validate duplicate ids
- validate supported widgets
- validate supported properties
- normalize declarations into explicit semantic values
- represent event and binding references explicitly
- lower into `ir/`

Exit criteria:

- semantic validation rejects unsupported constructs clearly
- the semantic layer emits a canonical IR for the minimal example

### Step 4: Complete One End-to-End Backend Slice

Goal:
Generate real LVGL C from the semantic IR for the minimal example.

Required work:

- connect CLI `build` to the real pipeline
- emit stable `.c` and `.h` files
- keep symbol naming deterministic
- keep code readable enough for snapshot review

Exit criteria:

- the minimal example builds through the full compiler path
- backend snapshots reflect real frontend input rather than synthetic IR only

### Step 5: Expand the Supported Surface Carefully

Goal:
Add the rest of the MVP in controlled increments.

Recommended expansion order:

1. `Container`
2. `Card`
3. `Image`
4. `Grid`
5. binding references
6. richer style properties

Exit criteria:

- every expansion updates docs, fixtures, and tests together
- LVGL mapping remains explicit and conservative

## Rules for Choosing the Next Task

When multiple tasks are possible, prefer the one that:

1. reduces ambiguity in the language or architecture
2. enables an end-to-end path
3. improves diagnostics or determinism
4. strengthens fixtures and tests

Avoid work that:

- broadens the surface before the current slice is complete
- introduces browser semantics
- adds preview/editor concerns before the compiler path is stable
- adds asset pipeline complexity before the core source pipeline works

## Recommended Immediate Task List

The next concrete implementation sequence should be:

1. update `LANGUAGE_SPEC.md` with a ratified MVP subset, not the whole aspirational surface
2. add parser tests for `examples/minimal`
3. implement real parsing for that subset
4. implement semantic normalization for the same subset
5. wire `build` to produce one real generated screen
6. update backend snapshots to reflect real frontend input

## Definition of Done for the Next Iteration

The next iteration should be considered successful when:

- one minimal example goes from source files to generated C
- the supported syntax is explicitly documented
- unsupported syntax fails clearly
- the implementation still feels narrow and controlled
