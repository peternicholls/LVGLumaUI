# Tasks

## M0 Repo Setup

### Goal

Establish a coherent repository, documentation pack, workspace layout, and examples.

### Acceptance Criteria

- Root docs exist and agree on scope.
- Rust workspace is scaffolded.
- Compiler stages have dedicated crates or modules.
- Examples exist for minimal and dashboard-shaped projects.
- AGENTS guidance is present.
- Tests and snapshot directories are present.

## M1 Parser and Syntax Validation

### Goal

Turn provisional frontend scaffolding into a real, testable parser.

### Acceptance Criteria

- Language grammar is explicitly ratified before broad parser expansion.
- Markup lexer and parser support the MVP screen tree subset.
- Style lexer and parser support the MVP declaration subset.
- Diagnostics report source spans and actionable messages.
- Parser unit tests cover success and failure cases.

## M2 Semantic Analysis and IR

### Goal

Resolve authored sources into a backend-ready, typed model.

### Acceptance Criteria

- Duplicate ids are detected.
- Unknown widgets and properties are rejected cleanly.
- Style rules are validated and normalized.
- Event and binding references are represented explicitly.
- Semantic output lowers into the canonical IR.
- Semantic validation tests cover key failures.

## M3 LVGL C Backend

### Goal

Generate deterministic, readable LVGL C from IR.

### Acceptance Criteria

- Screen files generate stable `.c` and `.h` output.
- Symbol naming is deterministic.
- Supported widgets map to documented LVGL APIs.
- Generated code is readable and sensibly structured.
- Snapshot tests cover representative screens.

## M4 Examples and Tests

### Goal

Make examples and tests function as trusted golden fixtures.

### Acceptance Criteria

- `examples/minimal` validates end-to-end.
- `examples/dashboard` validates end-to-end.
- Examples are used in automated tests where practical.
- Snapshot coverage exists for generated C.
- Regression tests capture known parser and semantic edge cases.

## M5 Preview Integration

### Goal

Add a preview path without compromising compiler-first design.

### Acceptance Criteria

- `lumaui preview` launches through an LVGL SDL-oriented workflow.
- Preview uses generated artifacts rather than a separate runtime model.
- Failure states are clear when LVGL/SDL prerequisites are missing.
- Preview remains optional and does not distort the language surface.
