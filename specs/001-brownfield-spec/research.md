# Research: Brownfield MVP Compiler Slice

## Decision 1: Use `examples/minimal` as the only normative first-slice fixture

**Decision**: Treat `examples/minimal` as the sole passing fixture for the first ratified slice and treat broader examples such as `examples/dashboard` as aspirational until the supported surface expands.

**Rationale**: The docs consistently prefer one honest end-to-end path over broad partial support. `examples/minimal` already exists, fits the recommended narrow widget set, and provides a concrete acceptance target for parser, semantic, backend, and snapshot work.

**Alternatives considered**:

- Make both `minimal` and `dashboard` normative immediately. Rejected because that widens the surface before the language is ratified.
- Create a new synthetic fixture instead of using an existing example. Rejected because the repository already points to `examples/minimal` as the intended first thin slice.

## Decision 2: Ratify an XML-like markup subset aligned with current fixtures and starter files

**Decision**: Keep the first ratified markup grammar close to the existing XML-like authored shape used by the starter project and `examples/minimal`.

**Rationale**: The current examples, starter template, and language sketches already use this shape. Ratifying that narrow subset avoids inventing a second syntax before parser completion and keeps tokenization and parsing deterministic.

**Alternatives considered**:

- Replace the current shape with a compact custom DSL before parser work. Rejected because it adds language-design churn without improving the first end-to-end slice.
- Keep the grammar fully provisional while implementing parsing. Rejected because the docs explicitly require the language to be ratified before broad parser expansion.

## Decision 3: Limit the first supported widget set to `Screen`, `Column`, `Row`, `Text`, and `Button`

**Decision**: The first slice supports only `Screen`, `Column`, `Row`, `Text`, and `Button`.

**Rationale**: This matches the recommended narrow slice in the architecture and next-step docs while still exercising root creation, layout containers, leaf text, and an interactive control. It is large enough to prove the pipeline and small enough to keep decisions reversible.

**Alternatives considered**:

- Include `Container`, `Grid`, `Image`, or `Card` in the first slice. Rejected because these either broaden the LVGL mapping surface or pull in unresolved asset and layout questions.
- Reduce the slice to only `Screen`, `Column`, and `Text`. Rejected because it would not exercise enough of the intended backend and event surface.

## Decision 4: Limit the first selector and style-property surface to class/id selectors plus a tiny property set

**Decision**: Support class selectors and id selectors only, with the first property subset limited to `padding`, `background-color`, `text-color`, `width`, and `height`.

**Rationale**: This keeps style parsing and semantic normalization intentionally small while covering the current minimal fixture and the property families already called out as preferred early support. It also stays within explicit LVGL mappings.

**Alternatives considered**:

- Support type selectors, descendant selectors, or cascade-heavy CSS semantics. Rejected because the docs explicitly defer complex selector semantics.
- Include margin shorthands, radius, borders, or fonts immediately. Rejected because they add normalization and backend complexity before the first path is proven.

## Decision 5: Allow named event handler references and explicitly reject bindings in the first slice

**Decision**: The first slice allows named event handler references on supported widgets and explicitly rejects bindings as out of scope.

**Rationale**: The feature spec requires named event handler references in the first slice and defers bindings. This preserves a useful interactive hook without implying a reactive runtime or hidden expression engine.

**Alternatives considered**:

- Defer both events and bindings. Rejected because the user clarification explicitly includes event handler references in the first slice.
- Allow symbolic bindings now. Rejected because the docs keep bindings as a later-phase concern and because binding rules would widen semantic scope immediately.

## Decision 6: Keep parser work isolated from semantic and backend policy

**Decision**: Phase 1 implementation focuses on real parsing, AST construction, and syntax diagnostics; semantic meaning and backend generation remain downstream gates.

**Rationale**: The architecture and constitution require stage isolation. Parser completion should produce stable syntax trees and clear diagnostics, not embed semantic rules or backend-driven shortcuts.

**Alternatives considered**:

- Add semantic defaults or backend-friendly normalization directly during parsing. Rejected because it couples stages and makes later behavior harder to reason about.
- Delay diagnostics until semantic analysis. Rejected because parser gate exit criteria require actionable syntax diagnostics with spans.

## Decision 7: Use semantic analysis to enforce the supported surface and lower to canonical IR

**Decision**: The semantic layer owns duplicate-id checks, supported widget validation, supported property validation, named event-reference validation, binding rejection, and lowering to canonical IR.

**Rationale**: Those responsibilities match the architecture doc directly. They also keep parser output syntax-facing and backend input backend-facing.

**Alternatives considered**:

- Push supported-surface checks into the CLI. Rejected because that spreads compiler behavior into orchestration code.
- Let the backend infer unsupported or missing semantics. Rejected because the constitution prohibits backend cleverness that compensates for unresolved semantics.

## Decision 8: Keep backend work limited to explicit LVGL 9.x mappings for the ratified subset

**Decision**: Backend implementation for the first slice maps only the ratified widgets and style properties to the documented LVGL 9.x APIs.

**Rationale**: The mapping document already identifies concrete LVGL primitives for the relevant widgets and style property families. Staying inside that mapped subset preserves auditability and determinism.

**Alternatives considered**:

- Implement broader widget families now because some backend scaffolding already exists. Rejected because the language and semantic contracts for them are not yet ratified.
- Add LVGL 8.x compatibility early. Rejected because the repo explicitly targets LVGL 9.x first.

## Decision 9: Use deterministic repository behavior as a non-negotiable acceptance requirement

**Decision**: Preserve sorted source discovery, stable diagnostics ordering, stable emitted file names and symbol names, and readable generated C formatting across the slice.

**Rationale**: Determinism is one of the project’s core principles and is required for reviewable snapshots, trustworthy diffs, and repeatable CI behavior.

**Alternatives considered**:

- Optimize for faster iteration by accepting unstable ordering temporarily. Rejected because it would weaken the exact feedback loops the repo is designed around.
- Defer snapshot-readability work until after backend completion. Rejected because backend gate acceptance criteria already require readable, stable output.