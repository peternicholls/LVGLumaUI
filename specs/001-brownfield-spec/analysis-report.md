# Specification Analysis Report: Brownfield MVP Compiler Slice

**Feature**: `001-brownfield-spec` — Brownfield MVP Compiler Slice  
**Artifacts scanned**: `spec.md`, `plan.md`, `tasks.md`, `.specify/memory/constitution.md`  
**Generated**: 2026-04-05  
**Status**: Historical read-only analysis; constitution path references normalized after later governance cleanup.

---

## Findings Table

| ID | Category | Severity | Location(s) | Summary | Recommendation |
|----|----------|----------|-------------|---------|----------------|
| D1 | Duplication | HIGH | tasks.md: T003, T018, T037 | All three tasks write to `tests/README.md` with near-identical scope: T003 (Phase 1) covers normative/expected-fail/aspirational guidance; T018 (Phase 3) makes fixture participation "explicit" again in the same file; T037 (Phase 6) updates it a third time. The evolving boundary between passes is not defined, risking conflicting edits or false confidence that earlier passes completed the job. | Consolidate to distinct non-overlapping scopes: T003 = content categories and classification conventions only; T018 = snapshot and observable logging participation only; T037 = final wording consistency pass with no structural changes. |
| D2 | Duplication | HIGH | tasks.md: T004, T011 | Both tasks target `examples/dashboard/README.md`. T004 (Phase 1 Setup) sets "example status guidance"; T011 (Phase 2 US1) marks "aspirational fixture status." Same object, same purpose, same file. Neither references the other. Risk: T004's work gets silently overwritten or contradicted in T011. | Restrict T004 to creating a classification scaffold (structure only, no ratified content) with an explicit forward reference to T011. Let T011 fill ratified aspirational content post-T006. |
| D3 | Duplication | HIGH | tasks.md: T013, T019 | T013 (Phase 2 US1) reconciles docs and fixture labels across `README.md`, `examples/minimal/README.md`, `examples/dashboard/README.md`. T019 (Phase 3 Foundational) confirms fixture and mapping claims do not exceed the ratified slice in the same three files plus `docs/LVGL_MAPPING.md`. Three shared files, similar mandate. | Give each a distinct functional mandate. T013 = terminology and status-wording consistency (a writing task). T019 = diff-verify that post-T013 content does not over-claim the ratified slice (a review task). Remove `README.md` and the two example READMEs from T019's write scope; add only `docs/LVGL_MAPPING.md` as T019's sole new file. |
| C1 | Coverage Gap | HIGH | tasks.md: T038; spec.md: (no FR) | T038 (Phase 6) says "verify `doctor`, `validate`, and `build` behavior." `validate` has T027 and `build` has T034, but no task implements `doctor`. No FR in spec.md specifies what `doctor` must do. There is no implementation task, no test task, and no spec requirement covering it. T038's verification will have nothing to verify for `doctor`. | Remove `doctor` from T038's scope, add an explicit deferral note to T038, and record `doctor` implementation as deferred to a later feature slice. |
| A1 | Ambiguity | MEDIUM | tasks.md: T017, T031 | T017 (Phase 3): "Define canonical first-slice IR contracts … in `ir/src/lib.rs`." T031 (Phase 5): "Extend canonical IR structures … in `ir/src/lib.rs`." Both share "normalized styles/widgets" and "event metadata." An implementer cannot tell what T031 adds beyond T017. | State explicitly: T017 = type definitions and field contracts only, no business logic. T031 = adds ownership annotations and style-application nodes not available until T032 semantic lowering is in place. Add a forward dependency note to T031. |
| A2 | Ambiguity | MEDIUM | tasks.md: T020, T023 | T020 adds test coverage only for `parser/src/parse.rs`. T023 also touches `parser/src/lexer.rs` (implementation). No test task covers lexer behavior. Constitution Principle V requires docs, fixtures, and tests to travel with every meaningful compiler change. | Expand T020 to include lexer test expectations: token edge cases, unknown characters, binding-syntax token rejection, and span boundary correctness for `parser/src/lexer.rs`. |
| A3 | Ambiguity | MEDIUM | tasks.md: T005, T012; spec.md: (no FR) | T005 and T012 reference "starter project output" in `cli/src/main.rs`. No FR covers `init`/starter-template behavior. The behavioral contract lives only in `contracts/cli-commands.md`, which is a prerequisite doc but not anchored to any FR. Without an FR anchor, T005's regression coverage has no normative target to regress against. | Annotate FR-015 to explicitly include `init`/starter-template output as in-scope for Phase 1 alignment, or add a note to T005 citing the exact clause in `contracts/cli-commands.md` that defines the acceptance target. |
| A4 | Ambiguity | MEDIUM | spec.md: Constitution Alignment | The spec names five ratified widgets but does not state the LVGL 9.x primitives they map to. Constitution Principle II says all constructs MUST map to a named LVGL 9.x primitive before implementation is approved. The requirement is delegated to `docs/LVGL_MAPPING.md` with no explicit verification gate. | Add an inline LVGL mapping table to the spec's Constitution Alignment section or to the plan's Technical Context. Add an explicit verification sub-step to T008: "Confirm `docs/LVGL_MAPPING.md` contains named LVGL 9.x primitives for all five ratified widgets before T008 is considered complete." |
| U1 | Underspecification | MEDIUM | spec.md: SC-007 | SC-007 ("The next implementation sequence can proceed without requiring a scope-expanding redesign") is not measurable or buildable. It describes a governance discipline, cannot be tested or verified with a fixture, and has no coverage task. | Relocate SC-007 to the Assumptions section, or restate it as a process gate: "Each phase gate must be explicitly signed off before the next phase begins, confirmed via the Decision Gate notes in tasks.md." |
| I1 | Inconsistency | MEDIUM | tasks.md: T038; plan.md: Workstream 5 | T038 says "fixing any issues" in four cross-stage files (`cli/src/main.rs`, `parser/src/parse.rs`, `semantic/src/lib.rs`, `backend/lvgl_c/src/lib.rs`). A Polish-phase task that implements open-ended fixes across four stages contradicts the plan's single-purpose module guidance and constitution Principle IV (Strict Layer Isolation). | Restate T038 as verification-only. Fixes discovered during T038 must be raised as targeted errata against the specific prior task (T027, T034, etc.) rather than applied inline. |
| I2 | Inconsistency | MEDIUM | spec.md: FR-001; tasks.md: (missing) | FR-001 should cite `.specify/memory/constitution.md` as the governing authority, but the surrounding brownfield artifacts still imply a separate active docs-level constitution file. Because the repo only keeps `docs/archive/CONSTITUTION.md` as historical context, leaving the active path implicit makes the governing authority easy to misread. | Add a sub-step to T013 or T019: "Verify brownfield artifacts cite `.specify/memory/constitution.md` as the governing authority and treat `docs/archive/CONSTITUTION.md` as archival-only context." |
| L1 | Style/Clarity | LOW | tasks.md: T039 | T039 is a review task with no stated definition of done. It is undefined whether completing the review produces a sign-off comment, a doc edit, a filed issue, or nothing. | Add definition of done: each item reviewed results in either (a) confirmed-no-change with a comment or (b) a concrete edit landing in the referenced file within the same T039 scope. |
| L2 | Style/Clarity | LOW | tasks.md: T005 | T005 is tagged [P] (parallel), implying it can run concurrently with other US1 tasks. Regression coverage for "the ratified first slice" cannot usefully exist before T006 ratifies that slice. The parallel marker is misleading and could produce wasted rework. | Remove [P] from T005 and add: "Depends on T006 completing ratification." |
| L3 | Style/Clarity | LOW | tasks.md: Phase 2 ordering | T010, T012, and T013 carry no [P] markers but have no explicit ordering note relative to T006. T010 depends on T006; T012 depends on T006 and T010; T013 depends on T006, T010, and T011. | Add explicit ordering notes below the Phase 2 task list: "T010 after T006; T012 after T006 and T010; T013 after T006, T010, and T011." |
| L4 | Style/Clarity | LOW | spec.md: FR-006 | FR-006 ("remain narrow enough that it does not force simultaneous redesign of parser, semantic, IR, and backend layers") is a design constraint, not a verifiable delivery requirement. Its presence in the FR list implies coverage — it has none. | Move FR-006 to the Assumptions section, or annotate it: "Phase-gate constraint enforced by the Phase dependency rules in tasks.md; not an independently testable requirement." |

