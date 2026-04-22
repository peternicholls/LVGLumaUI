# Dashboard Example

Status: **aspirational** — this example is intentionally ahead of the ratified first slice. It exists as a forward-looking design target and is not exercised by CLI smoke runs or snapshot regression.

`lumaui validate` and `lumaui build` will reject this project today, by design.

## Support Status

- Out of scope for the active brownfield slice.
- Excluded from `scripts/lumaui-phase-check.sh`.
- No snapshot files are generated for this example.

## Ratified Constructs

None of the constructs in this example exceed what the minimal example already covers. Anything that overlaps (`Screen`, `Column`, `Row`, `Text`, `Button`, class/id selectors, basic style properties) is supported only in the minimal slice context.

## Deferred Constructs

The dashboard example uses constructs that remain deferred:

- Widgets: `Grid`, `Card`, `Image`
- Bindings: `bind="status_text"`, `source="metrics_chart"`
- Implied richer layout/styling expectations (multi-card grids, image assets)

These will move from aspirational to normative only after each construct is ratified in `docs/LANGUAGE_SPEC.md` and added to the supported surface.
