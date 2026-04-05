# Tasks: Brownfield MVP Compiler Slice

**Input**: Design documents from `/specs/001-brownfield-spec/`
**Prerequisites**: `plan.md`, `spec.md`, `research.md`, `data-model.md`, `quickstart.md`, `contracts/cli-commands.md`

**Tests**: Tests are required for parser behavior, semantic validation, IR shape, backend output, CLI validation/build behavior, fixture classification, and generated snapshots. User Story 1 is primarily a docs and fixture ratification slice, but any starter-project change in `cli/src/main.rs` must carry explicit CLI regression coverage.

**Organization**: Tasks are grouped by user story so each slice can be delivered and verified in constitution-compliant phase order.

## Phase 1: Setup (Shared Infrastructure)

**Purpose**: Establish shared fixtures and repository conventions used by later validation and backend work.

- [ ] T001 Create shared invalid markup fixtures in `tests/fixtures/unsupported_widget.lui`, `tests/fixtures/binding_reference.lui`, and `tests/fixtures/duplicate_ids.lui`
- [ ] T002 [P] Create shared invalid style fixtures in `tests/fixtures/unsupported_selector.lus` and `tests/fixtures/unsupported_property.lus`
- [ ] T003 [P] Update fixture usage guidance in `tests/README.md` for normative, expected-fail, and aspirational authored-source coverage
- [ ] T004 Update example status guidance in `examples/minimal/README.md` and `examples/dashboard/README.md` so later tasks can classify fixtures consistently

---

## Phase 2: User Story 1 - Ratify the First Supported Language Contract (Priority: P1) 🎯 MVP

**Goal**: Freeze the exact first-slice authored-language contract and fixture classification before parser, IR, or backend contracts move.

**Independent Test**: Review `docs/LANGUAGE_SPEC.md`, `README.md`, `docs/TASKS.md`, `docs/NEXT_STEPS.md`, `docs/ARCHITECTURE.md`, `docs/LVGL_MAPPING.md`, `examples/minimal`, and `examples/dashboard` together, then run the CLI starter regression to confirm starter output matches the ratified first slice.

### Tests for User Story 1 ⚠️

- [ ] T005 [P] [US1] Add CLI starter-template regression coverage for the ratified first slice in `cli/src/main.rs`

### Implementation for User Story 1

- [ ] T006 [US1] Ratify the supported markup grammar, style grammar, widget set, selector surface, and event/binding policy in `docs/LANGUAGE_SPEC.md`
- [ ] T007 [P] [US1] Align active phase, gate order, and immediate-work language in `docs/TASKS.md` and `docs/NEXT_STEPS.md`
- [ ] T008 [P] [US1] Align stage boundaries and narrow-slice guidance in `docs/ARCHITECTURE.md` and `docs/LVGL_MAPPING.md`
- [ ] T009 [P] [US1] Align project overview, roadmap wording, and current-status messaging in `README.md`
- [ ] T010 [US1] Update the normative fixture in `examples/minimal/ui/screens/home.lui` and `examples/minimal/ui/styles/theme.lus` so every construct is explicitly ratified
- [ ] T011 [P] [US1] Mark aspirational fixture status and out-of-scope constructs in `examples/dashboard/README.md`, `examples/dashboard/ui/screens/dashboard.lui`, and `examples/dashboard/ui/styles/theme.lus`
- [ ] T012 [US1] Align starter project output with the ratified first slice in `cli/src/main.rs`
- [ ] T013 [US1] Reconcile all slice-level docs and fixture labels across `docs/LANGUAGE_SPEC.md`, `README.md`, `examples/minimal/README.md`, and `examples/dashboard/README.md`

**Checkpoint**: The repo has one explicit, reviewable first-slice contract, one clearly labeled normative fixture, one clearly labeled aspirational fixture, and starter output that matches the ratified subset.

---

## Phase 3: Foundational (Blocking Prerequisites for Code-Bearing Stories)

