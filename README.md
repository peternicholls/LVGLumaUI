# Luma UI for LVGL

Luma UI for LVGL, shortened to LumaUI, is an open-source compiler project for authoring LVGL user interfaces in a declarative, text-first format and lowering them to deterministic, readable C.

This repository is the first disciplined pass of the project. It establishes the product framing, compiler architecture, crate boundaries, examples, diagnostics approach, and LVGL backend shape without prematurely freezing the authored language grammar.

It also treats clean stage ownership and operator-visible observability as part of the product design. Compiler stages should stay narrow, and command output should make pipeline progress and failures understandable without leaking logging noise into diagnostics or generated files.

Major language, architecture, mapping, and observability decisions are also handled through an explicit sign-off workflow. The agent should produce supporting research and discussion material, but repository-shaping decisions are not treated as final until the developer reviews and approves them.

## Current Phase

The project is currently in `Phase 0: Foundation`.

Phase 0 means:

- the repository shape is in place
- the compiler pipeline boundaries are defined
- examples exist as provisional fixtures
- the authored language is not frozen yet
- end-to-end compilation is not complete yet

## Why Rust

The implementation is scaffolded in Rust for three reasons:

1. A compiler pipeline benefits from strong typing across AST, semantic, and IR layers.
2. Deterministic code generation is easier to enforce with explicit data models and predictable iteration.
3. The resulting tool can stay small, portable, and CI-friendly without requiring a managed runtime.

## Project Positioning

LumaUI is:

- compile-time first
- web-inspired
- LVGL-targeted
- deterministic
- version-control friendly

LumaUI is not:

- a browser engine
- a DOM runtime
- full HTML
- full CSS
- a JavaScript host
- a clone of any proprietary LVGL tool

## Current Status

This repo intentionally prioritizes foundation over breadth.

- The documentation pack is in place.
- The Rust workspace is scaffolded by compiler stage.
- The CLI surface is defined.
- Examples exist as provisional authoring fixtures.
- Lexer, diagnostics, config loading, source discovery, IR types, and LVGL C generation scaffolds are implemented.
- Full authored-language parsing and semantic lowering are explicitly deferred to the next phase.

## Workspace Layout

```text
.
├── AGENTS.md
├── docs/
│   ├── ARCHITECTURE.md
│   ├── DECISION_BRIEF_TEMPLATE.md
│   ├── DOCUMENTATION_SCHEME.md
│   ├── DOCUMENT_RECLASSIFICATION_AUDIT.md
│   ├── LANGUAGE_SPEC.md
│   ├── LVGL_MAPPING.md
│   ├── NEXT_STEPS.md
│   ├── PRD.md
│   ├── TASKS.md
│   └── archive/
├── cli/
├── compiler/
├── parser/
├── semantic/
├── ir/
├── backend/lvgl_c/
├── examples/
└── tests/
```

## Compiler Shape

The intended production pipeline is:

```text
authored source -> AST -> semantic analysis -> IR -> LVGL C backend
```

The first repo pass includes real scaffolding for each stage, but does not claim the parser or language are feature complete.

Stage ownership is deliberate:

- `compiler/` owns shared config, discovery, diagnostics, and instrumentation contracts
- `parser/` owns syntax and source-span-aware parse failures
- `semantic/` owns supported-surface validation and normalization
- `ir/` owns the canonical backend-facing model
- `backend/lvgl_c/` owns LVGL emission and generated-file ownership boundaries
- `cli/` owns operator-facing orchestration, diagnostics presentation, and command logging

If a feature forces multiple adjacent stages to guess each other's responsibilities, the language slice is too broad or the contracts are not explicit enough yet.

When the contracts themselves are under discussion, the expected workflow is:

- prepare decision material with options, pros and cons, relevant practices, risks, and open questions
- review that material with the developer
- defer the final call until the developer signs off

## Provisional Authoring Files

The example `.lui` and `.lus` files in `examples/` are working placeholders for fixture development. They are illustrative, not normative. The project has not yet frozen the final grammar for markup or styles.

That distinction is deliberate: this repository pass is about building the compiler shape safely before hardening language syntax.

## Quick Start

With a Rust toolchain available:

```bash
cargo run -p lumaui-cli -- doctor
cargo run -p lumaui-cli -- validate examples/minimal
cargo test
```

`lumaui build` is wired as a compiler-stage command surface, but the source-language frontend is still under active implementation.

Operator-facing commands are expected to remain deterministic and reviewable. Diagnostics should stay stable and actionable, while progress logging should remain stage-scoped and separate from generated output.

For the standard repository verification bundle during the current brownfield slice:

```bash
./scripts/lumaui-phase-check.sh
```

That script runs the workspace tests plus `lumaui doctor` and `lumaui validate` against `examples/minimal`. It treats `lumaui build` as gated by default and only requires it with `--require-build`.

## First-Pass Goals

- keep repo boundaries crisp
- make future language work easier, not harder
- keep LVGL mappings conservative and explicit
- keep stage ownership easy to audit
- keep command observability useful without noisy output
- ensure examples, docs, tests, and code tell the same story

## Phase Roadmap

1. `Phase 0: Foundation`
   Repo shape, docs, crate boundaries, examples, diagnostics scaffolding.
2. `Phase 1: Language Freeze and Parsing`
   Ratify the MVP grammar and implement real parsing for markup and styles.
3. `Phase 2: Semantic Analysis and IR`
   Validate authored input, normalize supported properties, and lower to canonical IR.
4. `Phase 3: LVGL C Backend`
   Generate deterministic, readable C for the MVP widget and style surface.
5. `Phase 4: Fixtures and Stability`
   Turn examples into trusted golden fixtures and expand regression coverage.
6. `Phase 5: Preview`
   Add optional LVGL SDL preview without compromising compiler-first design.

## Immediate Next Steps

The highest-priority work is:

1. ratify the narrowest possible MVP grammar
2. implement the first real parser for `examples/minimal`
3. define semantic normalization for ids, widgets, and the supported property subset
4. complete one honest end-to-end path from source to generated C

See `NEXT_STEPS.md` for the operational checklist and `TASKS.md` for phased acceptance criteria.
See `docs/DOCUMENTATION_SCHEME.md` for the canonical filing and housekeeping rules for repository and feature documents.
See `docs/VERSIONING.md` for the repository versioning and tagging policy.
See `docs/LANGUAGE_SPEC.md` and `docs/LANGUAGE_CHANGELOG.md` for the authored-language contract and its separate revision history.
See `CHANGELOG.md` for release-facing change history.
See `docs/DECISION_BRIEF_TEMPLATE.md` for the standard decision brief format used for research and sign-off discussions.
See `AGENTS.md` for the canonical repository-local agent instructions.

## License

MIT. See `LICENSE`.
