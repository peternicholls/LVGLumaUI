use lumaui_parser::{parse_document, DocumentKind, TopLevel};
use std::path::Path;

#[test]
fn parses_normative_tree_and_styles() {
    let parsed = parse_document(
        Path::new("home.lui"),
        include_str!("../../examples/minimal/ui/screens/home.lui"),
        DocumentKind::Markup,
    )
    .unwrap();
    assert!(parsed.diagnostics.is_empty());
    let TopLevel::Widget(root) = &parsed.document.items[0] else {
        panic!()
    };
    assert_eq!(root.widget_type, "Screen");
    assert_eq!(root.children[0].children.len(), 2);
    let parsed = parse_document(
        Path::new("theme.lus"),
        include_str!("../../examples/minimal/ui/styles/theme.lus"),
        DocumentKind::Style,
    )
    .unwrap();
    assert_eq!(parsed.document.items.len(), 2);
}

#[test]
fn malformed_syntax_has_location() {
    for input in [
        "",
        "<Screen>",
        "<Screen><Text/></Row>",
        "<Text text=Hello/>",
        "<Screen id=\"lost",
        "<Screen/><Screen/>",
        "<Screen>hello</Screen>",
        "<Screen id=\"x\" id=\"y\"/>",
    ] {
        let errors = parse_document(Path::new("bad.lui"), input, DocumentKind::Markup).unwrap_err();
        assert!(errors[0].span.is_some(), "{input}");
    }
    for input in [".a { width: ; }", ".a { width: 10", ".a width: 10; }"] {
        assert!(parse_document(Path::new("bad.lus"), input, DocumentKind::Style).is_err());
    }
}

#[test]
fn unsupported_but_well_formed_input_reaches_semantics() {
    for input in [
        ".a { width: 50%; }",
        "#face-title { margin: 2px; }",
        ".a .b { width: calc(100% - 4); }",
        "Text:hover { padding: 1 2; }",
    ] {
        assert!(
            parse_document(Path::new("style.lus"), input, DocumentKind::Style).is_ok(),
            "{input}"
        );
    }
    assert!(parse_document(
        Path::new("ui.lui"),
        "<Screen><Grid bind=\"value\"/></Screen>",
        DocumentKind::Markup
    )
    .is_ok());
}
