# Quickstart: Brownfield MVP Compiler Slice

## Goal

Use this flow to move the repository from a provisional frontend scaffold to one honest end-to-end minimal slice without widening scope prematurely.

## Prerequisites

- Stable Rust toolchain installed with `rustfmt` and `clippy`
- Local access to the feature integration branch `001-brownfield-spec`
- Repo docs reviewed, especially `.specify/memory/constitution.md`, `docs/LANGUAGE_SPEC.md`, `docs/TASKS.md`, `docs/NEXT_STEPS.md`, and `docs/LVGL_MAPPING.md`

## Working Rules

- Treat `001-brownfield-spec` as the feature integration branch, not the default branch for direct implementation work.
- Create one flat-named phase branch per delivery phase from the current tip of `001-brownfield-spec`, open its PR back into `001-brownfield-spec`, and cut the next phase branch only after that PR merges.
- Start each code-bearing step by adding or updating a failing test, fixture, snapshot, or command-level assertion before implementation.
- Keep helpers and modules single-purpose so parser, semantic, IR, backend, and CLI boundaries stay readable and testable.
- Treat diagnostics and logging as stable operator-facing behavior: logs should describe stage progress and failure paths without contaminating generated output or deterministic error text.
- Update normative docs, example READMEs, and fixture labels in the same change when implementation or support status changes.
- Keep terminology consistent across docs so ratified behavior, deferred work, and aspirational examples are easy to distinguish.
- For major stage decisions, prepare supporting discussion material before locking in implementation: options, pros/cons, relevant practices, implementation developments, risks, and open questions.
- Do not treat stage-shaping decisions as final until the developer reviews the material and explicitly signs off.

- Name phase branches so review scope is obvious, for example `001-brownfield-spec-phase-1-setup` or `001-brownfield-spec-phase-4-us2-validation`.

## Step 0: Create the current phase branch

Before changing files for a phase, sync the integration branch and cut a flat-named phase branch for that phase:

```bash
git checkout 001-brownfield-spec
git pull origin 001-brownfield-spec
git checkout -b 001-brownfield-spec-phase-<n>-<scope>
```

When the phase is ready for review, push the phase branch and open a pull request with base `001-brownfield-spec`. Do not start the next phase branch until that PR lands.

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

Capture any current `doctor` or `validate` command output that is intended to remain user-visible so later tests can distinguish deliberate logging from accidental noise.

## Step 3: Ratify the language contract before broad parser work

Update the docs so the MVP parser no longer has to guess. Complete this before expanding parser implementation.

Before ratifying the slice, prepare or refresh the supporting research notes so the developer can review the tradeoffs rather than inheriting an implicit decision from code.

Required alignments:

- ratify the accepted markup subset in `docs/LANGUAGE_SPEC.md`
- ratify the accepted style subset in `docs/LANGUAGE_SPEC.md`
- state the named event reference rule explicitly
- state the binding rejection policy explicitly
- ensure `README.md`, `docs/TASKS.md`, `docs/NEXT_STEPS.md`, `docs/ARCHITECTURE.md`, and `docs/LVGL_MAPPING.md` tell the same story
- ensure `examples/minimal` only uses ratified constructs
- ensure documentation wording distinguishes current support from future aspirations without ambiguity
- capture the chosen recommendation and rejected alternatives clearly enough for developer sign-off

## Step 4: Implement the parser gate

Focus only on `parser/` plus shared diagnostics support in `compiler/`.

Implementation checklist:

- add or update failing parser tests and invalid fixtures first
- parse markup documents into real `WidgetNode` trees
- parse style documents into real `StyleRule` and `Declaration` records
- attach source spans to syntax diagnostics
- reject malformed syntax and unsupported selector forms clearly
- add unit tests for both successful and failing parser cases
- keep parser helpers narrow and avoid pushing semantic or backend concerns into parser code

Verification commands:

```bash
cargo test -p lumaui-parser
cargo run -p lumaui-cli -- validate examples/minimal
```

Parser gate is complete when `examples/minimal` parses successfully and invalid fixtures fail with clear source-located diagnostics.

## Step 5: Implement semantic validation and IR lowering

Once parsing is stable, expand only `semantic/` and `ir/`.

If semantic or IR contract choices affect downstream assumptions, pause to refresh the supporting decision material and review it with the developer before treating the contract as fixed.

Implementation checklist:

- add or update failing semantic tests first
- reject duplicate ids across the compiled project
- validate supported widgets and supported properties
- validate named event references
- reject bindings and other deferred constructs explicitly
- normalize accepted declarations into canonical semantic values
- lower accepted input into canonical IR
- emit deterministic validation-stage logging only where command behavior benefits from it

Verification commands:

```bash
cargo test -p lumaui-semantic
cargo run -p lumaui-cli -- validate examples/minimal
```

Semantic gate is complete when the minimal example lowers cleanly and invalid constructs produce actionable diagnostics.

## Step 6: Implement the backend gate

Only after semantic IR exists should backend and CLI build work expand.

Backend ownership-boundary policy and emitted-structure conventions should be discussed with the developer before they are treated as stable repository behavior.

Implementation checklist:

- add or update failing backend snapshots and CLI build assertions first
- connect `lumaui build` to the real frontend and semantic pipeline
- generate stable `.c` and `.h` artifacts for the minimal screen
- keep naming, file order, and formatting deterministic
- preserve the documented generated-output ownership model
- add or update snapshots driven by real frontend input
- keep backend/build logging stage-scoped, readable, and excluded from generated artifact contents

Verification commands:

```bash
cargo test -p lumaui-backend-lvgl-c
cargo run -p lumaui-cli -- build examples/minimal
```

Snapshot maintenance rule:

- treat `tests/snapshots/minimal_screen.c` and `tests/snapshots/minimal_screen.h` as exact generated-output contracts
- if backend emitter text changes intentionally, update those snapshots in the same change
- rerun the full verification bundle after any snapshot refresh so backend and frontend checks stay aligned

Backend gate is complete when repeated builds of the same input produce stable generated output.

## Step 7: Lock in fixture and stability expectations

After the backend gate passes:

- keep `examples/minimal` as the trusted normative fixture
- label `examples/dashboard` and any other broader examples as aspirational or expected-fail until supported
- expand regression tests around implemented behavior only
- keep snapshots readable and intentionally ordered
- refactor duplication or unclear ownership that tests exposed while preserving deterministic behavior
- confirm contributor-facing docs still describe the real workflow and verification steps succinctly

## Done Criteria For This Slice

The brownfield MVP slice is complete when:

1. The supported syntax and explicit deferrals are clearly documented.
2. `examples/minimal` validates and builds end to end.
3. Unsupported syntax, widgets, properties, and bindings fail clearly.
4. Generated output is deterministic and reviewable.
5. Docs, examples, tests, snapshots, and operator-visible logging all reflect the same supported surface.
6. Documentation is concise, terminology-consistent, and explicit about what is ratified versus deferred.
7. Major stage decisions taken during the slice were backed by written discussion material and explicit developer sign-off.
8. Each implementation phase landed through its own pull request into `001-brownfield-spec`.