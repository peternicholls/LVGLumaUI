# Research Brief: Language Open Questions

## Status

Research brief for pre-Phase-2 language ratification.

This document records external research against the open questions listed in `docs/LANGUAGE_REVIEW.md` and turns that research into LumaUI-specific recommendations.

It is intentionally split into two layers:

1. what the external sources actually say
2. what that implies for LumaUI's constrained XML-like MVP

## Method

The source set prioritizes primary or near-primary documentation from:

- W3C XML
- MDN HTML and CSS references
- LVGL 9.x documentation
- established declarative UI systems with documented syntax rules: QML, XAML, and Slint

This research does not assume that LumaUI should copy browser or desktop UI semantics wholesale. The goal is to identify stable precedents, then adopt only the subset that reduces author friction and maps cleanly to LVGL.

## Executive Summary

The strongest externally-supported answers are:

- A `.lui` document should have exactly one top-level `Screen`.
- `class` should be a space-separated list of identifier tokens.
- `id` should be a single identifier token and should be unique across the active compiled project.
- Self-closing tags should be allowed for any widget with no content.
- A widget can match multiple class rules.
- `#id` should outrank `.class`, and later declarations should break ties.
- Unsupported-but-well-formed declarations should be parsed, then rejected during semantic validation.

The two most important adjustments from the current review draft are:

- Research does not support requiring `Screen` to have exactly one child widget. One document root is standard; one child under that root is not.
- LVGL 9.x does support percentage width and height cleanly, so percentages are a viable MVP choice if you want to avoid later churn.

## Source Set

### XML and document structure

- W3C XML 1.0: https://www.w3.org/TR/xml/

### HTML and CSS semantics

- MDN `class`: https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Global_attributes/class
- MDN `id`: https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Global_attributes/id
- MDN cascade: https://developer.mozilla.org/en-US/docs/Web/CSS/CSS_cascade/Cascade
- MDN CSS error handling: https://developer.mozilla.org/en-US/docs/Web/CSS/Guides/Syntax/Error_handling

### LVGL 9.x behavior and mapping surface

- LVGL flex: https://docs.lvgl.io/9.0/layouts/flex.html
- LVGL grid: https://docs.lvgl.io/9.1/layouts/grid.html
- LVGL style properties: https://docs.lvgl.io/9.1/overview/style-props.html
- LVGL button: https://docs.lvgl.io/9.0/widgets/button.html
- LVGL label: https://docs.lvgl.io/9.0/widgets/label.html

### Declarative UI language analogues

- Qt QML document structure: https://doc.qt.io/qt-6/qtqml-documents-structure.html
- Microsoft XAML syntax guide: https://learn.microsoft.com/en-us/windows/apps/develop/platform/xaml/xaml-syntax-guide
- Slint file structure: https://docs.slint.dev/latest/docs/slint/guide/language/coding/file/

## Question Matrix

