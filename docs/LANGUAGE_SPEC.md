# Language Specification

**Language Spec Version**: `LS-0.2.0`
**Status**: ratified for the first MVP slice; broader language still provisional
**Repository Release Version**: tracked separately in the root workspace version and `CHANGELOG.md`

This specification governs the authored language for Luma UI for LVGL, shortened to LumaUI.

Language-specification versioning is intentionally separate from application and workspace versioning. Language-contract history is tracked in `docs/LANGUAGE_CHANGELOG.md`, while repository release history remains in `CHANGELOG.md`.

## Status

This document now freezes the first end-to-end authored-language slice used by the brownfield MVP compiler path.

The whole aspirational language is not frozen. Later widgets, selectors, properties, bindings, and shorthand forms remain provisional until they are explicitly ratified.

The language specification also assumes clean stage ownership: syntax belongs to `parser/`, meaning and supported-surface decisions belong to `semantic/`, canonical representation belongs to `ir/`, and LVGL API mapping belongs to `backend/lvgl_c/`. This document describes authored-language intent and boundaries; it does not delegate unresolved language design to downstream code generation.

Language-shaping decisions follow the repository sign-off workflow. The agent may prepare proposals, tradeoff analysis, and supporting research, but ratified language decisions are not considered final until the developer explicitly signs them off.

## Versioning Policy

The language specification has its own version line and should not inherit the workspace release version.

Use the `LS-MAJOR.MINOR.PATCH` format for the authored-language contract.

- Increment `MAJOR` for incompatible changes to already-ratified grammar or language-contract meaning.
- Increment `MINOR` for ratified surface expansion, new explicitly supported constructs, or major clarifications that broaden the language without invalidating already-ratified input.
- Increment `PATCH` for editorial clarifications, examples, wording cleanup, and non-semantic corrections that do not change the ratified contract.

While broader language work remains provisional, versions still advance on meaningful contract revisions, but the document status must remain explicit so the version number is not mistaken for implementation completeness.

## Phase Policy

Language design should proceed in explicit phases.

### Current phase

`First MVP slice ratified`

Meaning:

- the repository has a ratified narrow authored-language subset
- parser and semantic work should implement that subset without guessing
- broader language expansion remains a later design task

### Rule for the next phase

The next phase should implement the ratified slice and only widen the language when new design material is prepared and explicitly approved.

Before a proposal becomes ratified, the supporting decision material should capture options considered, pros and cons, relevant practices, implementation developments, risks, and open questions for developer review.

Ratified first slice:

- `Screen`
- `Column`
- `Row`
- `Text`
- `Button`
- `id` support
- `class` support
- `onPress` event references
- a tiny style declaration subset

Deferred beyond this slice:

- `Grid`
- `Image`
- `Card`
- complex selectors
- bindings
- shorthand-heavy style syntax

## Ratified MVP Contract

This section is the accepted language surface for the first end-to-end compiler slice.

Anything not listed here should be treated as deferred or unsupported for MVP.

### Source artifacts

- `.lui` is the ratified source format for widget trees in MVP.
- `.lus` is the ratified source format for styles in MVP.
- `lumaui.toml` remains the project configuration format.

File-extension permanence beyond MVP is still a later language-governance decision, but these extensions are the supported authored formats for the current slice.

### Markup model

- Markup is XML-like.
- One `.lui` file contains exactly one top-level `Screen` element.
- Multiple top-level widgets in a single `.lui` file are invalid.
- `Screen` may contain one or more child widgets.
- Supported widget tags are `Screen`, `Column`, `Row`, `Text`, and `Button` only.
- Widgets with no children may use either explicit open/close tags or self-closing tags.

### Markup attributes

- `id` is optional.
- `id` values are singular identifier tokens and must be unique across the active compiled project.
- `class` is optional.
- `class` is a space-separated list of identifier tokens.
- Identifier tokens use the current MVP identifier grammar: they must start with an ASCII letter or `_`, and may continue with ASCII letters, digits, `_`, or `-`.
- String-valued attributes use quoted string literals.
- `Text` literal content is expressed through the `text` attribute only in MVP.
- Inner-text shorthand such as `<Text>Hello</Text>` is not part of MVP.
- `Button` may contain child widgets.
- The normative MVP button-label pattern is a nested `Text` child.

