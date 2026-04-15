# Minimal Example

This example is intentionally small and acts as both documentation and the normative fixture for the ratified first slice.

The authored files here are the expected pass path for the active slice. They should stay inside the ratified widget, selector, property, and event-reference surface so `doctor`, `validate`, starter-template regression checks, and later backend snapshots can use them as the repository's trusted baseline.

The screen fixture intentionally exercises the ratified MVP event shape through `onPress="open_settings"` on the button while keeping the rest of the tree minimal.
