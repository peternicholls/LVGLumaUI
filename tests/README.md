# Tests

This directory stores shared fixtures and snapshots used by crate-local tests.

- `fixtures/` contains shared authored-source examples and targeted failure cases used across parser, semantic, and CLI regressions.
- Normative coverage should use authored sources that stay inside the currently ratified first slice and are expected to validate or build successfully.
- Expected-fail coverage should use narrowly scoped fixtures that isolate one rejected construct, such as unsupported widgets, unsupported selectors, unsupported properties, duplicate ids, or binding syntax.
- Aspirational coverage may document broader future-facing authored sources, but those fixtures must be labeled clearly and must not be treated as passing validation or snapshot inputs for the active slice.
- `snapshots/` contains expected generated C output for deterministic backend tests driven by normative fixtures only.
