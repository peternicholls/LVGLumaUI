# Constitution

## Status

This is the authoritative governing document for the LVGLuamUI repository.

The Speckit execution copy lives at `.specify/memory/constitution.md`.
Amendments MUST update both files in the same change so planning and execution
rules stay aligned.

When any other document contradicts this one, this document wins.

---

## Identity

### Repository

**LVGLuamUI** is the repository name and the project identity.

### Product

**LumaUI** is the name of the authored language and compiler toolchain built inside this repository.
The CLI binary, the authored file format, and user-facing documentation use the name **LumaUI**.

These are two distinct names for distinct things. Do not conflate them.

---

## Mission

LumaUI gives embedded teams a declarative, text-first authoring model for LVGL UIs.

It is a compiler. It produces deterministic, human-readable LVGL C from authored source files.

It is not a runtime engine, a browser, a DOM implementation, or a UI framework.

---

## Non-Negotiable Constraints

These cannot be relaxed without a documented decision that amends this constitution.

1. **Compile-time only production path.**
   The path from authored source to firmware is always: parse → semantic → IR → generated C. No interpretation at device runtime.

2. **LVGL 9.x is the committed minimum baseline.**
   The backend targets LVGL 9.x APIs. LVGL 8.x compatibility is not in scope until explicitly ratified.

3. **Generated output is deterministic.**
   The same input and compiler version must produce byte-stable output. Hash-indeterminate ordering is forbidden in generated artifacts.

4. **Compiler stages are isolated layers.**
   `parser/`, `semantic/`, `ir/`, and `backend/lvgl_c/` are distinct crates with distinct contracts.
   Cross-layer coupling is prohibited. No stage may import an adjacent stage's internals.

5. **Every supported language construct maps to a named LVGL primitive.**
   If a concept does not map cleanly and explicitly to an LVGL 9.x construct, it is deferred.
   Ambiguous mappings stay as TODOs in `LVGL_MAPPING.md` until resolved.

6. **The authored language grammar is not frozen until explicitly ratified.**
   Per `LANGUAGE_SPEC.md`, grammar decisions must be ratified phase-by-phase.
   Do not build parser or backend features against a grammar that has not been ratified.

---

## Design Principles

Listed in priority order. When principles conflict, higher-ranked principles win.

### 1. Embedded developer ergonomics come first.

When a tradeoff exists between familiar web semantics and what is sound for embedded firmware workflows,
the embedded-developer perspective takes precedence.

Implications:
- No runtime overhead that firmware builds cannot budget for.
- Generated C must be understandable by a developer reading LVGL documentation.
- Diagnostics must identify source locations and actionable remediation steps.

### 2. Narrow and correct beats broad and approximate.

A smaller coherent language is strictly better than a larger unstable one.

Implications:
- Reject language features during implementation if they force unratified grammar decisions.
- Prefer deferring a feature to documenting it as a constraint.
- The MVP widget set and property set must be ratified before expansion begins.

### 3. Generated code is readable and auditable.

Generated C files are a product deliverable, not an internal artifact.

Implications:
- Symbol names must be predictable and human-readable.
- No unexplained magic in generated code.
- Teams must be able to read generated output alongside LVGL documentation without a decoder.

### 4. Text is the source of truth.

All inputs, outputs, diagnostics, and diffs must be meaningful in version control and CI.

Implications:
- No binary or opaque intermediate formats in the committed pipeline.
- Snapshot tests lock generated output and must be kept readable.
- Diagnostics must produce stable, parseable messages.

### 5. Escape hatches are explicit, not implicit.

Where generated C cannot represent a required idiom, the system provides a named escape hatch.

Implications:
- Escape-hatch sections in generated files are marked and owned by the user. They survive regeneration.
- The regenerated sections around them are always overwritten.
- Escape-hatch boundaries must be clearly delimited in generated output.

---

## Generated Code Policy

Generated C output follows a **hybrid ownership model**:

- The compiler owns all non-escape-hatch sections. These are overwritten on every compile run.
- The user owns escape-hatch sections. These are preserved across compile runs.
- Escape-hatch boundaries are delimited by markers defined by the backend.
- Generated files must not be manually edited outside of escape-hatch sections.

This model allows auditable, reviewable diffs while providing a safety valve for constructs the compiler does not yet support.

---

## Scope Boundary

### Hard out-of-scope (not deferrable, not negotiable)

- Full HTML or full CSS semantics.
- Browser DOM emulation or mutation model.
- JavaScript runtime, in-device scripting, or dynamic template evaluation.
- Pixel-perfect parity with any browser rendering engine.
- WYSIWYG editor tooling in v1.
- CSS cascade or specificity model.
- Runtime binding evaluation on device.

### Deferred (explicitly out of scope until ratified)

- Final markup grammar and exact file extension.
- Final style grammar.
- Event handler and binding reference syntax.
- SDL/simulator preview path.
- LVGL 8.x backend compatibility.
- Grid, Image, Card widget support (deferred until MVP slice is stable).
- Font asset pipeline.
- Component reuse and theme tokens beyond the MVP.

---

## Architectural Law

### Pipeline

```
project config
  + authored source files
      -> parser (AST)
      -> semantic analysis (validation + lowering)
      -> IR
      -> LVGL C backend
      -> generated .c / .h
```

### Crate responsibilities

| Crate | Owns | Must not |
|---|---|---|
| `compiler/` | config, diagnostics, source discovery, project utilities | depend on parser, semantic, ir, or backend |
| `parser/` | AST, lexer, syntax diagnostics | know LVGL backend details |
| `semantic/` | validation, property resolution, lowering to IR | access LVGL APIs directly |
| `ir/` | backend-facing canonical model | contain codegen logic |
| `backend/lvgl_c/` | LVGL 9.x code generation | perform semantic analysis or grammar decisions |
| `cli/` | command orchestration, user-facing diagnostics | contain compiler logic |

### Ordering constraint

Maturation must follow this sequence: `compiler/` → `parser/` → `semantic/` + `ir/` → `backend/lvgl_c/` → `cli/` integration.

A task that forces simultaneous redesign of multiple layers indicates the language slice is too large and must be reduced.

---

## Decision Procedure

When a proposal is unclear, apply these questions in order:

1. Does this map cleanly to a named LVGL 9.x primitive? If not, defer it.
2. Does this require grammar decisions that have not been ratified? If so, document the constraint and stop.
3. Does this relax a non-negotiable constraint? If so, it requires a constitutional amendment, not a code change.
4. Does this serve embedded developers first? If it primarily serves web familiarity at embedded cost, defer or reject.
5. Does this maintain layer isolation? If it couples two stages, restructure or reject.

---

## Amendment Process

Any change to this document must:

1. Explicitly identify which section is being amended and why.
2. Reflect the current state of `TASKS.md` (active phase and exit gate).
3. Be committed alongside the task or decision that required the change.

Ad-hoc amendments to resolve implementation convenience are not permitted.