---

## Coverage Summary Table

| Requirement Key | Has Task? | Task IDs | Notes |
|-----------------|-----------|----------|-------|
| FR-001 | ✅ | T006, T007, T008, T009, T013 | |
| FR-002 | ✅ | T007, T009 | |
| FR-003 | ✅ | T006, T010 | |
| FR-004 | ✅ | T006, T010, T016 | |
| FR-005 | ✅ | T006, T016 | |
| FR-006 | ⚠️ | T014–T017 (indirect) | Governance constraint; no direct verifiable coverage. See L4. |
| FR-007 | ✅ | T007, T008, T036 | |
| FR-008 | ✅ | T004, T010, T013 | T004/T011 duplication noted in D2. |
| FR-009 | ✅ | T020–T027 | |
| FR-010 | ✅ | T001, T002, T020–T026 | |
| FR-011 | ✅ | T021, T026, T028, T031, T032 | |
| FR-012 | ✅ | T028–T035 | |
| FR-013 | ✅ | T029, T033, T035 | |
| FR-014 | ✅ | T003, T004, T011, T018, T030, T037 | Triple-write on `tests/README.md` noted in D1. |
| FR-015 | ✅ | T006–T009, T013, T015, T019, T035–T039 | |
| FR-016 | ✅ | T006, T011, T013, T039 | |
| SC-001 | ✅ | T007–T009, T013 | |
| SC-002 | ✅ | T010, T013 | |
| SC-003 | ✅ | T020–T027 | |
| SC-004 | ✅ | T028–T035 | |
| SC-005 | ✅ | T029, T033, T035 | |
| SC-006 | ✅ | T003, T004, T011, T037 | |
| SC-007 | ❌ | — | Not buildable or measurable. See U1. Recommend relocating to Assumptions. |

