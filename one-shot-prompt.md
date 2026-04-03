# One-shot prompt for Codex

This is written to suit Codex’s current best-practice pattern: give it a concrete mission, explicit constraints, exact deliverables, repo structure, verification steps, and a defined completion bar.


You are building a new open-source project called LumaUI.

Mission:
Create a clean-room, declarative, web-inspired UI compiler for LVGL that lets users author embedded UIs in a simple markup-and-style language, then compile them to deterministic, readable C code targeting LVGL.

Important framing:
This is NOT a browser engine.
This is NOT full HTML/CSS.
This is NOT a clone of any proprietary LVGL commercial tool.
It is an original project with familiar ergonomics for web designers and strong constraints suitable for embedded systems.

Product goals:
1. Declarative UI source files for LVGL
2. CSS-like styling with a deliberately limited, embedded-safe property set
3. Deterministic C code generation
4. Readable generated code
5. Strong validation and diagnostics
6. Version-control-friendly text workflow
7. Preview path via LVGL SDL simulator later

Non-goals:
- Full HTML support
- Full CSS support
- JavaScript runtime
- DOM emulation
- WYSIWYG editor in v1
- Dynamic in-device template evaluation

Name:
LumaUI

Initial repo deliverables:
Create the following files and directories with real, thoughtful content:

README.md
PRD.md
ARCHITECTURE.md
LANGUAGE_SPEC.md
LVGL_MAPPING.md
AGENTS.md
TASKS.md
LICENSE
/cli
/compiler
/parser
/semantic
/ir
/backend/lvgl_c
/examples/minimal
/examples/dashboard
/tests

Technical requirements:
- Build the project as a compiler pipeline:
  source files -> AST -> semantic analysis -> IR -> LVGL C backend
- Keep the architecture modular
- Make code generation deterministic
- Generated C must be readable and sensibly named
- Do not hand-wave parser details; define a real syntax
- Do not overreach with CSS; keep selectors and properties intentionally small
- Design for extensibility but implement only a careful MVP
- Include a project config format
- Include diagnostics design
- Include testing strategy
- Include clean separation between authored files and generated output

Language design requirements:
Define:
- a markup language for widget trees
- a style language inspired by CSS
- ids
- classes
- simple event handler references
- simple binding references
- layout primitives that map cleanly to LVGL

Support these core widgets in the spec:
- Screen
- Container
- Row
- Column
- Grid
- Text
- Button
- Image
- Card

Support these concepts in the spec:
- width/height
- padding
- margin subset
- background color
- text color
- radius
- border width/color
- font reference
- row/column/grid layout
- align/justify subset

Avoid browser semantics that do not map cleanly to LVGL.

Implementation expectations:
Phase 1:
- write the documentation pack first
- define the syntax clearly
- define the IR
- define LVGL mapping tables
- define the CLI commands

Phase 2:
- scaffold a compiler codebase
- implement parser skeletons
- implement AST types
- implement semantic pass skeletons
- implement IR types
- implement backend skeletons
- add tests and example fixtures

CLI commands to support in design:
- lumaui init
- lumaui validate
- lumaui build
- lumaui preview
- lumaui doctor

Code quality rules:
- choose one implementation language and justify it in README
- keep code clean, minimal, and modular
- write docstrings/comments where useful but do not over-comment
- avoid placeholder fluff
- avoid fake completeness
- where something is not implemented yet, mark it clearly as planned
- do not invent unsupported LVGL APIs
- if unsure about a mapping, isolate it as a TODO with rationale

AGENTS.md requirements:
Create a high-quality repo-local agent instruction file that tells coding agents:
- project intent
- architectural boundaries
- non-goals
- determinism expectations
- how to approach tasks
- how to add fixtures and tests
- how to avoid scope creep

TASKS.md requirements:
Break the project into practical milestones with acceptance criteria:
- M0 repo setup
- M1 parser + syntax validation
- M2 semantic analysis + IR
- M3 LVGL C backend
- M4 examples + tests
- M5 preview integration

Testing requirements:
- parser unit tests
- semantic validation tests
- snapshot tests for generated C
- examples that serve as golden fixtures

What to do right now:
1. Create the initial documentation pack
2. Scaffold the repo structure
3. Add the first example source files
4. Add initial compiler skeleton code
5. Add AGENTS.md with precise instructions for future agent runs
6. Add TASKS.md with milestone breakdown
7. Make the repo coherent and internally consistent

Output expectations:
- Produce actual file contents, not just summaries
- Keep the project realistic and disciplined
- Prefer a narrow but solid foundation over breadth
- Make the result feel like a serious open-source compiler project
```

# Recommended `AGENTS.md` starter

Since Codex benefits from repo-local operating instructions and explicit workflows, add something like this near the start of the repo: ([OpenAI Developers][2])

```md
# AGENTS.md

## Project intent
LumaUI is a declarative UI compiler for LVGL. It is web-inspired, but not a browser and not a full HTML/CSS implementation.

## Core rules
- Preserve compile-time-first architecture
- Do not add browser semantics that do not map cleanly to LVGL
- Prefer deterministic code generation over cleverness
- Keep generated C readable and stable
- Treat documentation and examples as first-class artifacts
- Avoid proprietary format imitation

## Implementation priorities
1. Parser correctness
2. Semantic clarity
3. IR cleanliness
4. LVGL backend determinism
5. Tests and fixtures
6. Preview tooling later

## Scope control
Reject or defer:
- JavaScript runtime ideas
- full CSS selector logic
- dynamic DOM-like mutation models
- editor-first features before compiler maturity

## When adding features
- update LANGUAGE_SPEC.md
- update LVGL_MAPPING.md
- add or update fixture examples
- add tests
- document limitations clearly
