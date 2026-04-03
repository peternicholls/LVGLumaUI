use lumaui_ir::{Project, Screen, Widget, WidgetKind};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GeneratedFile {
    pub path: String,
    pub contents: String,
}

pub fn generate_files(project: &Project) -> Vec<GeneratedFile> {
    let mut screens = project.screens.clone();
    screens.sort_by(|left, right| left.name.cmp(&right.name));

    let mut files = Vec::new();

    for screen in screens {
        let slug = slugify(&screen.name);
        files.push(GeneratedFile {
            path: format!("generated/ui/screens/{}_gen.h", slug),
            contents: render_header(&slug),
        });
        files.push(GeneratedFile {
            path: format!("generated/ui/screens/{}_gen.c", slug),
            contents: render_source(&slug, &screen),
        });
    }

    files
}

fn render_header(slug: &str) -> String {
    let guard = format!("LUMAUI_{}_GEN_H", slug.to_ascii_uppercase());

    format!(
        "#ifndef {guard}\n#define {guard}\n\n#include <lvgl.h>\n\nlv_obj_t *lumaui_screen_{slug}_create(void);\n\n#endif\n"
    )
}

fn render_source(slug: &str, screen: &Screen) -> String {
    let mut lines = vec![
        format!("#include \"{}_gen.h\"", slug),
        String::new(),
        format!("lv_obj_t *lumaui_screen_{slug}_create(void) {{"),
        "    lv_obj_t *screen = lv_obj_create(NULL);".to_string(),
    ];

    let mut counter = 0usize;
    emit_widget(&screen.root, "screen", &mut counter, &mut lines);
    lines.push(String::new());
    lines.push("    return screen;".to_string());
    lines.push("}".to_string());
    lines.push(String::new());

    lines.join("\n")
}

fn emit_widget(widget: &Widget, parent_name: &str, counter: &mut usize, lines: &mut Vec<String>) {
    *counter += 1;
    let variable_name = widget_variable_name(widget, *counter);
    let constructor = constructor_name(widget.kind);

    lines.push(format!(
        "    lv_obj_t *{variable_name} = {constructor}({parent_name});"
    ));

    match widget.kind {
        WidgetKind::Row => {
            lines.push(format!(
                "    lv_obj_set_layout({variable_name}, LV_LAYOUT_FLEX);"
            ));
            lines.push(format!(
                "    lv_obj_set_flex_flow({variable_name}, LV_FLEX_FLOW_ROW);"
            ));
        }
        WidgetKind::Column => {
            lines.push(format!(
                "    lv_obj_set_layout({variable_name}, LV_LAYOUT_FLEX);"
            ));
            lines.push(format!(
                "    lv_obj_set_flex_flow({variable_name}, LV_FLEX_FLOW_COLUMN);"
            ));
        }
        WidgetKind::Grid => {
            lines.push(format!(
                "    lv_obj_set_layout({variable_name}, LV_LAYOUT_GRID);"
            ));
        }
        WidgetKind::Card => {
            lines.push(format!(
                "    /* TODO: apply card style preset for {variable_name}. */"
            ));
        }
        _ => {}
    }

    if matches!(widget.kind, WidgetKind::Text) {
        if let Some(text) = &widget.text {
            lines.push(format!(
                "    lv_label_set_text({variable_name}, \"{}\");",
                escape_c_string(text)
            ));
        }
    }

    for child in &widget.children {
        emit_widget(child, &variable_name, counter, lines);
    }
}

fn constructor_name(kind: WidgetKind) -> &'static str {
    match kind {
        WidgetKind::Container
        | WidgetKind::Row
        | WidgetKind::Column
        | WidgetKind::Grid
        | WidgetKind::Card => "lv_obj_create",
        WidgetKind::Text => "lv_label_create",
        WidgetKind::Button => "lv_button_create",
        WidgetKind::Image => "lv_image_create",
    }
}

fn widget_variable_name(widget: &Widget, counter: usize) -> String {
    if let Some(id) = &widget.id {
        return slugify(id);
    }

    format!("{}_{}", kind_slug(widget.kind), counter)
}

fn kind_slug(kind: WidgetKind) -> &'static str {
    match kind {
        WidgetKind::Container => "container",
        WidgetKind::Row => "row",
        WidgetKind::Column => "column",
        WidgetKind::Grid => "grid",
        WidgetKind::Text => "text",
        WidgetKind::Button => "button",
        WidgetKind::Image => "image",
        WidgetKind::Card => "card",
    }
}

fn slugify(input: &str) -> String {
    let mut output = String::new();

    for ch in input.chars() {
        if ch.is_ascii_alphanumeric() {
            output.push(ch.to_ascii_lowercase());
        } else if !output.ends_with('_') {
            output.push('_');
        }
    }

    output.trim_matches('_').to_string()
}

fn escape_c_string(value: &str) -> String {
    value.replace('\\', "\\\\").replace('"', "\\\"")
}