---

## Constitution Alignment Issues

| Principle | Status | Notes |
|-----------|--------|-------|
| I. Compile-Time First | ✅ Pass | No runtime or DOM semantics introduced. |
| II. Embedded-Safe LVGL Mapping | ⚠️ Partial | Five widgets listed; LVGL 9.x primitives not named inline. Delegated to `docs/LVGL_MAPPING.md` without an explicit verification gate. See A4. |
| III. Deterministic, Readable Artifacts | ✅ Pass | SC-005, T029, T033, T035 address this. |
| IV. Strict Layer Isolation | ✅ Pass | Phase gates enforce stage ordering. T038 polish scope is a concern (I1) but is not a structural violation. |
| V. Ratified Narrow Slices w/ Docs/Tests/Fixtures | ✅ Pass | Phase 2 gates ratification before parser work. Phase gate dependencies are explicit. |
| Operational: constitution path alignment | ⚠️ Gap | No task verifies that brownfield artifacts cite `.specify/memory/constitution.md` as the governing authority and treat `docs/archive/CONSTITUTION.md` as archival-only context. See I2. |
| Workflow: language shape ratified before implementation | ✅ Pass | Phase 3 Foundational is blocked on Phase 2 (User Story 1). |

---

## Unmapped Tasks

| Task | Notes |
|------|-------|
| T038 (`doctor` verification) | References `doctor` command with no implementation task backing it. See C1. |
| T039 (Polish review) | Loosely maps to FR-013/FR-016 but lacks a concrete acceptance criterion. See L1. |

All other 37 tasks map to at least one FR, SC, or User Story.

---

## Metrics

| Metric | Value |
|--------|-------|
| Total Functional Requirements | 16 |
| Total Success Criteria (spec) | 7 (6 buildable, 1 governance-only) |
| Total Tasks | 39 |
| FR Coverage (≥1 task) | 16/16 = 100% |
| SC Coverage (buildable) | 6/6 = 100% |
| Critical Issues | 0 |
| High Issues | 4 (D1, D2, D3, C1) |
| Medium Issues | 7 (A1, A2, A3, A4, U1, I1, I2) |
| Low Issues | 4 (L1, L2, L3, L4) |

---

## Next Actions

No CRITICAL issues were found. All 16 FRs and 6 buildable SCs have task coverage. The artifacts are constitutionally sound at the structural level.

**Resolve these HIGH issues before `/speckit.implement`:**

1. **D1** — Scope-separate T003, T018, T037 so each writes a distinct section of `tests/README.md`.
2. **D2** — Limit T004 to a classification scaffold only; let T011 fill ratified content after T006.
3. **D3** — Give T013 a writing mandate (terminology/wording) and T019 a diff-review mandate (no over-claiming); remove their shared file overlap.
4. **C1** — Remove `doctor` from T038's scope, add an explicit deferral note.

