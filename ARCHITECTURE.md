# Architecture

## Purpose

LumaUI is structured as a staged compiler, not a monolith. Each stage owns a distinct transformation so that syntax, meaning, and backend mapping remain understandable and testable.

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

## Repository Layers

### `compiler/`

Shared compiler infrastructure.

- config loading
- source discovery
- diagnostics primitives
- project-level utility types

This crate deliberately avoids depending on higher compiler stages so it can serve as the base layer for the workspace.

### `parser/`

Frontend data structures and tokenisation utilities.

- AST definitions
- source kind classification
- lexer
- future grammar parser

The parser crate is allowed to know syntax, but not LVGL backend details.

### `semantic/`

Typed validation and meaning resolution.

- duplicate identity checks
- rule validation
- property typing
- style application
- binding/event reference validation
- lowering from AST to IR

The semantic layer should reject unsupported constructs before code generation sees them.

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

### `backend/lvgl_c/`

Deterministic LVGL C generation.

- generated file planning
- stable naming
- widget constructor mapping
- readable C output

The backend should never need to infer browser-like behavior. It should only consume already-resolved IR.

### `cli/`

User-facing entrypoint.

- command parsing
- invoking compiler stages
- reporting diagnostics
- writing generated output later

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

## Extensibility

The architecture is designed so later work can add:

- richer semantic validation
- asset pipeline support
- theme token resolution
- alternate IR exporters
- simulator preview orchestration
- version-aware LVGL backend adapters
