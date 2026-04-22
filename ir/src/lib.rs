//! Canonical, backend-facing model of a compiled LumaUI project.
//!
//! The IR is intentionally narrow: only the ratified first slice is
//! representable. New constructs land here only after a decision brief and a
//! ratified language-spec bump.

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Project {
    pub project_name: String,
    pub symbol_prefix: String,
    pub screens: Vec<Screen>,
}

impl Project {
    pub fn new(project_name: impl Into<String>, symbol_prefix: impl Into<String>) -> Self {
        Self {
            project_name: project_name.into(),
            symbol_prefix: symbol_prefix.into(),
            screens: Vec::new(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Screen {
    pub name: String,
    pub root: Widget,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Widget {
    pub kind: WidgetKind,
    pub id: Option<String>,
    pub class: Option<String>,
    pub text: Option<String>,
    pub event_press: Option<String>,
    pub applied_styles: AppliedStyles,
    pub children: Vec<Widget>,
}

impl Widget {
    pub fn new(kind: WidgetKind) -> Self {
        Self {
            kind,
            id: None,
            class: None,
            text: None,
            event_press: None,
            applied_styles: AppliedStyles::default(),
            children: Vec::new(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WidgetKind {
    Screen,
    Column,
    Row,
    Text,
    Button,
}

impl WidgetKind {
    pub fn as_str(self) -> &'static str {
        match self {
            WidgetKind::Screen => "Screen",
            WidgetKind::Column => "Column",
            WidgetKind::Row => "Row",
            WidgetKind::Text => "Text",
            WidgetKind::Button => "Button",
        }
    }
}

/// Resolved style values applied to a single widget after selector application.
///
/// The semantic stage performs the resolution in deterministic order so that
/// later rules win over earlier rules on the same property.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct AppliedStyles {
    pub padding: Option<u32>,
    pub width: Option<u32>,
    pub height: Option<u32>,
    pub background_color: Option<HexColor>,
    pub text_color: Option<HexColor>,
}

impl AppliedStyles {
    pub fn is_empty(&self) -> bool {
        self.padding.is_none()
            && self.width.is_none()
            && self.height.is_none()
            && self.background_color.is_none()
            && self.text_color.is_none()
    }
}

/// `#rrggbb` style hex color.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HexColor(pub String);

impl HexColor {
    pub fn lvgl_hex_literal(&self) -> String {
        // Strip leading `#`, prepend `0x`.
        debug_assert!(self.0.starts_with('#') && self.0.len() == 7);
        let body = &self.0[1..];
        format!("0x{body}")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hex_color_to_lvgl_literal() {
        assert_eq!(HexColor("#20242b".into()).lvgl_hex_literal(), "0x20242b");
    }

    #[test]
    fn applied_styles_is_empty_by_default() {
        assert!(AppliedStyles::default().is_empty());
    }
}