**Purpose**: Prepare shared compiler contracts after ratification so validation and backend implementation can proceed safely.

**⚠️ CRITICAL**: No code-bearing user story work should begin until this phase is complete.

- [ ] T014 Update span-aware diagnostic structures and rendering in `compiler/src/diagnostics.rs` and `compiler/src/lib.rs`
- [ ] T015 [P] Preserve deterministic source discovery and project layout contracts in `compiler/src/project.rs` and `compiler/src/config.rs`
- [ ] T016 [P] Extend parser-facing node contracts for the ratified selector and event-reference surface in `parser/src/ast.rs` and `parser/src/lib.rs`
- [ ] T017 [P] Define canonical first-slice IR contracts for ratified widgets, normalized styles, and event metadata in `ir/src/lib.rs`
- [ ] T018 [P] Make normative versus aspirational test and snapshot participation explicit in `tests/README.md` and `backend/lvgl_c/tests/generation.rs`
- [ ] T019 Confirm repository-level fixture and mapping claims in `README.md`, `examples/minimal/README.md`, `examples/dashboard/README.md`, and `docs/LVGL_MAPPING.md` do not exceed the ratified slice

**Checkpoint**: Shared diagnostics, discovery, AST, IR, and fixture/snapshot participation rules are ready for code-bearing story implementation.

---

## Phase 4: User Story 2 - Validate the Normative Thin Slice (Priority: P2)

**Goal**: Parse and validate the ratified subset so accepted input succeeds and out-of-scope input fails with actionable diagnostics.

**Independent Test**: Run `cargo test -p lumaui-parser`, `cargo test -p lumaui-semantic`, and `cargo run -p lumaui-cli -- validate examples/minimal`; confirm `examples/minimal` validates successfully while shared invalid fixtures fail with source-located diagnostics.

### Tests for User Story 2 ⚠️

- [ ] T020 [P] [US2] Add parser success and failure coverage for the ratified subset in `parser/src/parse.rs`
- [ ] T021 [P] [US2] Add semantic validation coverage for duplicate ids, unsupported widgets, unsupported properties, event references, and binding rejection in `semantic/src/lib.rs`
- [ ] T022 [P] [US2] Wire shared invalid fixtures into regression scenarios in `tests/fixtures/unsupported_widget.lui`, `tests/fixtures/binding_reference.lui`, `tests/fixtures/unsupported_selector.lus`, `tests/fixtures/unsupported_property.lus`, and `tests/fixtures/duplicate_ids.lui`

### Implementation for User Story 2

- [ ] T023 [US2] Implement ratified markup and style parsing in `parser/src/parse.rs` and `parser/src/lexer.rs`
- [ ] T024 [P] [US2] Update exported parser node shapes and parse outcomes in `parser/src/ast.rs` and `parser/src/lib.rs`
- [ ] T025 [US2] Implement source-located parse and validation diagnostics in `compiler/src/diagnostics.rs` and `semantic/src/lib.rs`
- [ ] T026 [US2] Implement supported-surface validation, event-reference handling, and binding rejection in `semantic/src/lib.rs`
- [ ] T027 [US2] Wire deterministic validate-command behavior over parsed documents in `cli/src/main.rs` and `compiler/src/project.rs`

**Checkpoint**: `validate` becomes a trustworthy phase-gated command for the normative example and the shared expected-fail fixtures.

---

## Phase 5: User Story 3 - Deliver One Phase-Gated End-to-End Build Path (Priority: P3)

**Goal**: Lower the validated minimal slice into canonical IR and generate deterministic, readable LVGL output with explicit ownership boundaries.

**Independent Test**: Run `cargo test -p lumaui-backend-lvgl-c` and `cargo run -p lumaui-cli -- build examples/minimal`; confirm repeated builds produce stable generated artifacts, preserve documented ownership boundaries, and keep aspirational fixtures out of the current snapshot path.

