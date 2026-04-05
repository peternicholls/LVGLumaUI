# Contract: CLI Commands for the Brownfield MVP Slice

## Purpose

The CLI is the public interface for exercising the compiler pipeline during the brownfield MVP slice. This contract defines command behavior, expected inputs, phase-aware outputs, and failure conditions.

## Shared Inputs

All project-oriented commands operate on a directory containing:

- `lumaui.toml`
- `ui/screens/*.lui`
- `ui/styles/*.lus`

Discovery rules:

- screen files are discovered from `source_dir/screens`
- style files are discovered from `source_dir/styles`
- discovery order must be deterministic
- generated output is written under `output_dir`

## Command: `lumaui doctor [project]`

**Purpose**: Report project layout and discovery information without parsing or generating output.

**Inputs**:

- optional project path, default `.`

**Behavior**:

- loads `lumaui.toml`
- reports project root, config path, LVGL baseline, source root, output root, and discovered screen and style counts
- warns when screen or style directories are empty

**Success contract**:

- exits successfully when config loading and discovery succeed
- emits deterministic line ordering

**Failure contract**:

- exits with an error when the config file cannot be read or parsed

## Command: `lumaui validate [project]`

**Purpose**: Parse and validate the authored project against the currently implemented slice.

**Inputs**:

- optional project path, default `.`

**Behavior during the brownfield MVP slice**:

- loads config and discovers source files
- parses markup and style documents
- reports syntax diagnostics with file and span context where available
- performs semantic validation for the supported first slice
- rejects malformed syntax, unsupported widgets, unsupported properties, duplicate ids, bindings, and other deferred constructs
- does not generate output files

**Success contract**:

- exits successfully only when the authored project conforms to the implemented slice
- reports project summary and validation results in deterministic order

**Failure contract**:

- exits with an error if parsing fails or semantic validation reports errors
- each error includes corrective direction where possible

## Command: `lumaui build [project]`

**Purpose**: Produce deterministic LVGL C artifacts from a valid authored project.

**Inputs**:

- optional project path, default `.`

**Behavior during the brownfield MVP slice**:

- performs the same discovery, parsing, and semantic validation steps as `validate`
- lowers accepted input into canonical IR
- generates deterministic `.c` and `.h` files under `generated/ui/screens`
- uses only explicit LVGL 9.x mappings for the supported slice

**Success contract**:

- exits successfully only when a non-empty semantic IR is available for generation
- repeated builds of unchanged input produce stable file paths and stable contents

**Failure contract**:

- exits with an error if validation fails or if the requested build is not yet available for the active phase
- must not silently skip unsupported constructs

## Command: `lumaui init [path]`

**Purpose**: Create a starter project aligned with the currently recommended authoring shape.

**Inputs**:

- optional target path, default `.`
- optional `--name`
- optional `--force`

**Behavior**:

- creates a config file, `ui/screens`, and `ui/styles`
- writes a starter `main.lui` and `theme.lus`
- refuses to overwrite existing starter files unless `--force` is set

**Success contract**:

- starter files reflect the currently recommended narrow authored shape
- emitted starter content remains consistent with the ratified first slice once the grammar is frozen

## Command: `lumaui preview [project]`

**Purpose**: Reserved for a later milestone after generated-output flow is stable.

**Brownfield MVP contract**:

- the command exists but returns a clear deferred-message failure
- it must not imply a runtime interpreter or separate preview-only semantics

## Output Invariants

All CLI commands in this slice must preserve:

- deterministic discovery order
- deterministic output ordering
- source-located diagnostics where available
- no browser-style hidden behavior
- no backend inference that compensates for unresolved semantics

## Compatibility Notes

- The current baseline targets LVGL 9.x only.
- LVGL 8.x compatibility remains out of scope for the brownfield MVP slice.
- Binding evaluation, runtime reactivity, and asset-pipeline behavior are outside this contract until later phases ratify them.