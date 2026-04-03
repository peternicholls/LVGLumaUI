use lumaui_backend_lvgl_c::generate_files;
use lumaui_ir::{Project, Screen, Widget, WidgetKind};

#[test]
fn generates_stable_minimal_screen_output() {
    let project = Project::new("minimal").with_screen(Screen {
        name: "home".to_string(),
        root: Widget::new(WidgetKind::Column)
            .with_id("root")
            .with_child(
                Widget::new(WidgetKind::Text)
                    .with_id("title")
                    .with_text("Hello LumaUI"),
            )
            .with_child(
                Widget::new(WidgetKind::Button)
                    .with_id("open_settings")
                    .with_child(Widget::new(WidgetKind::Text).with_text("Settings")),
            ),
    });

    let files = generate_files(&project);
    let generated = files
        .iter()
        .find(|file| file.path.ends_with("home_gen.c"))
        .expect("expected generated screen source");

    let expected = include_str!("../../../tests/snapshots/minimal_screen.c");
    assert_eq!(generated.contents, expected);
}
