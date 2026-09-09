use lumaui_parser::{parse_document, Selector, TopLevel};
use std::path::Path;

#[test]
fn hex_shaped_identifier_is_an_id_selector() {
    let doc = parse_document(Path::new("theme.lus"), "#abcdef { width: 10; }").unwrap();
    let TopLevel::StyleRule(rule) = &doc.items[0] else {
        panic!()
    };
    assert_eq!(rule.selector, Selector::Id("abcdef".into()));
}

#[test]
fn semantic_source_paths_are_not_discarded() {
    let path = Path::new("project/ui/screens/home.lui");
    let doc = parse_document(path, "<Screen><Row/></Screen>").unwrap();
    assert_eq!(doc.source_name, path.display().to_string());
}

#[test]
fn carriage_return_cannot_split_a_string_literal() {
    assert!(parse_document(Path::new("bad.lui"), "<Text text=\"a\rb\"/>").is_err());
}

#[test]
fn deeply_nested_input_reports_an_error_instead_of_overflowing_stack() {
    let source = format!("{}{}", "<Row>".repeat(200), "</Row>".repeat(200));
    let errors = parse_document(Path::new("deep.lui"), &source).unwrap_err();
    assert!(errors.iter().any(|e| e.message.contains("128-level")));
}
