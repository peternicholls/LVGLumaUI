//! AST types produced by the LumaUI parser.
//!
//! The AST is syntax-facing: it preserves spans and authored attribute names so
//! the semantic layer can decide what is in or out of the ratified slice.

use lumaui_compiler::Span;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DocumentKind {
    Markup,
    Style,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Document {
    pub source_name: String,
    pub kind: DocumentKind,
    pub items: Vec<TopLevel>,
}

impl Document {
    pub fn new(source_name: impl Into<String>, kind: DocumentKind) -> Self {
        Self {
            source_name: source_name.into(),
            kind,
            items: Vec::new(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TopLevel {
    Widget(WidgetNode),
    StyleRule(StyleRule),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WidgetNode {
    pub widget_type: String,
    pub attributes: Vec<Attribute>,
    pub children: Vec<WidgetNode>,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Attribute {
    pub name: String,
    pub value: AttributeValue,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AttributeValue {
    String(String),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StyleRule {
    pub selector: Selector,
    pub declarations: Vec<Declaration>,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Selector {
    Class(String),
    Id(String),
    /// Anything else parsed but not in the ratified surface; rejected by the
    /// semantic layer with a source-located diagnostic.
    Unsupported(String),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Declaration {
    pub name: String,
    pub value: DeclarationValue,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DeclarationValue {
    Number(u32),
    HexColor(String),
}
