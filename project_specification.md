Below is a compact spec pack you can drop into a repo, followed by a one-shot Codex prompt tuned for an agentic coding workflow. The spec pack is intentionally detailed to provide clear guardrails for the language design and implementation, while the prompt is designed to get Codex to produce real file contents and a coherent initial codebase.

# Spec pack

## 1. Project title

**LumaUI**
A declarative, web-inspired UI language and compiler for LVGL that generates deterministic, readable C code for embedded applications.

## 2. Product summary

LumaUI is a compile-time UI authoring system for LVGL. It gives developers and designers a familiar declarative syntax inspired by HTML and CSS, but intentionally constrained to embedded-safe semantics. It is not a browser engine, not a DOM runtime, and not a full HTML/CSS implementation. Its purpose is to let teams author screens, styles, assets, and bindings in plain text, then compile them into LVGL-flavoured C with minimal runtime overhead.

## 3. Problem statement

LVGL is powerful, but authoring non-trivial interfaces directly in C is laborious and noisy. Existing higher-level workflows are either commercial, insufficiently open, or not aligned with the mental model of web-oriented designers and frontend developers.

Teams need:

* a declarative UI source format
* CSS-like theming and reusable classes
* deterministic code generation to plain C
* zero dependency on a browser-style runtime
* a toolchain that fits embedded engineering practices, CI, and version control

## 4. Vision

Create the best open declarative UI compiler for LVGL: familiar enough for web-minded authors, strict enough for embedded systems, and transparent enough that generated code remains auditable and production-grade.

## 5. Goals

### Primary goals

* Declarative authoring for LVGL UIs
* Compile to plain, readable, deterministic C
* Embedded-safe subset of layout and styling
* Fast feedback via preview/simulator support
* Strong schema validation and helpful diagnostics
* Text-first workflow suitable for git and CI

### Secondary goals

* Easy migration path for teams already comfortable with HTML/CSS concepts
* Component reuse and theme tokens
* Extensible backend architecture for future targets

## 6. Non-goals - explicitly out of scope

* Full HTML support
* Full CSS cascade/selectors model
* Browser DOM emulation
* JavaScript runtime in-device
* Arbitrary client-side scripting
* Pixel-perfect parity with browsers
* WYSIWYG-first tooling in v1

## 7. Target users

### Embedded developers

Need maintainable LVGL UIs without hand-writing large amounts of repetitive C.

### Web/frontend developers

Need a familiar path into embedded UI authoring without learning LVGL internals on day one.

### Product teams

Need versionable UI source, reusable themes, CI validation, and deterministic builds.

## 8. Core design principles

1. **Compile-time first**
   Production path is source → IR → generated C → firmware build.

2. **Strict subset, not imitation browser**
   Borrow ergonomics from HTML/CSS without inheriting their full complexity.

3. **Deterministic output**
   Same input and version must produce byte-stable generated output where practical.

4. **Readable generated code**
   The output must be inspectable by engineers and debuggable without magic.

5. **Escape hatches**
   Allow explicit hooks into raw LVGL where necessary.

6. **Separation of concerns**
   Syntax, semantic analysis, IR, and backend codegen should be distinct layers.

## 9. Language overview

## 9.1 Markup language

A small declarative tree language, tentatively `.lui`, with tags representing screens and widgets.

Example:

```xml
<Screen id="home">
  <Row class="header">
    <Text text="Hello"/>
    <Button id="settingsBtn" onPress="open_settings">
      <Text text="Settings"/>
    </Button>
  </Row>

  <Card class="panel">
    <Text class="title" text="Temperature"/>
    <Text class="value" bind="temp_text"/>
  </Card>
</Screen>
```

## 9.2 Style language

A CSS-inspired style file with limited selectors and embedded-safe properties.

Example:

```css
.header {
  width: 100%;
  layout: row;
  justify: space-between;
  padding: 12;
}

.panel {
  bg-color: #20242b;
  radius: 8;
  padding: 16;
}

.title {
  font-size: 18;
  text-color: #cfd6e4;
}
```

