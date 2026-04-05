# Language Specification v1.0

## Status

This document is intentionally provisional.

The first repository pass does not freeze the LumaUI authored language. Instead, it defines the boundaries that the future language must satisfy so implementation work can begin without committing too early to a concrete grammar.

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

## Language Exit Gate for the Next Phase

The next phase should be considered complete when:

- the supported syntax is explicitly documented
- unsupported syntax is explicitly documented
- `examples/minimal` is fully expressible within the ratified subset
- parser implementation work can proceed without guessing intended language behavior
