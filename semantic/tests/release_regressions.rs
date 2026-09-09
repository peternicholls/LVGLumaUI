use lumaui_parser::parse_document;
use lumaui_semantic::{analyze_documents, AnalysisInput};
use std::path::Path;

fn input() -> AnalysisInput {
    AnalysisInput {
        project_name: "test".into(),
        symbol_prefix: "lumaui_".into(),
    }
}

#[test]
fn nested_screens_are_rejected() {
    let doc = parse_document(
        Path::new("nested.lui"),
        "<Screen><Screen><Row/></Screen></Screen>",
    )
    .unwrap();
    let out = analyze_documents(input(), &[doc]);
    assert!(out.has_errors());
    assert!(out.project.is_none());
}

#[test]
fn no_markup_is_not_a_valid_project() {
    assert!(analyze_documents(input(), &[]).has_errors());
}

#[test]
fn invalid_symbol_prefix_is_rejected() {
    let doc = parse_document(Path::new("home.lui"), "<Screen><Row/></Screen>").unwrap();
    let mut config = input();
    config.symbol_prefix = "9bad-".into();
    assert!(analyze_documents(config, &[doc]).has_errors());
}

#[test]
fn dimensions_cannot_overflow_lvgl_coordinates() {
    let doc = parse_document(
        Path::new("home.lui"),
        "<Screen><Row class=\"root\"/></Screen>",
    )
    .unwrap();
    let style = parse_document(Path::new("theme.lus"), ".root { width: 4294967295; }").unwrap();
    assert!(analyze_documents(input(), &[doc, style]).has_errors());
}

#[test]
fn later_style_rules_override_earlier_id_rules() {
    let doc = parse_document(
        Path::new("home.lui"),
        "<Screen><Row id=\"row\" class=\"root\"/></Screen>",
    )
    .unwrap();
    let style = parse_document(
        Path::new("theme.lus"),
        "#row { width: 1; } .root { width: 2; width: 3; }",
    )
    .unwrap();
    let out = analyze_documents(input(), &[doc, style]);
    assert_eq!(
        out.project.unwrap().screens[0].root.children[0]
            .applied_styles
            .width,
        Some(3)
    );
}
