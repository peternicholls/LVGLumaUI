# Language Review Before Semantic Phase

## Purpose

This document captures the main language-contract and planning gaps that should be resolved before broad Phase 2 semantic and IR work begins.

The goal is not to broaden the MVP. The goal is to remove avoidable ambiguity so parser, semantic, IR, and backend work do not each make their own local guesses.

External research supporting and refining the recommendations in this document is recorded in `specs/001-brownfield-spec/decisions/language-open-questions-research.md`.

## Overall Assessment

The repository is close to a coherent MVP slice, but it is not yet ready for broad semantic implementation.

The main issue is not missing code. The main issue is that the brownfield spec pack already assumes an exact first-slice contract, while the main language document still describes the syntax as provisional and directional.

That mismatch will create friction immediately in semantic normalization, IR design, fixture classification, and test expectations.

## Blocking Gaps

### 1. Grammar ratification is still contradictory

Current state:

- `docs/LANGUAGE_SPEC.md` still says final markup grammar, final style grammar, exact selector syntax, exact event syntax, and exact binding syntax are not fixed yet.
- `specs/001-brownfield-spec/spec.md`, `plan.md`, `research.md`, and `data-model.md` already assume the first slice should ratify exact grammar and exact event policy.

Why this matters:

- Parser work can proceed with a narrow grammar.
- Semantic work cannot proceed honestly if the accepted surface is still described as provisional.

Recommended resolution:

- Treat the first slice as ratified at the markup and style surface level, even if the broader language remains provisional.
- Move `docs/LANGUAGE_SPEC.md` from "directional" wording to an explicit accepted-and-rejected MVP contract.

### 2. Event support is in scope, but the contract is not exercised clearly

Current state:

- The brownfield spec requires named event handler references in the first slice.
- `examples/minimal` does not currently use an event reference.
- `docs/LVGL_MAPPING.md` still leaves the generated callback contract as TODO.

Why this matters:

- If events are part of MVP, they need one normative authoring shape.
- If the normative fixture never uses them, event support will be underspecified until backend time.

Recommended resolution:

- Ratify a single event form: `onPress="handler_name"`.
- Constrain handler names to identifier syntax only.
- Add one event usage to the normative path, either in `examples/minimal` or in a second clearly normative micro-fixture.
- Keep the generated C callback signature as a backend-phase concern, but freeze the authored syntax now.

### 3. Bindings are deferred, but the planning language still leaks them into Phase 2

Current state:

- The first-slice policy is to reject bindings.
- Some planning and architecture text still describes semantic and IR work as representing both event and binding references.

Why this matters:

- This creates false pressure to design binding semantics, IR shape, or validation rules before the repository is ready.
- It also weakens the narrow-slice discipline the constitution requires.

Recommended resolution:

- Remove binding representation from the MVP semantic and IR deliverables.
- Keep only explicit binding rejection in the first semantic slice.
- Reintroduce binding representation only when a later slice actually ratifies binding syntax and lifecycle semantics.

### 4. Markup semantics are still too implicit

Current state:

- The examples imply an XML-like tree language.
- The docs do not yet define key tree and attribute rules tightly enough.

Questions that remain open:

- Must a `.lui` document contain exactly one top-level `Screen`?
- Must `Screen` have exactly one child root widget?
- Is `Text` content expressed only through the `text` attribute?
- May `Button` have arbitrary children, or only the subset needed for MVP?
- Is `class` a single identifier or a space-separated list?
- Are self-closing tags allowed on all leaf widgets?
- Are ids unique per screen or project-wide for the active compile unit?

Why this matters:

- These choices affect parser shape, semantic validation, IR structure, and fixture authoring.
- Leaving them implicit will produce low-quality diagnostics and brittle tests.

Recommended resolution:

- Freeze the MVP tree grammar explicitly.
- Prefer the narrowest rules that reduce ambiguity without inventing unnecessary structural restrictions.

Suggested MVP decisions:

- A markup document contains exactly one top-level `Screen`.
- `Screen` may contain one or more child widgets in the first slice.
- `Text` uses the `text` attribute for literal content.
- `Button` may contain child widgets, but the normative MVP pattern is a nested `Text` label.
- `class` is a space-separated list of identifiers.
- Self-closing tags are allowed for widgets with no children.
- Id uniqueness is project-wide across the validated authored project.

### 5. Style semantics are still too implicit

Current state:

- The brownfield research narrows selectors to `.class` and `#id` and narrows properties to `padding`, `background-color`, `text-color`, `width`, and `height`.
- The docs still do not define the minimum merge and override rules clearly enough for semantic normalization.

Questions that remain open:

- Can a widget match multiple class rules?
- In what order are matching rules applied?
- If the same property appears twice, which declaration wins?
- Are width and height integers only, or do percentages exist in MVP?
- Is `padding` a scalar only, or are per-side values allowed?
- Are unsupported-but-well-formed properties rejected during parsing or semantic validation?

Why this matters:

- Phase 2 needs a normalization rule, not just a property list.
- Without a deterministic merge model, IR and backend behavior will drift.

Recommended resolution:

- Freeze the style application model now, even if the property surface stays tiny.

Suggested MVP decisions:

- Only `.class` and `#id` selectors are legal.
- A widget may match multiple class rules and at most one id rule.
- Matching rules use a simplified specificity model: `#id` overrides `.class`, and later declarations break ties within the same specificity.
- For the same property, the last matching declaration wins when specificity is equal.
- `width` and `height` accept integer pixel values and percentages in MVP.
- `padding` accepts integer scalar values only in MVP.
- Unsupported selectors and properties are parser-valid syntax but semantic errors.

