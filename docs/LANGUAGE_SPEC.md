# Language Specification v1.0

This specification governs the authored language for Luma UI for LVGL, shortened to LumaUI.

## Status

This document is intentionally provisional.

The first repository pass does not freeze the LumaUI authored language. Instead, it defines the boundaries that the future language must satisfy so implementation work can begin without committing too early to a concrete grammar.

The language specification also assumes clean stage ownership: syntax belongs to `parser/`, meaning and supported-surface decisions belong to `semantic/`, canonical representation belongs to `ir/`, and LVGL API mapping belongs to `backend/lvgl_c/`. This document describes authored-language intent and boundaries; it does not delegate unresolved language design to downstream code generation.

Language-shaping decisions follow the repository sign-off workflow. The agent may prepare proposals, tradeoff analysis, and supporting research, but ratified language decisions are not considered final until the developer explicitly signs them off.

## Phase Policy

Language design should proceed in explicit phases.

### Current phase

`Phase 0/1 boundary`

Meaning:

- the repository already contains provisional source examples
- the final grammar is not yet ratified
- the next language task is to freeze a narrow MVP subset, not the full aspirational surface

### Rule for the next phase

The next phase should ratify only the syntax needed for the first end-to-end compiler slice.

Before a proposal becomes ratified, the supporting decision material should capture options considered, pros and cons, relevant practices, implementation developments, risks, and open questions for developer review.

Recommended first ratified slice:

- `Screen`
- `Column`
- `Row`
- `Text`
- `Button`
- id support
- class support
- a tiny style declaration subset

Recommended deferrals:

- `Grid`
- `Image`
- `Card`
- complex selectors
- advanced binding syntax
- shorthand-heavy style syntax

## What Is Fixed in This Phase

The following expectations are considered stable:

- LumaUI will have a declarative widget-tree source format.
- LumaUI will have a separate style source format.
- Source files will map cleanly to LVGL concepts rather than browser semantics.
- Identifiers, classes, event references, and binding references will exist.
- Layout concepts will be limited to primitives that LVGL can represent directly.
- Arbitrary scripting expressions are out of scope.
- Operator-facing commands may expose stage-scoped progress and failure information, but authored language behavior must stay distinct from logging and diagnostics presentation.

## What Is Not Fixed Yet

The following are explicitly deferred:

- final markup grammar
- final style grammar
- exact selector syntax
- exact binding-path syntax
- exact event reference syntax
- file extension permanence

## Source Artifact Roles

The repository uses the following working conventions while the grammar is still evolving:

- `.lui` for widget-tree examples
- `.lus` for style examples
- `lumaui.toml` for project configuration

These conventions are useful for fixtures and tooling, but they are not yet language guarantees.

## Required Language Capabilities

The eventual v1 language must support:

- screen declarations
- nested widget trees
- ids
- classes
- named event handler references
- named binding references
- reusable style rules
- width and height
- padding
- margin subset
- background color
- text color
- radius
- border width and color
- font reference
- row, column, and grid layout
- align and justify subset

## Required Core Widgets

The eventual v1 language must cover:

- Screen
- Container
- Row
- Column
- Grid
- Text
- Button
- Image
- Card

## Semantic Boundaries

The v1 language must not imply browser behavior that LumaUI cannot or should not emulate.

Specifically, the language should avoid:

- DOM mutation semantics
- full CSS cascade complexity
- deep selector combinators
- pseudo-elements
- layout rules that depend on browser formatting contexts
- runtime expression evaluation in source files

The language must also avoid cross-stage ambiguity. Grammar should not rely on backend inference, and supported-surface rules should not be left implicit for CLI or backend code to reinterpret later.

## Constraints on Future Grammar Design

When the grammar is finalized, it should preserve the following properties:

### Readability

Source should remain easy to author and review in plain text.

### Deterministic Parsing

The grammar should be unambiguous and straightforward to tokenize.

### Embedded Safety

Every supported construct should have a compile-time interpretation.

### LVGL Affinity

The language should describe intent that can lower directly to LVGL APIs.

### Toolability

Diagnostics, formatting, linting, and future editor support should remain feasible.

### Stage Ownership

Grammar decisions should make it obvious which stage owns each responsibility:

- parser decides syntactic validity
- semantic decides supported-surface validity and normalization
- IR records canonical intent without syntax quirks
- backend maps canonical intent to LVGL constructs without guessing upstream meaning

If a proposed language feature weakens those boundaries, it should be reduced or deferred.

### Observability Compatibility

Language design should support clear diagnostics and command observability without making logging part of the authored source model.

This means:

- errors should be attributable to authored files and spans
- success and progress reporting should happen at command/stage boundaries, not inside the language itself
- generated output should remain free of ad hoc tracing text added only to compensate for unclear language contracts

## Illustrative Example Only

The example below shows the intended shape of the authoring experience, but is not a frozen syntax contract:

```xml
<Screen id="home">
  <Column class="root">
    <Text text="Hello"/>
    <Button id="openSettings" onPress="open_settings">
      <Text text="Settings"/>
    </Button>
  </Column>
</Screen>
```

And a matching style sketch:

```css
.root {
  padding: 16;
}
```

These examples are directional. Future work must ratify the exact grammar in a dedicated language-design phase.

## Config Format

The project configuration format is intentionally fixed earlier than the UI language because it affects CLI behavior and repository structure.

Working example:

```toml
project_name = "minimal"
lvgl_version = "9.x"
source_dir = "ui"
output_dir = "generated/ui"
```

## Next-Phase Deliverables

The next language phase should produce:

- a real grammar definition
- parser acceptance tests
- selector and property surface definition
- binding and event reference rules
- source-to-AST examples with diagnostics
- stage ownership that stays explicit from syntax through backend mapping
- command-observability expectations that do not blur language behavior with logging behavior

## Language Exit Gate for the Next Phase

The next phase should be considered complete when:

- the supported syntax is explicitly documented
- unsupported syntax is explicitly documented
- `examples/minimal` is fully expressible within the ratified subset
- parser implementation work can proceed without guessing intended language behavior
- stage ownership remains clear enough that parser, semantic, IR, backend, and CLI work can advance without hidden coupling
- the developer has explicitly signed off on the supporting material for the ratified language slice
