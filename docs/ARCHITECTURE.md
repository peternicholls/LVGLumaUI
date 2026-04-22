# Architecture

## Purpose

LumaUI is structured as a staged compiler, not a monolith. Each stage owns a distinct transformation so that syntax, meaning, and backend mapping remain understandable and testable.

The architecture also assumes two cross-cutting disciplines:

- stage ownership stays narrow so each crate has one clear reason to change
- observability is intentional so operators can see pipeline progress and failures without blurring stage responsibilities

It also assumes decision discipline: architecture-shaping choices should be researched, documented, and discussed with the developer before they are treated as final repository policy.

## Pipeline

```text
project config
  + authored source files
      -> parser frontend
      -> AST
      -> semantic analysis
      -> IR
      -> LVGL C backend
      -> generated .c/.h output
```

## Phase-to-Architecture Map

Not every layer should mature at the same time. The recommended sequence is:

### Phase 0: Foundation

- `compiler/` owns config, diagnostics, and discovery
- `parser/` owns tokenisation and AST scaffolding
- `semantic/`, `ir/`, and `backend/` define their contracts
- `cli/` exposes command shape

### Phase 1: Language Freeze and Parsing

- `parser/` becomes the main focus
- `compiler/` continues to support diagnostics and project loading
- downstream layers should remain stable consumers, not moving targets

### Phase 2: Semantic Analysis and IR

- `semantic/` and `ir/` become the main focus
- parser output should already be stable enough to lower from
- backend work should still follow documented mapping constraints

### Phase 3: Backend Integration

- `backend/lvgl_c/` and `cli/` become the main focus
- semantic output should already be canonical enough that the backend does not guess

### Phase 4 and Later

- examples, regression coverage, preview orchestration, and supported-surface expansion

This sequencing matters. If a task forces simultaneous redesign of parser, semantic, IR, and backend layers, the language slice is probably still too large.

## Repository Layers

### `compiler/`

Shared compiler infrastructure.

- config loading
- source discovery
- diagnostics primitives
- stage-level instrumentation primitives shared across commands
- project-level utility types

This crate deliberately avoids depending on higher compiler stages so it can serve as the base layer for the workspace.

It owns shared diagnostic and instrumentation contracts, but it does not own parser rules, semantic policy, IR design, backend mapping, or CLI presentation decisions.

### `parser/`

Frontend data structures and tokenisation utilities.

- AST definitions
- source kind classification
- lexer
- future grammar parser

The parser crate is allowed to know syntax, but not LVGL backend details.

It owns syntax acceptance, source spans, and syntax-level failure reporting. It must not absorb semantic normalization, backend mapping shortcuts, or command-orchestration concerns.

### `semantic/`

Typed validation and meaning resolution.

- duplicate identity checks
- rule validation
- property typing
- style application
- binding/event reference validation
- lowering from AST to IR

The semantic layer should reject unsupported constructs before code generation sees them.

It owns meaning, validation policy, and normalization. It must not format CLI output, guess backend APIs, or re-interpret syntax that should already be settled in `parser/`.

### `ir/`

Backend-facing intermediate representation.

- screen model
- widget tree
- layout intent
- style surface
- event/binding metadata

The IR must be:

- backend-oriented
- stable
- explicit
- free of syntax-specific quirks

The IR owns canonical compiler-facing data shape only. It must not contain code generation policy, filesystem concerns, or user-facing logging rules.

### `backend/lvgl_c/`

Deterministic LVGL C generation.

- generated file planning
- stable naming
- widget constructor mapping
- readable C output

The backend should never need to infer browser-like behavior. It should only consume already-resolved IR.

It owns LVGL 9.x emission decisions and generated-file ownership boundaries. It must not perform semantic recovery, widen unsupported language features, or define CLI-facing progress messaging by itself.

### `cli/`

User-facing entrypoint.

- command parsing
- invoking compiler stages
- reporting diagnostics
- writing generated output later

The CLI owns operator-facing command flow, stage orchestration, and the final presentation of diagnostics and logs. It should not become a second home for parser, semantic, IR, or backend business logic.

## Stage Ownership Rules

Keep these boundaries explicit as the repository grows:

