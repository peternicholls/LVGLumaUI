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
    pub id: Option<String>,
    pub classes: Vec<String>,
    pub attributes: Vec<Attribute>,
    pub children: Vec<WidgetNode>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Attribute {
    pub name: String,
    pub value: AttributeValue,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AttributeValue {
    String(String),
    Reference(String),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StyleRule {
    pub selector: String,
    pub declarations: Vec<Declaration>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Declaration {
    pub name: String,
    pub value: String,
}
