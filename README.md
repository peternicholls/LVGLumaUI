# LumaUI

LumaUI is an open-source compiler project for authoring LVGL user interfaces in a declarative, text-first format and lowering them to deterministic, readable C.

This repository is the first disciplined pass of the project. It establishes the product framing, compiler architecture, crate boundaries, examples, diagnostics approach, and LVGL backend shape without prematurely freezing the authored language grammar.

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
├── ARCHITECTURE.md
├── LANGUAGE_SPEC.md
├── LVGL_MAPPING.md
├── PRD.md
├── TASKS.md
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

## First-Pass Goals

- keep repo boundaries crisp
- make future language work easier, not harder
- keep LVGL mappings conservative and explicit
- ensure examples, docs, tests, and code tell the same story

## Planned Next Steps

1. Freeze the v1 authored-language grammar.
2. Implement real parsing for screen and style documents.
3. Lower parsed documents into typed semantic models.
4. Expand the LVGL C backend from synthetic IR fixtures to end-to-end builds.
5. Add preview integration through an LVGL SDL flow.

## License

MIT. See `LICENSE`.
