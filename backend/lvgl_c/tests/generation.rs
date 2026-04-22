//! End-to-end snapshot generation tests for the LVGL C backend.
//!
//! These tests build the IR by hand to keep the backend boundary honest.
//! Frontend-driven snapshot coverage lives in the integration tests under
//! `tests/` at the workspace root.

use lumaui_backend_lvgl_c::{generate_files, GeneratedFile};
use lumaui_ir::{AppliedStyles, HexColor, Project, Screen, Widget, WidgetKind};

fn find<'a>(files: &'a [GeneratedFile], path: &str) -> &'a GeneratedFile {
    files
        .iter()
        .find(|f| f.path == path)
        .unwrap_or_else(|| panic!("expected generated file `{path}`"))
}

#[test]
fn minimal_screen_matches_snapshot() {
    let mut project = Project::new("minimal", "lumaui_");
    let title = {
        let mut w = Widget::new(WidgetKind::Text);
        w.id = Some("title".into());
        w.text = Some("Hello LumaUI".into());
        w.applied_styles = AppliedStyles {
            text_color: Some(HexColor("#f5f7fa".into())),
            ..Default::default()
        };
        w
    };

    let inner_label = {
        let mut w = Widget::new(WidgetKind::Text);
        w.text = Some("Settings".into());
        w
    };

    let button = {
        let mut w = Widget::new(WidgetKind::Button);
        w.id = Some("openSettings".into());
        w.event_press = Some("open_settings".into());
        w.children.push(inner_label);
        w
    };

    let column = {
        let mut w = Widget::new(WidgetKind::Column);
        w.class = Some("root".into());
        w.applied_styles = AppliedStyles {
            padding: Some(16),
            background_color: Some(HexColor("#20242b".into())),
            ..Default::default()
        };
        w.children.push(title);
        w.children.push(button);
        w
    };

    let mut root = Widget::new(WidgetKind::Screen);
    root.id = Some("home".into());
    root.children.push(column);

    project.screens.push(Screen {
        name: "home".into(),
        root,
    });

    let files = generate_files(&project);
    let header = find(&files, "screens/home_gen.h");
    let source = find(&files, "screens/home_gen.c");

    let expected_header = include_str!("../../../tests/snapshots/minimal_screen.h");
    let expected_source = include_str!("../../../tests/snapshots/minimal_screen.c");

    assert_eq!(
        header.contents,
        expected_header,
        "backend header snapshot drifted; update tests/snapshots/minimal_screen.h if the emitter change is intentional"
    );
    assert_eq!(
        source.contents,
        expected_source,
        "backend source snapshot drifted; update tests/snapshots/minimal_screen.c if the emitter change is intentional"
    );
}

#[test]
fn empty_project_emits_no_files() {
    let project = Project::new("empty", "lumaui_");
    assert!(generate_files(&project).is_empty());
}
