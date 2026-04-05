# LVGLuamUI Development Guidelines

Auto-generated from all feature plans. Last updated: 2026-04-05

## Active Technologies

- Rust 2021 workspace on the stable toolchain; authored `.lui` and `.lus` sources remain provisional until ratified + `anyhow`, `clap` with derive, `serde` with derive, `toml`; LVGL 9.x C API family as the backend target (001-brownfield-spec)

## Project Structure

```text
src/
tests/
```

## Commands

cargo test [ONLY COMMANDS FOR ACTIVE TECHNOLOGIES][ONLY COMMANDS FOR ACTIVE TECHNOLOGIES] cargo clippy

## Code Style

Rust 2021 workspace on the stable toolchain; authored `.lui` and `.lus` sources remain provisional until ratified: Follow standard conventions

## Recent Changes

- 001-brownfield-spec: Added Rust 2021 workspace on the stable toolchain; authored `.lui` and `.lus` sources remain provisional until ratified + `anyhow`, `clap` with derive, `serde` with derive, `toml`; LVGL 9.x C API family as the backend target

<!-- MANUAL ADDITIONS START -->
## LumaUI Workflow Guardrails

- Treat `docs/TASKS.md` and `docs/NEXT_STEPS.md` as the active execution order. State the active phase and exit gate before implementation.
- Check `docs/LANGUAGE_SPEC.md` before widening syntax, selectors, widgets, properties, bindings, or events. If the surface is not already ratified, stop at docs or decision work instead of implementing it.
- Keep stage ownership strict: `compiler/` shared infrastructure, `parser/` syntax, `semantic/` meaning and lowering, `ir/` canonical backend-facing model, `backend/lvgl_c/` LVGL C generation, `cli/` orchestration and operator-facing output.
- If the task changes language policy, LVGL mapping policy, generated ownership conventions, diagnostics policy, or shared downstream contracts, write or update a decision brief under `specs/<feature-id>/decisions/` using `docs/DECISION_BRIEF_TEMPLATE.md` before implementation.
- For code-bearing work, add or update the failing test, fixture, snapshot, or CLI assertion first, then keep docs, examples, fixtures, tests, and snapshots aligned in the same change.
- Use `scripts/lumaui-phase-check.sh` for the standard verification bundle. Treat `lumaui build` as gated unless the current slice explicitly requires end-to-end build success.
- Use `.specify/memory/constitution.md` as the governing workflow authority. Treat `docs/archive/CONSTITUTION.md` as archival-only context.
<!-- MANUAL ADDITIONS END -->
