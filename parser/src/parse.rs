//! Recursive-descent parser for LumaUI markup and style documents.

use crate::ast::{
    Attribute, AttributeValue, Declaration, DeclarationValue, Document, DocumentKind, Selector,
    StyleRule, TopLevel, WidgetNode,
};
use crate::lexer::{lex, Token, TokenKind};
use lumaui_compiler::{Diagnostic, Span};
use std::path::Path;

pub fn parse_document(path: &Path, source: &str) -> Result<Document, Vec<Diagnostic>> {
    let kind = classify_kind(path);
    let source_name = path
        .file_name()
        .map(|s| s.to_string_lossy().into_owned())
        .unwrap_or_else(|| path.display().to_string());
    let tokens = lex(path, source)?;
    let mut parser = Parser::new(path, tokens, source_name, kind);
    parser.parse();
    if parser.diagnostics.is_empty() {
        Ok(parser.document)
    } else {
        Err(parser.diagnostics)
    }
}

fn classify_kind(path: &Path) -> DocumentKind {
    match path.extension().and_then(|s| s.to_str()) {
        Some("lus") => DocumentKind::Style,
        _ => DocumentKind::Markup,
    }
}

struct Parser<'a> {
    file: &'a Path,
    tokens: Vec<Token>,
    cursor: usize,
    diagnostics: Vec<Diagnostic>,
    document: Document,
}

impl<'a> Parser<'a> {
    fn new(file: &'a Path, tokens: Vec<Token>, source_name: String, kind: DocumentKind) -> Self {
        Self {
            file,
            tokens,
            cursor: 0,
            diagnostics: Vec::new(),
            document: Document::new(source_name, kind),
        }
    }

    fn parse(&mut self) {
        match self.document.kind {
            DocumentKind::Markup => self.parse_markup_document(),
            DocumentKind::Style => self.parse_style_document(),
        }
    }

    // ---------- helpers ----------

    fn peek(&self) -> Option<&Token> {
        self.tokens.get(self.cursor)
    }

    fn advance(&mut self) -> Option<Token> {
        let t = self.tokens.get(self.cursor).cloned();
        if t.is_some() {
            self.cursor += 1;
        }
        t
    }

    fn expect(&mut self, kind: TokenKind, label: &str) -> Option<Token> {
        match self.peek() {
            Some(t) if t.kind == kind => self.advance(),
            Some(t) => {
                let span = t.span;
                let lex = t.lexeme.clone();
                self.error(format!("expected {label}, found `{lex}`"), span, None);
                None
            }
            None => {
                let span = self.last_span();
                self.error(
                    format!("expected {label} but reached end of file"),
                    span,
                    None,
                );
                None
            }
        }
    }

    fn last_span(&self) -> Span {
        self.tokens.last().map(|t| t.span).unwrap_or(Span {
            line: 1,
            column: 1,
            length: 0,
        })
    }

    fn error(&mut self, message: impl Into<String>, span: Span, hint: Option<&str>) {
        let mut d = Diagnostic::error(message)
            .with_file(self.file.to_path_buf())
            .with_span(span);
        if let Some(h) = hint {
            d = d.with_hint(h);
        }
        self.diagnostics.push(d);
    }

    // ---------- markup ----------

    fn parse_markup_document(&mut self) {
        while self.peek().is_some() {
            match self.parse_widget() {
                Some(w) => self.document.items.push(TopLevel::Widget(w)),
                None => {
                    // attempt to recover by skipping to next `<`
                    self.advance();
                }
            }
        }
    }

    fn parse_widget(&mut self) -> Option<WidgetNode> {
        let open = self.expect(TokenKind::OpenAngle, "`<`")?;
        let name_tok = self.expect(TokenKind::Identifier, "widget name")?;
        let widget_type = name_tok.lexeme.clone();
        let mut attributes = Vec::new();

        loop {
            match self.peek().map(|t| t.kind.clone()) {
                Some(TokenKind::Identifier) => {
                    let attr = self.parse_attribute()?;
                    attributes.push(attr);
                }
                Some(TokenKind::Slash) => {
                    self.advance();
                    self.expect(TokenKind::CloseAngle, "`>` to close self-closing tag")?;
                    return Some(WidgetNode {
                        widget_type,
                        attributes,
                        children: Vec::new(),
                        span: open.span,
                    });
                }
                Some(TokenKind::CloseAngle) => {
                    self.advance();
                    let children = self.parse_children()?;
                    self.expect(TokenKind::OpenAngle, "`<` to begin closing tag")?;
                    self.expect(TokenKind::Slash, "`/` in closing tag")?;
                    let close_name = self.expect(TokenKind::Identifier, "closing tag name")?;
                    if close_name.lexeme != widget_type {
                        let span = close_name.span;
                        self.error(
                            format!(
                                "closing tag `{}` does not match opening tag `{}`",
                                close_name.lexeme, widget_type
                            ),
                            span,
                            Some("opening and closing tag names must match exactly"),
                        );
                    }
                    self.expect(TokenKind::CloseAngle, "`>` to close end tag")?;
                    return Some(WidgetNode {
                        widget_type,
                        attributes,
                        children,
                        span: open.span,
                    });
                }
                Some(_) => {
                    let t = self.peek().cloned().unwrap();
                    self.error(
                        format!(
                            "expected attribute, `/>`, or `>` in `<{}>`, found `{}`",
                            widget_type, t.lexeme
                        ),
                        t.span,
                        None,
                    );
                    return None;
                }
                None => {
                    let span = self.last_span();
                    self.error(format!("unterminated `<{}>` tag", widget_type), span, None);
                    return None;
                }
            }
        }
    }