| Question | External finding | Recommended LumaUI answer | Confidence |
| --- | --- | --- | --- |
| Must a `.lui` document contain exactly one top-level `Screen`? | XML, QML, and XAML all require a single root element/object per document/component. | Yes. One `.lui` file should contain exactly one top-level `Screen`. | High |
| Must `Screen` have exactly one child root widget? | Root-object systems usually allow the root object to own multiple children; single-root does not imply single-child. | No. Do not require exactly one child under `Screen`. Keep a single container as a style convention, not a grammar rule. | Medium |
| Is `Text` content expressed only through the `text` attribute? | LVGL Label text is represented through a text-setting API; inner-text systems like XAML require extra content-property semantics. | Yes for MVP. Use `text="..."` only. Defer inner-text shorthand. | Medium |
| May `Button` have arbitrary children? | LVGL Button adds no new containment limits beyond Base object; declarative UI systems commonly allow nested child trees. | Allow child widgets syntactically. Treat a nested `Text` label as the normative MVP pattern. | Medium |
| Is `class` a single identifier or a space-separated list? | HTML `class` is a whitespace-separated token list. XML tokenized names also use space-separated forms. | Space-separated list of identifier tokens. | High |
| Are self-closing tags allowed on all leaf widgets? | XML empty-element tags may be used for any element with no content. | Yes. Allow self-closing form for any widget with no children. | High |
| Are ids unique per screen or project-wide? | XML IDs are unique per document. HTML ids are unique in the entire document. | Unique across the active compiled project, not only within one screen. | Medium |
| Can a widget match multiple class rules? | Yes, in CSS an element may match multiple class selectors simultaneously. | Yes. A widget may match zero or more class rules. | High |
| In what order are matching rules applied? | CSS uses relevance, then origin, then specificity, then order of appearance. | Use a simplified CSS subset: `#id` outranks `.class`; later declarations break ties within the same specificity. | High |
| If the same property appears twice, which declaration wins? | CSS resolves equal-precedence conflicts by order of appearance. | Later declaration wins. This applies both within one rule and across equal-specificity rules. | High |
| Are width and height integers only, or do percentages exist in MVP? | LVGL 9 supports pixel, percentage, and content sizing for width and height. | Support integer pixels and percentages in MVP if parser scope allows. This is technically safe to ratify early. | Medium |
| Is `padding` scalar only, or are per-side values allowed? | LVGL exposes per-side padding and row/column gap properties. | Keep MVP `padding` scalar only and normalize to all four sides. Defer per-side and multi-value shorthands. | Medium |
| Are unsupported-but-well-formed properties rejected during parsing or semantic validation? | CSS and XAML both separate tokenization/parsing from grammar or property validity checks. | Parse first, reject semantically with diagnostics. | High |

## Detailed Findings

### 1. Single document root is strongly supported

#### External evidence

- XML defines a well-formed document as having exactly one root element.
- QML states that a QML document consists of a single root object declaration and that a `.qml` file must contain only a single root object definition.
- XAML states that a XAML file always has exactly one element serving as its root.
- Slint states that the root element of a valid `.slint` file must be a component.

#### Implication for LumaUI

LumaUI should not leave the top-level structure ambiguous. A `.lui` file should describe exactly one screen tree, and that tree should begin at `Screen`.

#### Recommendation

- Ratify: one `.lui` file = one top-level `Screen`.
- Reject multiple top-level widgets as a syntax error.

### 2. Single root does not imply single child under `Screen`

#### External evidence

- XAML uses one root object, but container roots can contain multiple child elements through collection content syntax.
- QML uses one root object, but object hierarchies commonly place multiple children under that root.
- Slint components have a root component but may declare multiple nested elements inside it.
- LVGL widgets are object trees; Button specifically adds no new features beyond Base object, which implies generic object composition rules still apply.

#### Implication for LumaUI

The external research supports one root object per document, not one child per root object.

Requiring `Screen` to have exactly one child widget would be a LumaUI-specific simplification, not a standard structural consequence of the XML-like model.

#### Recommendation

- Do not make `Screen` single-child-only.
- Keep `examples/minimal` as a single-container example because it is tidy and easy to reason about.
- If you want stronger stylistic guidance, document it as recommended structure rather than grammar law.

### 3. `class` should be tokenized, not singular

#### External evidence

- MDN defines the HTML `class` attribute as a list of class values separated by ASCII whitespace.
- MDN also recommends choosing class values that are valid CSS identifiers.
- XML tokenized name productions also use space-separated token lists.

#### Implication for LumaUI

If LumaUI keeps `.class` selectors, a token-list `class` attribute is the least surprising authored shape.

#### Recommendation

- Ratify `class` as a space-separated list.
- Restrict each token to a LumaUI identifier grammar chosen to avoid selector escaping.
- Disallow empty tokens after normalization.

### 4. `id` should be singular and globally unique within the compile unit

#### External evidence

- MDN defines HTML `id` as a single identifier that must be unique within the entire document.
- XML validity rules define ID values as unique within the XML document.

#### Implication for LumaUI

The closest analogue to a browser document is the active authored project being compiled and code-generated as one unit.

Per-screen uniqueness is possible, but project-wide uniqueness reduces collisions in:

- style application
- generated symbol naming
- event hook naming
- diagnostics wording
- future cross-screen references

#### Recommendation

