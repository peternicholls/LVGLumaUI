# Quickstart: Brownfield MVP Compiler Slice

## Goal

Use this flow to move the repository from a provisional frontend scaffold to one honest end-to-end minimal slice without widening scope prematurely.

## Prerequisites

- Stable Rust toolchain installed with `rustfmt` and `clippy`
- Working tree checked out on branch `001-brownfield-spec`
- Repo docs reviewed, especially `docs/CONSTITUTION.md`, `docs/LANGUAGE_SPEC.md`, `docs/TASKS.md`, `docs/NEXT_STEPS.md`, and `docs/LVGL_MAPPING.md`

## Step 1: Confirm the active phase and target slice

The active repo state is Phase 0 with a planned transition into Phase 1. The target slice for this iteration is:

- one `Screen`
- `Column` and `Row` layouts
- `Text` and `Button`
- ids and classes
- class and id selectors
- `padding`, `background-color`, `text-color`, `width`, and `height`
- named event handler references
- no bindings

## Step 2: Verify the current baseline

Run the current CLI against the normative example to confirm the starting point:

```bash
cargo run -p lumaui-cli -- doctor examples/minimal
cargo run -p lumaui-cli -- validate examples/minimal
cargo test
```

Expected current-state behavior:

- `doctor` reports discovered screen and style files.
- `validate` tokenizes the provisional sources and reports current diagnostics.
- `cargo test` passes the existing crate-local tests.

## Step 3: Ratify the language contract before broad parser work

Update the docs so the MVP parser no longer has to guess. Complete this before expanding parser implementation.

Required alignments:

- ratify the accepted markup subset in `docs/LANGUAGE_SPEC.md`
- ratify the accepted style subset in `docs/LANGUAGE_SPEC.md`
- state the named event reference rule explicitly
- state the binding rejection policy explicitly
- ensure `README.md`, `docs/TASKS.md`, `docs/NEXT_STEPS.md`, `docs/ARCHITECTURE.md`, and `docs/LVGL_MAPPING.md` tell the same story
- ensure `examples/minimal` only uses ratified constructs

## Step 4: Implement the parser gate

Focus only on `parser/` plus shared diagnostics support in `compiler/`.

Implementation checklist:

- parse markup documents into real `WidgetNode` trees
- parse style documents into real `StyleRule` and `Declaration` records
- attach source spans to syntax diagnostics
- reject malformed syntax and unsupported selector forms clearly
- add unit tests for both successful and failing parser cases

Verification commands:

```bash
cargo test -p lumaui-parser
cargo run -p lumaui-cli -- validate examples/minimal
```

Parser gate is complete when `examples/minimal` parses successfully and invalid fixtures fail with clear source-located diagnostics.

## Step 5: Implement semantic validation and IR lowering

Once parsing is stable, expand only `semantic/` and `ir/`.

Implementation checklist:

- reject duplicate ids across the compiled project
- validate supported widgets and supported properties
- validate named event references
- reject bindings and other deferred constructs explicitly
- normalize accepted declarations into canonical semantic values
- lower accepted input into canonical IR

Verification commands:

```bash
cargo test -p lumaui-semantic
cargo run -p lumaui-cli -- validate examples/minimal
```

Semantic gate is complete when the minimal example lowers cleanly and invalid constructs produce actionable diagnostics.

## Step 6: Implement the backend gate

Only after semantic IR exists should backend and CLI build work expand.

Implementation checklist:

- connect `lumaui build` to the real frontend and semantic pipeline
- generate stable `.c` and `.h` artifacts for the minimal screen
- keep naming, file order, and formatting deterministic
- preserve the documented generated-output ownership model
- add or update snapshots driven by real frontend input

Verification commands:

```bash
cargo test -p lumaui-backend-lvgl-c
cargo run -p lumaui-cli -- build examples/minimal
```

Backend gate is complete when repeated builds of the same input produce stable generated output.

## Step 7: Lock in fixture and stability expectations

After the backend gate passes:

- keep `examples/minimal` as the trusted normative fixture
- label `examples/dashboard` and any other broader examples as aspirational or expected-fail until supported
- expand regression tests around implemented behavior only
- keep snapshots readable and intentionally ordered

## Done Criteria For This Slice

The brownfield MVP slice is complete when:

1. The supported syntax and explicit deferrals are clearly documented.
2. `examples/minimal` validates and builds end to end.
3. Unsupported syntax, widgets, properties, and bindings fail clearly.
4. Generated output is deterministic and reviewable.
5. Docs, examples, tests, and snapshots all reflect the same supported surface.