    fn parse_attribute(&mut self) -> Option<Attribute> {
        let name_tok = self.expect(TokenKind::Identifier, "attribute name")?;
        self.expect(TokenKind::Equals, "`=` after attribute name")?;
        let value_tok = self.expect(TokenKind::StringLiteral, "string literal attribute value")?;
        Some(Attribute {
            name: name_tok.lexeme.clone(),
            value: AttributeValue::String(value_tok.lexeme),
            span: name_tok.span,
        })
    }

    fn parse_children(&mut self) -> Option<Vec<WidgetNode>> {
        let mut children = Vec::new();
        loop {
            match (
                self.peek().map(|t| t.kind.clone()),
                self.tokens.get(self.cursor + 1).map(|t| t.kind.clone()),
            ) {
                (Some(TokenKind::OpenAngle), Some(TokenKind::Slash)) => return Some(children),
                (Some(TokenKind::OpenAngle), _) => {
                    let w = self.parse_widget()?;
                    children.push(w);
                }
                (None, _) => {
                    let span = self.last_span();
                    self.error("unexpected end of file inside element body", span, None);
                    return None;
                }
                (Some(_), _) => {
                    let t = self.peek().cloned().unwrap();
                    self.error(
                        format!(
                            "expected child element or closing tag, found `{}`",
                            t.lexeme
                        ),
                        t.span,
                        None,
                    );
                    return None;
                }
            }
        }
    }

    // ---------- style ----------

    fn parse_style_document(&mut self) {
        while self.peek().is_some() {
            match self.parse_rule() {
                Some(rule) => self.document.items.push(TopLevel::StyleRule(rule)),
                None => {
                    self.advance();
                }
            }
        }
    }

    fn parse_rule(&mut self) -> Option<StyleRule> {
        let (selector, span) = self.parse_selector()?;
        self.expect(TokenKind::OpenBrace, "`{` to begin style block")?;
        let mut declarations = Vec::new();
        loop {
            match self.peek().map(|t| t.kind.clone()) {
                Some(TokenKind::CloseBrace) => {
                    self.advance();
                    return Some(StyleRule {
                        selector,
                        declarations,
                        span,
                    });
                }
                Some(TokenKind::Identifier) => {
                    if let Some(decl) = self.parse_declaration() {
                        declarations.push(decl);
                    } else {
                        return None;
                    }
                }
                Some(_) => {
                    let t = self.peek().cloned().unwrap();
                    self.error(
                        format!("expected property or `}}`, found `{}`", t.lexeme),
                        t.span,
                        None,
                    );
                    return None;
                }
                None => {
                    let span = self.last_span();
                    self.error("unterminated style block", span, None);
                    return None;
                }
            }
        }
    }

    fn parse_selector(&mut self) -> Option<(Selector, Span)> {
        let head = self.peek().cloned()?;
        match head.kind {
            TokenKind::Dot => {
                self.advance();
                let name = self.expect(TokenKind::Identifier, "class selector name")?;
                Some((Selector::Class(name.lexeme), head.span))
            }
            TokenKind::Hash => {
                self.advance();
                let name = self.expect(TokenKind::Identifier, "id selector name")?;
                Some((Selector::Id(name.lexeme), head.span))
            }
            TokenKind::Identifier => {
                self.advance();
                Some((Selector::Unsupported(head.lexeme.clone()), head.span))
            }
            _ => {
                self.error(
                    format!("expected selector, found `{}`", head.lexeme),
                    head.span,
                    Some("ratified selectors are `.class` and `#id`"),
                );
                None
            }
        }
    }