- `compiler/` provides shared infrastructure contracts consumed by stages; it does not decide language semantics.
- `parser/` decides whether authored text is syntactically valid; it does not decide whether the construct is supported in the MVP.
- `semantic/` decides whether parsed constructs are valid and how they normalize; it does not decide emitted LVGL syntax.
- `ir/` records canonical intent; it does not perform validation recovery or code emission.
- `backend/lvgl_c/` maps canonical intent to LVGL C; it does not patch over unresolved upstream ambiguity.
- `cli/` orchestrates stages and exposes operator behavior; it does not own compiler-stage policy.

If a change makes ownership unclear across more than one adjacent stage, reduce the slice or introduce a clearer contract before continuing.

When the contract itself is in question, the expected workflow is:

- prepare supporting material that captures options, pros/cons, relevant practices, risks, and recent implementation developments
- review that material with the developer
- defer final architectural commitment until the developer signs off

## Determinism Rules

Every compiler stage must preserve deterministic behavior.

- Sort filesystem discovery results.
- Preserve source order where it is semantically meaningful.
- Avoid hash-map iteration order in emitted output.
- Use stable name derivation for generated symbols.
- Keep generated formatting fixed.

## Diagnostics

Diagnostics are a first-class cross-cutting concern.

Each diagnostic should include:

- severity
- message
- optional source file
- optional line/column span
- optional hint

The first pass includes a shared diagnostic type in `compiler/` so all stages can report issues consistently.

## Observability

Observability is a product concern for operator-facing commands, not incidental debug output.

The repository should follow these rules:

- logging is stage-scoped and deterministic
- diagnostics remain stable, actionable, and separate from progress logs
- generated `.c` and `.h` output never contains command logging noise
- verbose tracing, if added later, must layer on top of the same stage boundaries instead of bypassing them

Ownership expectations:

- `compiler/` may define shared instrumentation types or hooks
- stage crates may emit structured stage events or messages at their boundaries
- `cli/` decides how those events become user-visible command output

This keeps observability useful without turning every crate into its own logging frontend.

Observability conventions that materially affect command behavior should follow the same sign-off path as other architecture-shaping decisions.

## Source and Output Separation

The authored project tree and generated output must stay separate.

Recommended layout:

```text
project/
  lumaui.toml
  ui/
    screens/
    styles/
  generated/
    ui/
```

Generated files are never the source of truth.

## First-Pass Implementation Boundary

This repository pass intentionally implements:

- workspace and crate boundaries
- config loading
- source discovery
- diagnostics formatting
- tokenisation scaffolding
- IR definitions
- backend scaffolding

It intentionally defers:

- final grammar freeze
- full parser implementation
- full semantic lowering
- on-disk C emission workflow
- preview runtime integration

## Phase Gates

Each implementation phase should pass a gate before the next one expands scope.

### Gate A: Language Gate

Before broad parser work:

- supported syntax is documented
- unsupported syntax is documented
- example fixtures do not over-promise extra features
- the supporting research and discussion material has been reviewed and signed off by the developer

### Gate B: Parser Gate

Before broad semantic work:

- the MVP subset parses into a real AST
- syntax errors produce useful diagnostics
- parser-stage observability is intentional where exposed through commands
- parser tests cover invalid inputs

### Gate C: Semantic Gate

Before broad backend work:

- the MVP subset lowers into canonical IR
- duplicate ids and unsupported properties are rejected
- semantic normalization rules are explicit
- validation-stage observability is intentional where exposed through commands
- any shared contract changes have been reviewed and signed off by the developer

### Gate D: Backend Gate

Before preview or broader feature work:

- one example goes from source to generated C
- symbol naming is stable
- generated output is protected by exact backend snapshots, and the real frontend build path is checked against those same contracts where project-level naming differences are normalized intentionally
- build-stage observability is intentional and separate from generated artifacts
- backend ownership-boundary and emission-policy decisions have been reviewed and signed off by the developer

## Extensibility

The architecture is designed so later work can add:

- richer semantic validation
- asset pipeline support
- theme token resolution
- alternate IR exporters
- simulator preview orchestration
- version-aware LVGL backend adapters

## Recommended Narrow Slice

The preferred first end-to-end slice is:

- one `Screen`
- one layout container, ideally `Column`
- one `Text`
- one `Button`
- class and id support
- a tiny supported style subset

That slice is large enough to exercise the pipeline and small enough to keep decisions reversible.