### Events and bindings

- The only ratified event attribute in MVP is `onPress`.
- Event references use quoted identifier values, for example `onPress="open_settings"`.
- Event references are names only. Embedded code, expressions, argument lists, and inline scripting are not part of MVP.
- Bindings are not part of MVP.
- Any `bind`-style authored input is rejected in semantic validation for this slice.

### Style model

- One style rule contains exactly one selector and a declaration block.
- The only supported selector kinds are `.class` and `#id`.
- Selector identifiers use the same MVP identifier grammar as markup `id` and `class` tokens.
- The only supported style properties are `padding`, `background-color`, `text-color`, `width`, and `height`.
- `background-color` and `text-color` use hex color literals in MVP.
- `padding` accepts an integer scalar value only.
- `width` and `height` accept integer pixel values and percentages.

### Style matching and precedence

- A widget may match zero or more class rules.
- A widget may match at most one id rule.
- Style application uses a simplified specificity model.
- `#id` rules outrank `.class` rules.
- When specificity is equal, later declarations win.
- If the same property appears multiple times within the same rule, the last declaration wins.
- Scalar `padding` is normalized semantically to the four LVGL side paddings.

### Validation boundaries

- The parser owns syntactic structure, tokenization, and source spans.
- The semantic layer owns supported-surface validation and normalization.
- Unsupported but well-formed selectors, properties, and value shapes are not parser errors.
- Unsupported but well-formed selectors, properties, and value shapes are semantic errors.
- The parser must not guess unsupported constructs into supported ones.
- The semantic layer must not reinterpret invalid syntax as a fallback form.

## Explicitly Deferred Beyond MVP

The following language areas are intentionally outside the first ratified slice:

- `Grid`
- `Image`
- `Card`
- `Container` as a separate authored widget distinct from the current layout widgets
- binding syntax and binding lifecycle semantics
- selector combinators and complex selectors
- per-side padding properties
- shorthand forms beyond scalar `padding`
- margin properties
- radius
- border width and border color
- font references
- align and justify properties
- content-sized values such as LVGL content sizing keywords
- additional event names beyond `onPress`
- embedded expressions or scripting
- browser-style cascade features such as layers, importance, pseudo-elements, or deep combinators

## Required Language Capabilities

The broader eventual v1 language is expected to grow beyond the MVP slice. Candidate future capabilities include:

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

The broader eventual v1 language is expected to cover at least:

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

The ratified MVP language must not imply browser behavior that LumaUI cannot or should not emulate.

Specifically, the language should avoid:

- DOM mutation semantics
- full CSS cascade complexity
- deep selector combinators
- pseudo-elements
- layout rules that depend on browser formatting contexts
- runtime expression evaluation in source files

The language must also avoid cross-stage ambiguity. Grammar should not rely on backend inference, and supported-surface rules should not be left implicit for CLI or backend code to reinterpret later.

## Constraints on Future Grammar Design

When the language expands beyond MVP, it should preserve the following properties:

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

## Normative MVP Example

The example below is inside the ratified MVP subset:

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

The following style example is also valid in MVP because percentages are supported for `width` and `height`:

```css
.hero {
  width: 100%;
  height: 50%;
}
```

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

The next implementation phase should produce:

- a real parser for the ratified grammar
- parser acceptance tests
- semantic normalization for the ratified selector and property surface
- event-reference validation for `onPress`
- explicit rejection diagnostics for bindings and other deferred constructs
- source-to-AST examples with diagnostics
- stage ownership that stays explicit from syntax through backend mapping
- command-observability expectations that do not blur language behavior with logging behavior

## Language Exit Gate for the Next Phase

The next phase should be considered complete when:

- the ratified syntax is fully implemented in parser and semantic stages
- unsupported syntax is explicitly rejected with actionable diagnostics
- `examples/minimal` is fully expressible within the ratified subset
- parser implementation work can proceed without guessing intended language behavior
- stage ownership remains clear enough that parser, semantic, IR, backend, and CLI work can advance without hidden coupling
- the developer has explicitly signed off on the supporting material for the ratified language slice
