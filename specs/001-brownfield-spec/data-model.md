# Data Model: Brownfield MVP Compiler Slice

## Overview

This feature introduces a tightly bounded domain model for the first ratified LumaUI slice. The model is intentionally split by compiler stage so syntax, semantic meaning, canonical IR, and generated output stay distinct.

## Entities

### BrownfieldDocumentHierarchy

Defines the governing order for planning and implementation decisions.

**Fields**:

- `constitution_source`: canonical governing document path
- `supporting_docs`: ordered list of subordinate guidance documents
- `active_phase`: current roadmap phase label
- `current_exit_gate`: the next gate that must be satisfied before scope expands

**Validation rules**:

- `constitution_source` must resolve to the constitution document used by Speckit.
- Conflicts are resolved in favor of the constitution, then the remaining docs in declared order.

### RatifiedFirstSlice

Represents the explicitly supported authored-language subset for the current brownfield iteration.

**Fields**:

- `supported_widgets`: `Screen`, `Column`, `Row`, `Text`, `Button`
- `supported_selector_kinds`: `class`, `id`
- `supported_style_properties`: `padding`, `background-color`, `text-color`, `width`, `height`
- `supported_event_references`: named handler references only
- `binding_policy`: rejected in MVP
- `deferred_constructs`: widgets, selectors, properties, and semantics intentionally out of scope

**Validation rules**:

- Every construct used by the normative fixture must appear in `supported_*` or `deferred_constructs`.
- Bindings must not appear in accepted authored input for this slice.

### NormativeFixture

Identifies an example that is expected to pass during the active phase.

**Fields**:

- `name`: fixture identifier
- `project_root`: example directory
- `status`: `normative` or `aspirational`
- `expected_cli_behaviors`: supported commands and expected outcomes
- `snapshot_targets`: generated artifacts expected to remain stable

**Validation rules**:

- A normative fixture may only use ratified constructs.
- An aspirational fixture must be visibly labeled in docs or tests.

### WorkspaceConfig

Represents `lumaui.toml` and controls project discovery.

**Fields**:

- `project_name`: logical project identifier
- `lvgl_version`: targeted LVGL major baseline
- `source_dir`: source root for authored files
- `output_dir`: generated output root
- `symbol_prefix`: configured naming prefix for generated symbols

**Relationships**:

- One `WorkspaceConfig` belongs to one `AuthoredProject`.
- `WorkspaceConfig` determines the `ProjectLayout` used by CLI commands.

### ProjectLayout

Represents discovered authored sources and output locations.

**Fields**:

- `project_root`
- `source_root`
- `output_root`
- `screen_files`: sorted list of `.lui` files
- `style_files`: sorted list of `.lus` files

**Validation rules**:

- File discovery order must be deterministic.
- Missing source directories are allowed but must produce a clear doctor or validation signal.

### SourceDocument

Represents one parsed authored input file.

**Fields**:

- `source_name`
- `kind`: `Markup` or `Style`
- `items`: top-level authored nodes

**Relationships**:

- One `AuthoredProject` has many `SourceDocument` records.
- Each `SourceDocument` is parsed into AST nodes before semantic analysis.

### WidgetNode

Represents markup syntax for one authored widget in the parser stage.

**Fields**:

- `widget_type`
- `id`
- `classes`
- `attributes`
- `children`

**Validation rules**:

- `widget_type` must be in the ratified supported set for accepted input.
- `id`, if present, must be unique within the compiled project.
- Child widgets must follow the ratified tree grammar.

### Attribute

Represents one parser-stage markup attribute.

**Fields**:

- `name`
- `value`

**Attribute value variants**:

- `String`
- `Reference`

**Validation rules**:

- Event attributes must use named references only.
- Binding-shaped references are rejected for the MVP slice.

### StyleRule

Represents one parser-stage style rule.

**Fields**:

- `selector`
- `declarations`

**Validation rules**:

- Only class and id selectors are valid in the first slice.
- Unsupported selector forms must produce actionable diagnostics.

### Declaration

Represents one parser-stage style declaration.

**Fields**:

- `name`
- `value`

**Validation rules**:

- `name` must be in the supported property set for accepted input.
- `value` must parse into the normalized semantic form expected by the property.

### DiagnosticRecord

Represents one user-facing validation or build finding.

**Fields**:

- `severity`
- `message`
- `source_file`
- `span`
- `hint`

**Validation rules**:

- Syntax diagnostics must include source spans where available.
- Unsupported-construct diagnostics must identify what to change or remove.

### SemanticProject

Represents the semantically validated authored project before backend generation.

**Fields**:

- `project_name`
- `screens`
- `normalized_styles`
- `event_references`
- `diagnostics`

**Relationships**:

- Created from `SourceDocument` records plus `WorkspaceConfig`.
- Lowers into `CanonicalIRProject`.

### CanonicalIRProject

Represents the backend-facing canonical model.

**Fields**:

- `project_name`
- `screens`

**Related entities**:

- `ScreenModel`
- `WidgetModel`
- `NormalizedStyleApplication`
- `EventBindingSlot`

**Validation rules**:

- The IR must be free of syntax-specific ambiguity.
- Only semantically accepted constructs may appear in IR.

### ScreenModel

Represents one named screen in canonical form.

**Fields**:

- `name`
- `root_widget`

### WidgetModel

Represents one backend-facing widget node.

**Fields**:

- `kind`
- `id`
- `text`
- `children`
- `applied_styles`
- `event_references`

**Validation rules**:

- `kind` must map to a documented LVGL 9.x primitive.
- Style application must be normalized before backend generation.

### GeneratedScreenArtifact

Represents one emitted C or header file.

**Fields**:

- `path`
- `contents`
- `artifact_kind`: `header` or `source`
- `screen_name`
- `ownership_regions`

**Validation rules**:

- File paths and symbol names must be deterministic.
- Compiler-owned regions are regenerated; user-owned escape-hatch regions remain delimited and preserved.

### PhaseGate

Represents a roadmap checkpoint that controls scope expansion.

**Fields**:

- `name`: `Language`, `Parser`, `Semantic`, `Backend`, `ExamplesAndStability`
- `entry_conditions`
- `exit_conditions`
- `status`

**Validation rules**:

- Later gates cannot be treated as complete until the previous gate's exit conditions are satisfied.

## Relationships

- One `BrownfieldDocumentHierarchy` governs one `RatifiedFirstSlice`.
- One `NormativeFixture` instantiates one `AuthoredProject` defined by one `WorkspaceConfig` and one `ProjectLayout`.
- One `AuthoredProject` contains many `SourceDocument` records.
- Markup `SourceDocument` records contain `WidgetNode` trees; style `SourceDocument` records contain `StyleRule` records with `Declaration` values.
- Semantic analysis converts `SourceDocument` plus `WorkspaceConfig` into a `SemanticProject` and `DiagnosticRecord` values.
- Lowering converts `SemanticProject` into `CanonicalIRProject`.
- Backend generation converts `CanonicalIRProject` into one or more `GeneratedScreenArtifact` records.
- `PhaseGate` instances control when a downstream transformation is allowed to expand.

## State Transitions

### Fixture lifecycle

1. `Aspirational`
2. `NormativeCandidate`
3. `Parses`
4. `SemanticallyValid`
5. `GeneratesStableArtifacts`
6. `Normative`

### Source pipeline lifecycle

1. `Discovered`
2. `Parsed`
3. `Validated`
4. `LoweredToIR`
5. `Generated`
6. `Snapshotted`

Any diagnostic at one stage blocks advancement to the next stage.