## 9.3 Constraints

Supported:

* class selectors
* id selectors
* element selectors, optionally
* token variables
* simple inheritance or theme tokens if explicitly designed

Not supported in v1:

* descendant combinators beyond a very restricted subset
* pseudo-elements
* complex specificity wars
* floats
* browser positioning semantics
* scripting expressions beyond simple binding references

## 10. Supported UI concepts in MVP

### Containers

* `Screen`
* `Container`
* `Row`
* `Column`
* `Grid`
* `Card`

### Content widgets

* `Text`
* `Button`
* `Image`
* `Icon`
* `Input`
* `Switch`
* `Slider`
* `ProgressBar`

### Structural features

* `id`
* `class`
* inline properties
* event handler names
* simple data binding references
* asset references
* theme tokens

## 11. Layout model

The layout model must map cleanly to LVGL primitives.

### MVP layout support

* row
* column
* simple grid
* width and height
* percentage where LVGL supports it sensibly
* padding
* margin subset
* gap
* align / justify subset
* grow / shrink subset only if it maps cleanly

### Explicitly avoid in MVP

* browser box model quirks
* absolute/fixed/sticky semantics unless carefully scoped
* intrinsic text reflow assumptions beyond what LVGL already handles

## 12. Styling model

Properties should be semantic and map directly to LVGL concepts.

Examples:

* `bg-color`
* `text-color`
* `opacity`
* `radius`
* `border-width`
* `border-color`
* `padding`
* `margin`
* `font`
* `font-size`
* `align`
* `width`
* `height`

Every property should have:

* declared type
* valid units
* default behaviour
* LVGL mapping
* compile-time validation rules

## 13. Events and bindings

## 13.1 Events

Declare handler names only; business logic remains in user C code.

Example:

```xml
<Button onPress="open_settings"/>
```

Codegen produces registration hooks to named functions.

## 13.2 Bindings

Bindings are symbolic references, not arbitrary expressions in v1.

Example:

```xml
<Text bind="temp_text"/>
```

Backend options:

* bind through generated setter functions
* bind through explicit user-provided data update calls
* optional generated view-model struct

No hidden reactive runtime in v1.

## 14. Asset pipeline

Support:

* bitmap/image asset references
* font declarations
* icon mapping
* optional localisation string tables later

The asset pipeline should:

* validate files
* optionally convert assets
* emit predictable C/H references
* support hash-based cache invalidation in tooling

## 15. Architecture

## 15.1 Frontend

Parses source files into ASTs:

* markup parser
* style parser
* schema validator

## 15.2 Semantic analysis

Resolves:

* ids
* classes
* components
* style rules
* property typing
* event signatures
* asset references

## 15.3 Intermediate representation

IR should contain:

* widget tree
* resolved styles
* layout rules
* binding metadata
* event metadata
* asset references
* source spans for diagnostics

## 15.4 Backend

Initial backend:

* LVGL C code generator

Future backends:

* JSON IR export
* preview runtime
* alternate embedded targets

## 16. Code generation requirements

Generated code must be:

* deterministic
* split into sensible `.c` and `.h` files
* stable in ordering
* consistent in naming
* easy to diff
* easy to call from application code

Suggested output structure:

```text
ui/
  src/
    screens/
      home_gen.c
      home_gen.h
    styles/
      theme_gen.c
      theme_gen.h
    assets/
      assets_gen.c
      assets_gen.h
    ui_gen.c
    ui_gen.h
```

## 17. Escape hatches

Support at least one or more of:

* custom post-create hook per widget
* raw LVGL property injection block
* generated object IDs accessible to user code
* preserve sections in generated output only if very carefully designed

Preferred approach: generated hooks and partials, not hand-editing generated files.

## 18. CLI

Proposed commands:

```bash
lumaui init
lumaui validate
lumaui build
lumaui preview
lumaui doctor
```

### `validate`

Checks syntax, semantics, unresolved bindings, unknown properties, unsupported layouts.

