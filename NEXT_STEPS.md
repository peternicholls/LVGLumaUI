# Next Steps

## Purpose

This document is the execution-oriented companion to `TASKS.md`.

- `PRD.md` explains what LumaUI is and why it should exist.
- `ARCHITECTURE.md` explains how the compiler is structured.
- `TASKS.md` defines the full phased roadmap and acceptance criteria.
- `NEXT_STEPS.md` explains what to do next, in order.

## Current Status

Current active phase: `Phase 0: Foundation`

Completed in the current repository state:

- root documentation pack
- Rust workspace layout
- compiler stage crates
- provisional examples
- diagnostics, config, and discovery scaffolding
- provisional lexer and backend scaffolding

Not complete yet:

- ratified language grammar
- real AST construction
- semantic lowering
- end-to-end generated output flow
- preview integration

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