    fn parse_declaration(&mut self) -> Option<Declaration> {
        let first = self.expect(TokenKind::Identifier, "property name")?;
        let mut name = first.lexeme.clone();
        let span = first.span;
        // Allow `background-color`-style hyphenated property names.
        while self
            .peek()
            .map(|t| t.kind == TokenKind::Hyphen)
            .unwrap_or(false)
        {
            self.advance();
            let part = self.expect(TokenKind::Identifier, "property name continuation")?;
            name.push('-');
            name.push_str(&part.lexeme);
        }
        self.expect(TokenKind::Colon, "`:` after property name")?;
        let value_tok = self.advance().or_else(|| {
            let span = self.last_span();
            self.error("expected property value", span, None);
            None
        })?;
        let value = match value_tok.kind {
            TokenKind::Number => {
                let n = value_tok.lexeme.parse::<u32>().map_err(|_| ()).ok();
                match n {
                    Some(n) => DeclarationValue::Number(n),
                    None => {
                        self.error(
                            format!(
                                "number `{}` does not fit in a 32-bit unsigned value",
                                value_tok.lexeme
                            ),
                            value_tok.span,
                            None,
                        );
                        return None;
                    }
                }
            }
            TokenKind::HexColor => DeclarationValue::HexColor(value_tok.lexeme.clone()),
            _ => {
                self.error(
                    format!(
                        "expected a number or hex color, found `{}`",
                        value_tok.lexeme
                    ),
                    value_tok.span,
                    Some("ratified style values are integer pixels or `#rrggbb` colors"),
                );
                return None;
            }
        };
        self.expect(TokenKind::Semicolon, "`;` to end declaration")?;
        Some(Declaration { name, value, span })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::{AttributeValue, DeclarationValue, Selector, TopLevel};
    use std::path::PathBuf;

    fn parse_lui(src: &str) -> Document {
        parse_document(&PathBuf::from("test.lui"), src).expect("expected successful parse")
    }

    fn parse_lus(src: &str) -> Document {
        parse_document(&PathBuf::from("test.lus"), src).expect("expected successful parse")
    }

    #[test]
    fn parses_self_closing_widget() {
        let doc = parse_lui("<Screen id=\"home\"/>");
        assert_eq!(doc.items.len(), 1);
        match &doc.items[0] {
            TopLevel::Widget(w) => {
                assert_eq!(w.widget_type, "Screen");
                assert_eq!(w.attributes.len(), 1);
                assert_eq!(w.attributes[0].name, "id");
                match &w.attributes[0].value {
                    AttributeValue::String(s) => assert_eq!(s, "home"),
                }
                assert!(w.children.is_empty());
            }
            other => panic!("unexpected top-level item: {other:?}"),
        }
    }

    #[test]
    fn parses_nested_widgets_with_attributes() {
        let doc = parse_lui(
            "<Screen id=\"home\"><Column class=\"root\"><Text id=\"title\" text=\"Hi\"/></Column></Screen>",
        );
        let TopLevel::Widget(screen) = &doc.items[0] else {
            panic!("expected widget");
        };
        assert_eq!(screen.children.len(), 1);
        let column = &screen.children[0];
        assert_eq!(column.widget_type, "Column");
        assert_eq!(column.attributes[0].name, "class");
        assert_eq!(column.children[0].widget_type, "Text");
    }

    #[test]
    fn rejects_mismatched_closing_tag() {
        let err =
            parse_document(&PathBuf::from("t.lui"), "<Screen><Column></Row></Screen>").unwrap_err();
        assert!(err.iter().any(|d| d.message.contains("does not match")));
    }

    #[test]
    fn parses_class_and_id_selectors() {
        let doc = parse_lus(".root { padding: 16; } #title { text-color: #20242b; }");
        assert_eq!(doc.items.len(), 2);
        let TopLevel::StyleRule(r1) = &doc.items[0] else {
            panic!()
        };
        let TopLevel::StyleRule(r2) = &doc.items[1] else {
            panic!()
        };
        assert!(matches!(r1.selector, Selector::Class(ref s) if s == "root"));
        assert!(matches!(r2.selector, Selector::Id(ref s) if s == "title"));
        assert_eq!(r1.declarations[0].name, "padding");
        assert!(matches!(
            r1.declarations[0].value,
            DeclarationValue::Number(16)
        ));
        assert_eq!(r2.declarations[0].name, "text-color");
        assert!(
            matches!(r2.declarations[0].value, DeclarationValue::HexColor(ref c) if c == "#20242b")
        );
    }

    #[test]
    fn captures_unsupported_selector_for_semantic_rejection() {
        let doc = parse_lus("Screen { padding: 4; }");
        let TopLevel::StyleRule(r) = &doc.items[0] else {
            panic!()
        };
        assert!(matches!(r.selector, Selector::Unsupported(ref s) if s == "Screen"));
    }

    #[test]
    fn rejects_invalid_property_value() {
        let err =
            parse_document(&PathBuf::from("t.lus"), ".root { padding: \"oops\"; }").unwrap_err();
        assert!(err
            .iter()
            .any(|d| d.message.contains("expected a number or hex color")));
    }
}
