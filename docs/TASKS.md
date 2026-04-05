# Tasks

## How To Use This Document

This roadmap is phase-based.

- Each phase has a clear purpose.
- Each phase has explicit entry assumptions.
- Each phase has an exit gate.
- Work should stay inside the current phase unless a dependency is already satisfied.

Document storage and filing rules live in `docs/DOCUMENTATION_SCHEME.md` and should be followed when adding or relocating repository or feature documents.

For the active brownfield feature slice, `001-brownfield-spec` is the integration branch. Execute each documented phase on its own child branch created from the current tip of `001-brownfield-spec`, open the phase PR back into `001-brownfield-spec`, and cut the next phase branch only after the previous phase PR merges.

## Engineering Rules

- Start code-bearing work with failing tests, fixtures, snapshots, or command assertions before implementation.
- Keep stage boundaries and helper responsibilities narrow; refactor duplication or leaky abstractions when they reduce clarity or testability.
- Treat diagnostics and logging as part of the product surface for operator-facing commands. Logging should be deterministic, intentional, and separate from generated artifacts.
- Update documentation in the same change when behavior, support status, fixtures, or command output changes.
- Keep ratified behavior, deferred work, and aspirational examples clearly labeled so the roadmap does not overstate implementation status.
- For stage-shaping choices, prepare written discussion material before locking in implementation: options, pros/cons, relevant practices, risks, and open questions.
- Major architectural or language decisions are not final until the developer explicitly signs them off.

## Phase 0: Foundation

### Purpose

Establish a coherent repository, documentation pack, workspace layout, examples, and compiler scaffolding.

### Entry Assumptions

- project intent is agreed
- Rust is the chosen implementation language

### Deliverables

- root docs
- workspace and crate layout
- config loading
- source discovery
- diagnostics primitives
- provisional examples
- initial tests and snapshots

### Exit Gate

- root docs exist and agree on scope
- Rust workspace is scaffolded
- compiler stages have dedicated crates or modules
- examples exist for minimal and dashboard-shaped projects
- AGENTS guidance is present
- tests and snapshot directories are present

### Status

Current repository state: substantially complete

## Phase 1: Language Freeze and Parsing

### Purpose

Turn provisional frontend scaffolding into a real, testable parser for a narrow MVP subset.

### Entry Assumptions

- Phase 0 exit gate is satisfied
- the first supported language slice is explicitly documented

### Deliverables

- ratified markup grammar for the first supported subset
- ratified style grammar for the first supported subset
- real AST construction
- syntax diagnostics with spans
- deterministic parser-stage logging guidance where command behavior exposes it
- parser unit tests for success and failure cases

### Exit Gate

- language grammar is explicitly ratified before broad parser expansion
- markup parser supports the MVP screen tree subset
- style parser supports the MVP declaration subset
- diagnostics report source spans and actionable messages
- operator-visible parser logging is intentional and non-noisy where present
- parser unit tests cover success and failure cases
- `examples/minimal` parses end-to-end
- supporting research and discussion material for the ratified slice has been reviewed by the developer

### Immediate Work

1. reduce the MVP language slice if needed
2. update `LANGUAGE_SPEC.md` to ratify that slice
3. implement parser tests first
4. implement the parser against `examples/minimal`

## Phase 2: Semantic Analysis and IR

### Purpose

Resolve authored sources into a backend-ready, typed model.

### Entry Assumptions

- Phase 1 exit gate is satisfied
- parser output is stable enough to treat as input rather than a moving target

### Deliverables

- duplicate id checks
- supported widget validation
- property validation
- normalized semantic property model
- explicit event and binding references
- deterministic validation-stage logging guidance where command behavior exposes it
- lowering into the canonical IR

### Exit Gate

- duplicate ids are detected
- unknown widgets and properties are rejected cleanly
- style rules are validated and normalized
- event and binding references are represented explicitly
- semantic output lowers into the canonical IR
- operator-visible validation logging is intentional and stable where present
- semantic validation tests cover key failures
- shared contract changes have been reviewed and signed off by the developer before downstream stages rely on them

### Immediate Work

1. define the semantic model for the first supported slice
2. normalize style declarations into explicit forms
3. lower `examples/minimal` into IR

## Phase 3: LVGL C Backend

### Purpose

Generate deterministic, readable LVGL C from IR.

### Entry Assumptions

- Phase 2 exit gate is satisfied
- LVGL mapping for the implemented subset is explicit

### Deliverables

- generated screen `.c` and `.h` files
- stable symbol naming
- deterministic emission order
- readable generated code
- deterministic build-stage logging guidance where command behavior exposes it
- snapshot coverage for generated output

### Exit Gate

- screen files generate stable `.c` and `.h` output
- symbol naming is deterministic
- supported widgets map to documented LVGL APIs
- generated code is readable and sensibly structured
- operator-visible build logging is intentional and does not leak into generated files
- snapshot tests cover representative screens
- one real example builds through the full compiler path
- generated-file policy and backend mapping choices used by the slice have developer sign-off

### Immediate Work

1. connect `build` to the real semantic IR
2. replace synthetic backend-only fixtures with frontend-driven snapshots
3. document unsupported emitted cases clearly

## Phase 4: Examples and Stability

### Purpose

Make examples and tests function as trusted golden fixtures and stabilize the implemented MVP.

### Entry Assumptions

- Phase 3 exit gate is satisfied

### Deliverables

- golden example coverage
- parser regression tests
- semantic regression tests
- generated C snapshot tests
- CLI-level validation coverage where practical

### Exit Gate

- `examples/minimal` validates end-to-end
- `examples/dashboard` validates end-to-end or is clearly marked as aspirational if still ahead of implementation
- examples are used in automated tests where practical
- snapshot coverage exists for generated C
- regression tests capture known parser and semantic edge cases

### Immediate Work

1. decide which examples are normative versus aspirational
2. align examples with actual supported syntax
3. expand regressions around real bugs, not hypothetical ones

## Phase 5: Preview Integration

### Purpose

Add a preview path without compromising compiler-first design.

### Entry Assumptions

- Phase 4 exit gate is satisfied
- generated output flow is stable enough to preview honestly

### Deliverables

- `lumaui preview`
- LVGL SDL-oriented preview orchestration
- prerequisite detection and failure guidance
- minimal preview docs

### Exit Gate

- `lumaui preview` launches through an LVGL SDL-oriented workflow
- preview uses generated artifacts rather than a separate runtime model
- failure states are clear when LVGL or SDL prerequisites are missing
- preview remains optional and does not distort the language surface

## Cross-Phase Rules

- Do not widen language scope to unblock parser work; narrow the slice instead.
- Do not add backend cleverness to compensate for unclear semantics.
- Do not let examples promise more than the implementation supports.
- Keep Git review scope aligned with phase gates: one phase branch per phase, PR base `001-brownfield-spec`, then branch the next phase from the updated integration tip.
- Update docs, fixtures, and tests together.
- Update logging expectations and command assertions together when operator-visible behavior changes.
- Prefer concise, reviewable documentation updates over broad prose that drifts from the actual implementation.
- Produce discussion documents before finalizing major stage decisions, and defer the final call to the developer.