### `build`

Generates C/H files and asset outputs.

### `preview`

Runs SDL/LVGL simulator preview where available.

### `doctor`

Checks LVGL version compatibility, assets, fonts, project config.

## 19. Project structure

```text
project/
  ui/
    screens/
      home.lui
    styles/
      theme.luss
    assets/
      logo.png
    tokens/
      colors.json
  src/
    app.c
    ui_handlers.c
    ui_handlers.h
  lumaui.config.json
```

## 20. Configuration

Example:

```json
{
  "projectName": "demo-dashboard",
  "lvglVersion": "9.x",
  "input": "./ui",
  "output": "./generated/ui",
  "assets": {
    "embed": true
  },
  "naming": {
    "prefix": "app_"
  },
  "preview": {
    "enabled": true
  }
}
```

## 21. Diagnostics

Compiler diagnostics must be a major feature, not an afterthought.

Requirements:

* line/column aware errors
* suggestions for mistyped properties
* warnings for unsupported constructs
* clear mapping from source to generated artifact
* duplicate ID detection
* unused style detection later

## 22. Testing strategy

### Unit tests

* parser
* style resolution
* property typing
* LVGL mapping
* naming determinism

### Snapshot tests

* generated C output for fixtures

### Integration tests

* compile generated code against LVGL
* run simulator smoke tests

### Golden examples

* dashboard
* settings screen
* form screen
* media controller

## 23. Security and safety

Because this is a compiler/generator:

* no arbitrary code execution during parse
* no unsandboxed template execution
* asset parsing should be bounded and validated
* codegen must avoid unsafe string injection into generated C
* handler references must be validated symbols, not interpolated code blobs

## 24. Licensing and legal posture

* Project should use a permissive open-source license such as MIT or Apache-2.0
* All syntax, schemas, docs, and code should be original
* Public LVGL APIs are fair game as the compilation target
* Avoid cloning proprietary naming, formats, or documentation structures too closely
* Position the project as an original declarative UI compiler for LVGL, not a replica of a commercial toolchain

## 25. Milestones

### M0 — foundations

* repo bootstrap
* CLI skeleton
* config loader
* test harness
* fixture examples

### M1 — parsing and validation

* markup parser
* style parser
* AST
* diagnostics
* schema checks

### M2 — semantic layer and IR

* id/class resolution
* typed properties
* style application
* IR builder

### M3 — LVGL backend

* generate screens
* generate styles
* generate assets
* compile working examples

### M4 — preview

* SDL simulator integration
* hot rebuild
* basic preview runner

### M5 — polish

* docs
* examples
* component library
* CI snapshots
* error message improvements

## 26. Acceptance criteria for MVP

MVP is done when:

* a user can define at least three screens in declarative source
* classes and ids work
* row/column/grid layouts work for the supported subset
* buttons, text, images, and cards generate valid LVGL C
* handlers can be attached by name
* generated code compiles against supported LVGL versions
* simulator preview works for example apps
* generated output is stable across repeated builds
* docs include a quickstart and language reference

# Repo files to create first

## `README.md`

Explain concept, goals, non-goals, and quickstart.

## `PRD.md`

Use the material above.

## `ARCHITECTURE.md`

Describe parser → semantic analysis → IR → backend.

## `LANGUAGE_SPEC.md`

Define syntax and property semantics.

## `LVGL_MAPPING.md`

Map each supported widget/property to LVGL APIs.

## `AGENTS.md`

Tell Codex exactly how to behave in this repo. OpenAI’s recent guidance specifically highlights repo-local instructions and reusable skills as a productive pattern for coding agents. ([OpenAI Developers][2])

## `TASKS.md`

Concrete milestones and implementation order.

## `examples/`

A few minimal fixtures.

# How to use recommendation

Use the one-shot prompt to get Codex to build the first repo pass, but do not let it design the language unsupervised without guardrails. The risky failure mode is predictable: it will drift toward “mini web browser” rather than “embedded UI compiler.” The spec pack above is your fence.