### Tests for User Story 3 ⚠️

- [ ] T028 [P] [US3] Add canonical IR lowering coverage for the minimal slice in `semantic/src/lib.rs` and `ir/src/lib.rs`
- [ ] T029 [P] [US3] Add frontend-driven backend snapshot coverage, including ownership-boundary expectations, in `backend/lvgl_c/tests/generation.rs` and `tests/snapshots/minimal_screen.c`
- [ ] T030 [P] [US3] Add build-command smoke coverage for the minimal project and explicit non-participation or expected-fail handling for aspirational fixtures in `cli/src/main.rs` and `backend/lvgl_c/tests/generation.rs`

### Implementation for User Story 3

- [ ] T031 [US3] Extend canonical IR structures for normalized widgets, style applications, event metadata, and ownership annotations in `ir/src/lib.rs`
- [ ] T032 [US3] Implement semantic lowering from parsed documents into canonical IR in `semantic/src/lib.rs`
- [ ] T033 [US3] Implement deterministic first-slice LVGL emission with compiler-owned and user-owned region boundaries in `backend/lvgl_c/src/lib.rs`
- [ ] T034 [US3] Wire `lumaui build` to parse, validate, lower, and generate into `output_dir` in `cli/src/main.rs`
- [ ] T035 [US3] Reconcile generated-file naming, stable ordering, ownership-boundary output, and snapshot expectations in `backend/lvgl_c/src/lib.rs`, `backend/lvgl_c/tests/generation.rs`, and `tests/snapshots/minimal_screen.c`

**Checkpoint**: The minimal example builds end to end into deterministic LVGL artifacts with explicit ownership semantics and visible snapshot/test scope.

---

## Phase 6: Polish & Cross-Cutting Concerns

**Purpose**: Lock in consistency, determinism, and operator guidance across the completed slice.

- [ ] T036 [P] Update execution guidance and verification steps in `docs/TASKS.md`, `docs/NEXT_STEPS.md`, and `specs/001-brownfield-spec/quickstart.md`
- [ ] T037 [P] Update fixture and snapshot guidance in `tests/README.md`, `examples/minimal/README.md`, and `examples/dashboard/README.md`
- [ ] T038 Verify `doctor`, `validate`, and `build` behavior against `examples/minimal` and expected-fail fixtures, fixing any issues in `cli/src/main.rs`, `parser/src/parse.rs`, `semantic/src/lib.rs`, and `backend/lvgl_c/src/lib.rs`
- [ ] T039 Review preview deferral messaging, generated-output ownership wording, and LVGL mapping consistency in `cli/src/main.rs`, `backend/lvgl_c/src/lib.rs`, and `docs/LVGL_MAPPING.md`

---

## Dependencies & Execution Order

### Phase Dependencies

- **Phase 1: Setup**: No dependencies; can start immediately.
- **Phase 2: User Story 1**: Depends on Phase 1; ratification must happen before parser-, IR-, or backend-shaped work.
- **Phase 3: Foundational**: Depends on User Story 1; blocks all later code-bearing user stories.
- **Phase 4: User Story 2**: Depends on Phase 3; validation work requires the ratified supported surface and shared contracts.
- **Phase 5: User Story 3**: Depends on Phase 4; backend generation requires parsed and semantically validated canonical IR.
- **Phase 6: Polish**: Depends on the stories you want to ship being complete.

### User Story Dependencies

- **User Story 1 (P1)**: Starts after Setup and is the MVP contract freeze.
- **User Story 2 (P2)**: Depends on User Story 1 plus Foundational because validation cannot be implemented honestly before ratification and shared contracts are in place.
- **User Story 3 (P3)**: Depends on User Story 2 because build output depends on validated, lowered input.

### Within Each User Story

- Add or update tests and fixtures before implementation for parser, semantic, IR, backend, or CLI behavior.
- Complete documentation ratification before modifying contracts that depend on language shape.
- Update shared contracts before cross-stage wiring.
- Keep stage-local work isolated before integrating across crates.
- Complete story verification before moving to the next priority.