- Keep `id` singular.
- Enforce uniqueness across the active compiled project.
- If later phases introduce namespacing or explicit screen scoping, that can loosen the constraint deliberately rather than by accident.

### 5. Self-closing tags are safe for empty widgets

#### External evidence

- XML permits empty-element tags for any element with no content.
- XAML also documents self-closing object syntax for objects that do not contain other objects.

#### Implication for LumaUI

Allowing `<Text text="Hello" />` and `<Button id="save" />` is fully aligned with the chosen XML-like shape and reduces verbosity.

#### Recommendation

- Allow self-closing syntax for any widget that has no child nodes.
- Treat a self-closing form as exactly equivalent to an open/close pair with empty content.

### 6. `Text` should use a `text` attribute in the MVP

#### External evidence

- LVGL Label is explicitly the basic widget used to display text.
- LVGL Label text is set through `lv_label_set_text` and related APIs.
- XAML demonstrates inner text only for types that declare a content-property model. That is a richer semantic layer than plain XML.

#### Implication for LumaUI

Inner-text support would require LumaUI to answer extra questions now:

- which widgets have text-content semantics
- whether text content and child widgets may coexist
- whether whitespace normalization applies
- whether mixed content is legal

That is unnecessary parser and semantic scope for the first slice.

#### Recommendation

- Ratify `Text` literal content through `text="..."` only in MVP.
- Defer `<Text>Hello</Text>` and mixed-content semantics until later, if ever.

### 7. `Button` can contain children, but the MVP should keep one normative pattern

#### External evidence

- LVGL Button has no new features compared to Base object and differs mainly in defaults and semantics.
- XAML and QML container-style elements routinely nest other objects.

#### Implication for LumaUI

There is no strong external reason to prohibit Button children entirely. At the same time, fully general child composition for buttons can widen semantic policy quickly.

#### Recommendation

- Allow child widgets under `Button` syntactically.
- In MVP docs and fixtures, standardize on a nested `Text` child as the normative label pattern.
- If needed for MVP simplicity, semantic validation may temporarily restrict child combinations to just the ones the backend supports cleanly.

### 8. Multiple class rules should match, but selector precedence should not ignore specificity

#### External evidence

- MDN cascade defines the relevant order as: relevance, origin and importance, specificity, then order of appearance.
- MDN explicitly states that when competing declarations have the same precedence and specificity, the last declaration in style order wins.

#### Implication for LumaUI

If LumaUI supports both `.class` and `#id`, a pure source-order-only model would violate common expectations by allowing a later class rule to override an earlier id rule.

That would be simple to implement but surprising to authors.

#### Recommendation

Adopt a deliberately simplified CSS subset:

- selector kinds: only `.class` and `#id`
- origins/layers/importance: none in MVP
- specificity: `#id` beats `.class`
- order of appearance: later wins only among declarations with equal specificity

This preserves familiarity without importing full CSS complexity.

### 9. Later declarations should win

#### External evidence

- MDN cascade states that when precedence and specificity are equal, the last declaration in style order is applied.
- MDN CSS error-handling examples also rely on the order-of-appearance rule when prefixed declarations are followed by the standard declaration.

#### Implication for LumaUI

The least surprising rule is:

- later declaration wins inside the same rule block
- later matching rule wins when selector specificity is equal

#### Recommendation

- Ratify last-declaration-wins for equal-precedence conflicts.

### 10. Width and height percentages are technically safe in LVGL 9

#### External evidence

- LVGL style properties state that `width` accepts pixel, percentage, and `LV_SIZE_CONTENT`, with percentages relative to the parent's content area.
- LVGL style properties state the same for `height`.

#### Implication for LumaUI

Percent sizing is not a speculative backend feature. It is already part of LVGL's documented sizing model.

The question is therefore a scope decision, not a mapping-risk decision.

#### Recommendation

Two honest options exist:

Option A, lower parser scope:
- keep MVP width/height numeric only
- document percentages as explicitly deferred even though LVGL supports them

Option B, lower future churn:
- allow `<integer>` and `<percentage>` for `width` and `height` in MVP
- define percentages as relative to the parent content area exactly as LVGL does

Preferred recommendation:
- choose Option B unless parser scope is already under pressure