---

## Recommended Edits

The edits below address all HIGH issues and the two most impactful MEDIUM issues (A1, A2). Apply them to `tasks.md` and `spec.md`.

---

### tasks.md

#### D1 — Scope-separate the three `tests/README.md` tasks

**T003** — Replace:

```
- [ ] T003 [P] Update fixture usage guidance in `tests/README.md` for normative, expected-fail, and aspirational authored-source coverage, including documentation conventions for each class
```

With:

```
- [ ] T003 [P] Add fixture content-category conventions to `tests/README.md`: define normative, expected-fail, and aspirational categories and document the naming and labeling convention for each class (structural scaffold only; snapshot participation rules are deferred to T018)
```

**T018** — Replace:

```
- [ ] T018 [P] Make normative versus aspirational test, snapshot, and observable logging participation explicit in `tests/README.md` and `backend/lvgl_c/tests/generation.rs`
```

With:

```
- [ ] T018 [P] Add snapshot and observable logging participation rules to `tests/README.md` (builds on T003 category scaffold): document which fixture categories participate in snapshot regression, which are expected-fail, and how logging output is captured per category; update `backend/lvgl_c/tests/generation.rs` to reflect these participation rules
```

**T037** — Replace:

```
- [ ] T037 [P] Update fixture, snapshot, and accompanying documentation guidance in `tests/README.md`, `examples/minimal/README.md`, and `examples/dashboard/README.md`
```

With:

```
- [ ] T037 [P] Final wording-consistency pass on `tests/README.md`, `examples/minimal/README.md`, and `examples/dashboard/README.md`: confirm category labels, participation rules, and aspirational/normative distinctions use consistent terminology; no structural changes — raise any structural issues as errata against T003 or T018
```

---

#### D2 — Restrict T004 to scaffold only

**T004** — Replace:

```
- [ ] T004 Update example status guidance in `examples/minimal/README.md` and `examples/dashboard/README.md` so later tasks can classify fixtures consistently and document intended support status clearly
```

With:

```
- [ ] T004 Add a classification scaffold to `examples/minimal/README.md` and `examples/dashboard/README.md`: insert placeholder sections for "Support Status", "Ratified Constructs", and "Deferred Constructs" with no ratified content yet — content is filled by T010 (minimal) and T011 (dashboard) after T006 completes ratification
```

---

#### D3 — Give T013 and T019 distinct non-overlapping mandates

**T013** — Replace:

```
- [ ] T013 [US1] Reconcile all slice-level docs and fixture labels across `docs/LANGUAGE_SPEC.md`, `README.md`, `examples/minimal/README.md`, and `examples/dashboard/README.md`, keeping terminology and support-status wording consistent
```

With:

```
- [ ] T013 [US1] Terminology and status-wording consistency pass across `docs/LANGUAGE_SPEC.md`, `README.md`, `examples/minimal/README.md`, and `examples/dashboard/README.md`: ensure the same terms are used for ratified constructs, deferred constructs, normative fixtures, and aspirational fixtures in every doc; this is a writing task — no content changes beyond wording alignment
```

**T019** — Replace:

```
- [ ] T019 Confirm repository-level fixture and mapping claims in `README.md`, `examples/minimal/README.md`, `examples/dashboard/README.md`, and `docs/LVGL_MAPPING.md` do not exceed the ratified slice and remain documentation-consistent
```

With:

```
- [ ] T019 Diff-review `docs/LVGL_MAPPING.md` against the ratified first-slice widget and property set: confirm every listed construct has a named LVGL 9.x primitive mapping and no construct exceeds the ratified subset — raise any over-claim as an errata note; explicitly verify brownfield artifacts cite `.specify/memory/constitution.md` as the governing authority and treat `docs/archive/CONSTITUTION.md` as archival-only context where relevant (see FR-001)
```

> Note: `README.md`, `examples/minimal/README.md`, and `examples/dashboard/README.md` are removed from T019's scope because T013 owns wording alignment there. T019 is now a targeted review of `docs/LVGL_MAPPING.md` and the constitution alignment check from finding I2.

---

#### C1 — Remove `doctor` from T038 and add deferral note

**T038** — Replace:

```
- [ ] T038 Verify `doctor`, `validate`, and `build` behavior plus observable logging behavior against `examples/minimal` and expected-fail fixtures, fixing any issues in `cli/src/main.rs`, `parser/src/parse.rs`, `semantic/src/lib.rs`, and `backend/lvgl_c/src/lib.rs`
```

With:

```
- [ ] T038 Verify `validate` and `build` behavior plus observable logging behavior against `examples/minimal` and expected-fail fixtures; confirm operator-visible output matches the contracts in `contracts/cli-commands.md`; raise any discrepancies as errata against T027 (validate) or T034 (build) rather than applying inline fixes — `doctor` command implementation is explicitly deferred to a later feature slice
```

---

#### A1 — Clarify the T017/T031 scope boundary in `ir/src/lib.rs`

**T017** — Replace:

```
- [ ] T017 [P] Define canonical first-slice IR contracts for ratified widgets, normalized styles, and event metadata in `ir/src/lib.rs`
```

With:

```
- [ ] T017 [P] Define canonical first-slice IR type definitions and field contracts for ratified widgets, normalized styles, and event metadata in `ir/src/lib.rs` — type definitions and struct shapes only; no business logic, lowering, or ownership annotations (those are added in T031 after T032 semantic lowering is available)
```

**T031** — Replace:

```
- [ ] T031 [US3] Extend canonical IR structures for normalized widgets, style applications, event metadata, and ownership annotations in `ir/src/lib.rs`
```

With:

```
- [ ] T031 [US3] Extend `ir/src/lib.rs` with ownership annotations and style-application IR nodes not available until T032 semantic lowering is implemented: add the compiler-owned/user-owned region markers and style-application relationships required by the backend ownership model (depends on T017 type contracts and T032 lowering work)
```

---

#### A2 — Add lexer test coverage expectation to T020

**T020** — Replace:

```
- [ ] T020 [P] [US2] Add parser success and failure coverage for the ratified subset in `parser/src/parse.rs`
```

With:

```
- [ ] T020 [P] [US2] Add parser and lexer success and failure coverage for the ratified subset: parser tests in `parser/src/parse.rs` covering valid and invalid first-slice markup and style documents; lexer tests covering token edge cases, unknown characters, binding-syntax token rejection, and span boundary correctness in `parser/src/lexer.rs`
```

---

#### L2 — Remove misleading [P] from T005

**T005** — Replace:

```
- [ ] T005 [P] [US1] Add CLI starter-template regression coverage for the ratified first slice in `cli/src/main.rs`
```

With:

```
- [ ] T005 [US1] Add CLI starter-template regression coverage for the ratified first slice in `cli/src/main.rs` — depends on T006 completing language ratification before regression targets are defined
```

---

#### L3 — Add explicit ordering notes to Phase 2

After the Phase 2 checkpoint block, add:

```
**Intra-phase ordering (User Story 1)**:

- T010 depends on T006 (normative fixture update requires a ratified grammar).
- T012 depends on T006 and T010 (starter output alignment requires ratified grammar and updated normative fixture).
- T013 depends on T006, T010, and T011 (terminology reconciliation requires all ratified and aspirational content to be in place).
- T005 depends on T006 (regression targets cannot be written before ratification is complete).
```

---

### spec.md

#### U1 — Relocate SC-007 to Assumptions

**SC-007** — Remove from the Success Criteria list:

```
- **SC-007**: The next implementation sequence can proceed from ratified language scope through parser, semantic, backend, and fixture-stability work without requiring a scope-expanding redesign of multiple stages at once.
```

Add to the Assumptions section:

```
- The phase-gate dependency rules in `tasks.md` are sufficient to enforce narrow-slice sequencing. Proceeding without requiring multi-stage redesign is an execution constraint, not a measurable outcome, and does not require its own test or fixture.
```

---

#### L4 — Annotate FR-006 as a governance constraint

**FR-006** — Replace:

```
- **FR-006**: The supported first slice MUST remain narrow enough that it does not force simultaneous redesign of parser, semantic, IR, and backend layers beyond the thin-slice sequencing described in the brownfield docs.
```

With:

```
- **FR-006**: *(Phase-gate constraint)* The supported first slice MUST remain narrow enough that it does not force simultaneous redesign of parser, semantic, IR, and backend layers beyond the thin-slice sequencing described in the brownfield docs. This is enforced by the Phase dependency rules in `tasks.md` and is not an independently testable requirement; it does not require a dedicated test, fixture, or acceptance scenario.
```