### Parallel Opportunities

- `T002` and `T003` can run in parallel after `T001`.
- `T007`, `T008`, `T009`, and `T011` can run in parallel after `T006`.
- `T015`, `T016`, `T017`, and `T018` can run in parallel after `T014`.
- `T020`, `T021`, and `T022` can run in parallel within User Story 2.
- `T024` can run in parallel with `T025` after parser test scaffolding is in place.
- `T028`, `T029`, and `T030` can run in parallel within User Story 3.
- `T036` and `T037` can run in parallel during polish.

---

## Parallel Example: User Story 1

```bash
Task: T007 Align active phase, gate order, and immediate-work language in docs/TASKS.md and docs/NEXT_STEPS.md
Task: T008 Align stage boundaries and narrow-slice guidance in docs/ARCHITECTURE.md and docs/LVGL_MAPPING.md
Task: T009 Align project overview, roadmap wording, and current-status messaging in README.md
Task: T011 Mark aspirational fixture status and out-of-scope constructs in examples/dashboard/README.md, examples/dashboard/ui/screens/dashboard.lui, and examples/dashboard/ui/styles/theme.lus
```

## Parallel Example: User Story 2

```bash
Task: T020 Add parser success and failure coverage for the ratified subset in parser/src/parse.rs
Task: T021 Add semantic validation coverage for duplicate ids, unsupported widgets, unsupported properties, event references, and binding rejection in semantic/src/lib.rs
Task: T022 Wire shared invalid fixtures into regression scenarios in tests/fixtures/unsupported_widget.lui, tests/fixtures/binding_reference.lui, tests/fixtures/unsupported_selector.lus, tests/fixtures/unsupported_property.lus, and tests/fixtures/duplicate_ids.lui
```

## Parallel Example: User Story 3

```bash
Task: T028 Add canonical IR lowering coverage for the minimal slice in semantic/src/lib.rs and ir/src/lib.rs
Task: T029 Add frontend-driven backend snapshot coverage, including ownership-boundary expectations, in backend/lvgl_c/tests/generation.rs and tests/snapshots/minimal_screen.c
Task: T030 Add build-command smoke coverage for the minimal project and explicit non-participation or expected-fail handling for aspirational fixtures in cli/src/main.rs and backend/lvgl_c/tests/generation.rs
```

---

## Implementation Strategy

### MVP First (User Story 1 Only)

1. Complete Phase 1: Setup.
2. Complete Phase 2: User Story 1.
3. Stop and validate that docs, fixture labels, and starter content all describe the same supported first slice.

### Incremental Delivery

1. Finish Setup and User Story 1 to ratify the first slice and classify fixtures.
2. Finish Foundational work to stabilize shared diagnostics, discovery, AST, IR, and fixture/snapshot participation rules.
3. Deliver User Story 2 to make `validate` trustworthy for the ratified slice.
4. Deliver User Story 3 to make `build` produce stable LVGL output from the validated slice with explicit ownership boundaries.
5. Finish Polish to keep docs, fixtures, tests, and snapshots aligned.

### Suggested Team Split

1. One contributor owns docs and fixture ratification in User Story 1.
2. One contributor owns shared diagnostics and contract work in the Foundational phase.
3. One contributor owns parser and validation work in User Story 2.
4. One contributor owns semantic, IR, backend, and build wiring in User Story 3 after User Story 2 unblocks the path.

---

## Notes

- `[P]` tasks touch different files or can proceed without waiting on unfinished sibling tasks.
- User Story 1 is the suggested MVP because it freezes the contract that every later story depends on.
- Foundational work now follows ratification so the task order matches the constitution’s ratification-first rule.
- User Story 3 now carries explicit ownership-boundary and aspirational-fixture visibility work so generated-output and snapshot expectations are fully represented.