### 6. Planning terminology has a real friction point

Current state:

- The roadmap docs use `Phase 2` to mean semantic analysis and IR.
- `specs/001-brownfield-spec/tasks.md` uses `Phase 2` to mean User Story 1 contract ratification.

Why this matters:

- This is likely to confuse task planning, review discussions, and commit messages.
- The user request itself already surfaced the ambiguity around "before phase 2".

Recommended resolution:

- Reserve `Phase N` for the repository roadmap only.
- Refer to spec-pack execution slices as `Setup`, `US1`, `US2`, `US3`, or `Task Phase N`.
- Avoid using bare "phase 2" in planning conversation unless the document context is explicit.

## Decision Matrix

The table below turns the open questions into recommended ratification choices.

| Topic | Options considered | Recommendation | Why this is the best MVP choice |
| --- | --- | --- | --- |
| Document root | One top-level `Screen`; multiple top-level widgets | One top-level `Screen` per `.lui` file | Strong XML/QML/XAML precedent and cleaner parser and diagnostics |
| `Screen` children | Exactly one child; one or more children | Allow one or more children | Keeps the single-root rule without adding an unnecessary tree restriction |
| `Text` content model | `text="..."` only; inner text shorthand; mixed content | `text="..."` only in MVP | Smallest explicit model and direct LVGL mapping |
| `Button` content model | No children; arbitrary children; limited child support | Allow child widgets, with nested `Text` as the normative MVP pattern | Preserves flexibility without widening the normative surface too far |
| `class` representation | Single token; space-separated token list | Space-separated token list | Least surprising authored shape once `.class` selectors exist |
| `id` scope | Per-screen unique; project-wide unique | Project-wide unique within the active compile unit | Simplifies diagnostics, symbol generation, and future references |
| Empty widgets | Require open/close tags; allow self-closing form | Allow self-closing tags for widgets with no children | Matches XML expectations and reduces verbosity |
| Style matching | Source order only; simplified specificity; full CSS cascade | Simplified specificity: `#id` beats `.class`, later wins ties | Familiar enough for authors without importing full CSS complexity |
| Duplicate property resolution | First wins; last wins | Last declaration wins for equal specificity | Deterministic and aligned with common CSS expectations |
| `width` and `height` values | Integer pixels only; pixels plus percentages | Support integer pixels and percentages in MVP | LVGL 9 maps percentages directly, so early support avoids later churn |
| `padding` values | Scalar only; per-side properties; multi-value shorthand | Scalar integer only in MVP | Keeps grammar and normalization small while mapping cleanly to LVGL |
| Unsupported but well-formed syntax | Reject in parser; reject semantically | Parse first, reject semantically | Better diagnostics and cleaner stage boundaries |
| Event authoring shape | Defer; multiple event syntaxes; one named handler form | Ratify `onPress="handler_name"` only | Gives MVP one clear event contract without dragging backend details forward |
| Bindings | Partial support; parse-only; explicit rejection | Explicitly reject bindings in MVP | Preserves narrow-slice discipline and avoids premature semantic design |

## Recommended MVP Contract Decisions

These decisions would minimize friction while staying within the current constitution and brownfield plan.

### Markup

- Syntax stays XML-like.
- The first slice supports `Screen`, `Column`, `Row`, `Text`, and `Button` only.
- One `.lui` file contains one top-level `Screen`.
- `Screen` may contain one or more child widgets.
- `id` is optional and unique across the authored project.
- `class` is optional and space-separated.
- Self-closing tags are valid for widgets with no children.
- String-valued widget attributes use quoted string literals.
- Event references use quoted identifier strings in `onPress` only.
- `bind` is rejected everywhere in MVP.

### Styles

- One rule has one selector and a block of declarations.
- Only `.class` and `#id` selectors are valid.
- Only `padding`, `background-color`, `text-color`, `width`, and `height` are valid properties.
- `width` and `height` accept integer literals and percentages.
- `padding` accepts integer scalar literals only.
- Color properties use hex colors only in MVP.
- Widgets may match multiple class rules and at most one id rule.
- Matching style rules use simplified specificity: `#id` overrides `.class`, and later declarations win ties.

### Validation boundaries

- The parser owns syntactic structure and spans.
- The semantic layer owns supported-surface checks and normalization.
- The parser should not reject a well-formed property only because it is unsupported.
- The semantic layer should not reinterpret invalid syntax as a fallback form.

## Pre-Phase-2 Checklist

Before broad semantic work starts, the repository should satisfy all of the following:

1. `docs/LANGUAGE_SPEC.md` explicitly distinguishes accepted MVP syntax from deferred broader language work.
2. The event-reference syntax is frozen in docs and exercised by a normative fixture.
3. Bindings are described only as rejected input in the MVP semantic contract.
4. Markup tree rules and style merge rules are written down, not inferred from examples.
5. The roadmap terminology avoids the current "Phase 2" naming collision.
6. `examples/minimal`, tests, and starter output all stay inside the same ratified subset.

## Recommended Next Action

The next useful change is not broad semantic implementation.

The next useful change is to turn this review into concrete ratification updates in:

- `docs/LANGUAGE_SPEC.md`
- `docs/NEXT_STEPS.md`
- `docs/TASKS.md`
- `docs/ARCHITECTURE.md`
- `docs/LVGL_MAPPING.md`
- `README.md`
- `examples/minimal`

That keeps Phase 2 focused on semantic work instead of using semantic work to finish language design.