Rationale:
- the syntax cost is small
- the LVGL mapping is direct
- the rule is not browser-invented behavior

### 11. Padding should stay scalar in MVP, even though LVGL exposes per-side properties

#### External evidence

- LVGL documents `pad_top`, `pad_bottom`, `pad_left`, `pad_right`, `pad_row`, and `pad_column`.
- LVGL describes padding as space between the parent's sides and the children and among the children.

#### Implication for LumaUI

Per-side padding maps cleanly, but exposing all of it in MVP expands both parser grammar and normalization rules.

Scalar `padding: N` can still lower deterministically by writing the same value to all four sides.

#### Recommendation

- Keep MVP `padding` as a single scalar integer.
- Normalize it semantically to all four LVGL side paddings.
- Defer per-side properties and any two-value or four-value shorthand.

### 12. Unsupported properties and selectors should survive parsing and fail during semantic validation

#### External evidence

- MDN CSS error handling says that if a property name or value is invalid, the property-value declaration is ignored while parsing continues.
- MDN also notes that invalid selectors can invalidate the selector block while parsing continues after the closing brace.
- XAML separates XML parsing from whether an attribute or property is meaningful within the backing object model.

#### Implication for LumaUI

LumaUI is not a browser and should not silently ignore unsupported authored input. But the external model still supports separating syntactic well-formedness from semantic support.

That separation produces better diagnostics and cleaner stage boundaries.

#### Recommendation

- The parser should accept well-formed rule and declaration syntax for the ratified grammar.
- Semantic validation should reject unsupported selector kinds, unsupported property names, and unsupported value shapes with actionable diagnostics.
- Do not make the parser responsible for policy decisions that belong to the semantic stage.

## Bonus Finding: Event handler references should stay as identifier-shaped attribute values

#### External evidence

- XAML event-handler placeholders are function names supplied as attribute values in markup.
- LVGL events are callback-based and registered against widgets rather than embedded as inline code.

#### Recommendation

- Keep event references as quoted identifier values such as `onPress="open_settings"`.
- Avoid embedded expressions, code snippets, or argument lists in MVP.

## Concrete Recommendations For Ratification

If the goal is to minimize author friction without widening scope irresponsibly, the best-supported contract is:

### Markup

- one `.lui` file contains exactly one top-level `Screen`
- `Screen` may contain one or more child widgets
- supported widgets remain `Screen`, `Column`, `Row`, `Text`, and `Button`
- `class` is a space-separated list of identifier tokens
- `id` is a single identifier token
- ids are unique across the active compiled project
- self-closing tags are allowed for any empty widget
- `Text` literal content is expressed through `text="..."`
- `Button` may contain children, but nested `Text` is the normative MVP pattern
- events are identifier-shaped attribute values such as `onPress="open_settings"`

### Styles

- a widget may match multiple class selectors and at most one id selector
- selector precedence is `#id` over `.class`
- later declarations break ties within equal specificity
- repeated properties resolve by last declaration wins
- `width` and `height` may safely allow integer pixels and percentages
- `padding` should stay scalar in MVP and normalize to all four sides
- unsupported selectors and properties should fail during semantic validation, not during basic parsing

## Follow-on Impact On Existing Review Notes

This research supports revising two parts of `docs/LANGUAGE_REVIEW.md`:

1. Replace the suggested rule that `Screen` must have exactly one child widget with a softer recommendation that the normative fixture may use one container child, while the grammar allows multiple direct children.
2. Revisit the suggested rule that matching rules are applied only in source order. With both `.class` and `#id` present, a small specificity model is better supported by external practice and will be less surprising.

It also suggests one possible expansion of the earlier MVP recommendation:

3. `width` and `height` percentages are technically justified in the first slice if the team wants to reduce future contract churn.

## Open Questions Remaining After Research

The following are still policy decisions rather than externally-answered questions:

- whether to include percentage `width` and `height` immediately or defer them for parser-scope reasons
- whether MVP semantic validation should allow any child widgets under `Button`, or only the subset the backend can lower today
- whether `padding` should remain scalar-only for the full MVP or expand to side-specific properties in the first ratified grammar

These are now narrow tradeoff decisions, not evidence gaps.