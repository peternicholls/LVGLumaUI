# Product Requirements Document

## Product

LumaUI

## Summary

LumaUI is a declarative UI compiler for LVGL. It gives teams a web-inspired authoring model for embedded UIs while preserving the determinism, portability, and explicitness expected in firmware-adjacent toolchains.

The first repository pass focuses on architecture and implementation discipline rather than language lock-in.

## Problem

LVGL is capable and flexible, but hand-authoring screen hierarchies in C is repetitive, noisy, and hard to review. Teams want a higher-level source format that still produces auditable code and fits text-based engineering workflows.

## Users

- Embedded developers who want maintainable LVGL UI code.
- Frontend-minded developers who want familiar authoring ergonomics without browser complexity.
- Teams that want reviewable UI changes, stable diffs, and automation-friendly tooling.

## Product Goals

1. Declarative UI source files for LVGL.
2. CSS-like styling with a deliberately limited, embedded-safe subset.
3. Deterministic C code generation.
4. Readable generated code that can be audited and debugged.
5. Strong validation and diagnostics.
6. Text-first workflow that behaves well in git.
7. A future preview path through LVGL SDL simulation.

## Non-Goals

- Full HTML support
- Full CSS support
- Browser layout semantics
- JavaScript runtime
- DOM emulation
- Dynamic template execution on-device
- WYSIWYG editor in v1

## Core Principles

### Compile-Time First

The production path is compilation, not interpretation. Runtime magic should be minimized.

### Deterministic Output

Equivalent input must produce stable generated C with predictable naming and ordering.

### Clean LVGL Mapping

Every supported concept should map clearly to LVGL primitives. If it does not map cleanly, it should be deferred.

### Narrow MVP

A smaller, coherent language is preferable to a broad but unstable one.

### Text as the Source of Truth

Source artifacts, generated artifacts, and diagnostics should all be clear in version control and CI.

## MVP Scope

The MVP architecture supports:

- screen-oriented authored sources
- reusable style sources
- a project config file
- diagnostics with source spans
- AST, semantic, and IR layers
- LVGL C code generation

The MVP widget set is expected to cover:

- Screen
- Container
- Row
- Column
- Grid
- Text
- Button
- Image
- Card

## Current First-Pass Scope

This repository pass intentionally stops short of final language design. It provides:

- repo-level docs
- provisional file conventions
- compiler crate boundaries
- CLI command surface
- source discovery
- diagnostics infrastructure
- lexer scaffolding
- semantic and IR skeletons
- LVGL backend scaffolding
- examples and test fixtures

## Command Surface

The CLI is designed around these commands:

- `lumaui init`
- `lumaui validate`
- `lumaui build`
- `lumaui preview`
- `lumaui doctor`

In this first pass:

- `init` can create a starter project layout
- `doctor` verifies basic project expectations
- `validate` performs config loading, source discovery, and provisional frontend checks
- `build` is present but remains gated on the parser and semantic pipeline
- `preview` is intentionally deferred

## Success Criteria

This first pass is successful when:

- the project reads like a serious compiler repo
- the crate structure supports future implementation cleanly
- the docs agree on scope and non-goals
- examples and tests provide realistic fixtures
- later work can proceed without re-litigating project intent
