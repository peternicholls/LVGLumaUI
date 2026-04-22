//! Semantic stage: validate authored documents against the ratified first
//! slice and lower them into the canonical IR.

use lumaui_compiler::{Diagnostic, Severity, Span};
use lumaui_ir::{AppliedStyles, HexColor, Project, Screen, Widget, WidgetKind};
use lumaui_parser::{
    Attribute, AttributeValue, Declaration, DeclarationValue, Document, DocumentKind, Selector,
    StyleRule, TopLevel, WidgetNode,
};
use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

const RATIFIED_WIDGETS: &[&str] = &["Screen", "Column", "Row", "Text", "Button"];
const RATIFIED_STYLE_PROPS: &[&str] = &[
    "padding",
    "background-color",
    "text-color",
    "width",
    "height",
];

#[derive(Debug, Clone)]
pub struct AnalysisOutcome {
    pub project: Option<Project>,
    pub diagnostics: Vec<Diagnostic>,
}

impl AnalysisOutcome {
    pub fn has_errors(&self) -> bool {
        self.diagnostics
            .iter()
            .any(|d| matches!(d.severity, Severity::Error))
    }
}

#[derive(Debug, Clone, Default)]
pub struct AnalysisInput {
    pub project_name: String,
    pub symbol_prefix: String,
}

pub fn analyze_documents(input: AnalysisInput, documents: &[Document]) -> AnalysisOutcome {
    let mut diagnostics = Vec::new();

    // Split into markup screens and style rules in deterministic order.
    let mut screen_docs: Vec<&Document> = Vec::new();
    let mut style_docs: Vec<&Document> = Vec::new();
    for d in documents {
        match d.kind {
            DocumentKind::Markup => screen_docs.push(d),
            DocumentKind::Style => style_docs.push(d),
        }
    }

    let style_rules = collect_style_rules(&style_docs, &mut diagnostics);

    let mut screens = Vec::new();
    let mut id_locations: BTreeMap<String, (PathBuf, Span)> = BTreeMap::new();

    for doc in &screen_docs {
        let screen_path = PathBuf::from(&doc.source_name);
        let widgets: Vec<&WidgetNode> = doc
            .items
            .iter()
            .filter_map(|item| match item {
                TopLevel::Widget(w) => Some(w),
                _ => None,
            })
            .collect();

        if widgets.is_empty() {
            diagnostics.push(
                Diagnostic::error("markup document has no `Screen` element")
                    .with_file(screen_path.clone())
                    .with_hint("each `.lui` file must define exactly one `Screen` element"),
            );
            continue;
        }
        if widgets.len() > 1 {
            diagnostics.push(
                Diagnostic::error("markup document defines more than one root element")
                    .with_file(screen_path.clone())
                    .with_span(widgets[1].span)
                    .with_hint("each `.lui` file must define exactly one root `Screen` element"),
            );
        }

        let root = widgets[0];
        if root.widget_type != "Screen" {
            diagnostics.push(
                Diagnostic::error(format!(
                    "root element must be `Screen`, found `{}`",
                    root.widget_type
                ))
                .with_file(screen_path.clone())
                .with_span(root.span),
            );
            continue;
        }

        let mut ctx = LowerCtx {
            file: screen_path.clone(),
            diagnostics: &mut diagnostics,
            id_locations: &mut id_locations,
            style_rules: &style_rules,
        };
        let Some(root_widget) = ctx.lower_widget(root) else {
            continue;
        };

        let screen_name = derive_screen_name(&doc.source_name);
        screens.push(Screen {
            name: screen_name,
            root: root_widget,
        });
    }

    sort_diagnostics(&mut diagnostics);

    let project = if diagnostics
        .iter()
        .any(|d| matches!(d.severity, Severity::Error))
    {
        None
    } else {
        let mut p = Project::new(input.project_name, input.symbol_prefix);
        p.screens = screens;
        Some(p)
    };

    AnalysisOutcome {
        project,
        diagnostics,
    }
}

fn derive_screen_name(source_name: &str) -> String {
    let stem = std::path::Path::new(source_name)
        .file_stem()
        .map(|s| s.to_string_lossy().into_owned())
        .unwrap_or_else(|| source_name.to_string());
    stem
}

fn sort_diagnostics(diagnostics: &mut [Diagnostic]) {
    diagnostics.sort_by(|a, b| {
        let a_file = a
            .file
            .as_ref()
            .map(|p| p.to_string_lossy().into_owned())
            .unwrap_or_default();
        let b_file = b
            .file
            .as_ref()
            .map(|p| p.to_string_lossy().into_owned())
            .unwrap_or_default();
        let a_line = a.span.map(|s| s.line).unwrap_or(0);
        let b_line = b.span.map(|s| s.line).unwrap_or(0);
        let a_col = a.span.map(|s| s.column).unwrap_or(0);
        let b_col = b.span.map(|s| s.column).unwrap_or(0);
        (a_file, a_line, a_col).cmp(&(b_file, b_line, b_col))
    });
}

#[derive(Debug, Clone)]
struct ResolvedStyleRule {
    file: PathBuf,
    span: Span,
    selector: SelectorMatch,
    declarations: Vec<Declaration>,
}

#[derive(Debug, Clone)]
enum SelectorMatch {
    Class(String),
    Id(String),
}

fn collect_style_rules(
    style_docs: &[&Document],
    diagnostics: &mut Vec<Diagnostic>,
) -> Vec<ResolvedStyleRule> {
    let mut out = Vec::new();
    for doc in style_docs {
        let path = PathBuf::from(&doc.source_name);
        for item in &doc.items {
            let TopLevel::StyleRule(rule) = item else {
                continue;
            };
            let StyleRule {
                selector,
                declarations,
                span,
            } = rule;

            let matched = match selector {
                Selector::Class(name) => SelectorMatch::Class(name.clone()),
                Selector::Id(name) => SelectorMatch::Id(name.clone()),
                Selector::Unsupported(text) => {
                    diagnostics.push(
                        Diagnostic::error(format!(
                            "unsupported selector `{text}`; only `.class` and `#id` are ratified"
                        ))
                        .with_file(path.clone())
                        .with_span(*span)
                        .with_hint("see docs/LANGUAGE_SPEC.md → Style Grammar"),
                    );
                    continue;
                }
            };

            // Filter declarations to ratified properties.
            let mut kept = Vec::new();
            for decl in declarations {
                if !RATIFIED_STYLE_PROPS.contains(&decl.name.as_str()) {
                    diagnostics.push(
                        Diagnostic::error(format!(
                            "unsupported style property `{}`",
                            decl.name
                        ))
                        .with_file(path.clone())
                        .with_span(decl.span)
                        .with_hint(
                            "ratified properties: padding, background-color, text-color, width, height",
                        ),
                    );
                    continue;
                }
                if let Some(diag) = check_property_value_shape(&path, decl) {
                    diagnostics.push(diag);
                    continue;
                }
                kept.push(decl.clone());
            }

            out.push(ResolvedStyleRule {
                file: path.clone(),
                span: *span,
                selector: matched,
                declarations: kept,
            });
        }
    }
    out
}

fn check_property_value_shape(file: &Path, decl: &Declaration) -> Option<Diagnostic> {
    let want_color = matches!(decl.name.as_str(), "background-color" | "text-color");
    let want_number = matches!(decl.name.as_str(), "padding" | "width" | "height");
    match (&decl.value, want_color, want_number) {
        (DeclarationValue::HexColor(_), true, _) => None,
        (DeclarationValue::Number(_), _, true) => None,
        (DeclarationValue::Number(_), true, _) => Some(
            Diagnostic::error(format!(
                "property `{}` requires a hex color value",
                decl.name
            ))
            .with_file(file.to_path_buf())
            .with_span(decl.span),
        ),
        (DeclarationValue::HexColor(_), _, true) => Some(
            Diagnostic::error(format!(
                "property `{}` requires an integer pixel value",
                decl.name
            ))
            .with_file(file.to_path_buf())
            .with_span(decl.span),
        ),
        _ => None,
    }
}

struct LowerCtx<'a> {
    file: PathBuf,
    diagnostics: &'a mut Vec<Diagnostic>,
    id_locations: &'a mut BTreeMap<String, (PathBuf, Span)>,
    style_rules: &'a [ResolvedStyleRule],
}

impl<'a> LowerCtx<'a> {
    fn lower_widget(&mut self, node: &WidgetNode) -> Option<Widget> {
        let kind = match node.widget_type.as_str() {
            "Screen" => WidgetKind::Screen,
            "Column" => WidgetKind::Column,
            "Row" => WidgetKind::Row,
            "Text" => WidgetKind::Text,
            "Button" => WidgetKind::Button,
            other => {
                self.diagnostics.push(
                    Diagnostic::error(format!("unsupported widget `{other}`"))
                        .with_file(self.file.clone())
                        .with_span(node.span)
                        .with_hint(format!("ratified widgets: {}", RATIFIED_WIDGETS.join(", "))),
                );
                return None;
            }
        };

        let mut widget = Widget::new(kind);

        // Attributes.
        let mut id_seen = false;
        let mut class_seen = false;
        let mut text_seen = false;
        let mut event_press_seen = false;
        for attr in &node.attributes {
            match attr.name.as_str() {
                "id" => {
                    if id_seen {
                        self.diagnostics.push(self.duplicate_attr(attr));
                        continue;
                    }
                    id_seen = true;
                    let value = attr_string(attr);
                    if !is_identifier(&value) {
                        self.diagnostics.push(
                            Diagnostic::error(format!(
                                "`id` value `{value}` is not a valid identifier"
                            ))
                            .with_file(self.file.clone())
                            .with_span(attr.span),
                        );
                        continue;
                    }
                    if let Some((prev_file, prev_span)) = self
                        .id_locations
                        .insert(value.clone(), (self.file.clone(), attr.span))
                    {
                        self.diagnostics.push(
                            Diagnostic::error(format!("duplicate id `{value}`"))
                                .with_file(self.file.clone())
                                .with_span(attr.span)
                                .with_hint(format!(
                                    "previously defined in {} at line {}",
                                    prev_file.display(),
                                    prev_span.line
                                )),
                        );
                    }
                    widget.id = Some(value);
                }
                "class" => {
                    if class_seen {
                        self.diagnostics.push(self.duplicate_attr(attr));
                        continue;
                    }
                    class_seen = true;
                    let value = attr_string(attr);
                    if value.contains(' ') {
                        self.diagnostics.push(
                            Diagnostic::error(
                                "multiple classes are not ratified in the first slice",
                            )
                            .with_file(self.file.clone())
                            .with_span(attr.span)
                            .with_hint("use a single class identifier"),
                        );
                        continue;
                    }
                    if !is_identifier(&value) {
                        self.diagnostics.push(
                            Diagnostic::error(format!(
                                "`class` value `{value}` is not a valid identifier"
                            ))
                            .with_file(self.file.clone())
                            .with_span(attr.span),
                        );
                        continue;
                    }
                    widget.class = Some(value);
                }
                "text" => {
                    if kind != WidgetKind::Text {
                        self.diagnostics.push(
                            Diagnostic::error(format!(
                                "attribute `text` is only ratified on `Text`, not `{}`",
                                kind.as_str()
                            ))
                            .with_file(self.file.clone())
                            .with_span(attr.span),
                        );
                        continue;
                    }
                    if text_seen {
                        self.diagnostics.push(self.duplicate_attr(attr));
                        continue;
                    }
                    text_seen = true;
                    let value = attr_string(attr);
                    if value.is_empty() {
                        self.diagnostics.push(
                            Diagnostic::error("`text` attribute must not be empty")
                                .with_file(self.file.clone())
                                .with_span(attr.span),
                        );
                        continue;
                    }
                    widget.text = Some(value);
                }
                "onPress" => {
                    if kind != WidgetKind::Button {
                        self.diagnostics.push(
                            Diagnostic::error(format!(
                                "attribute `onPress` is only ratified on `Button`, not `{}`",
                                kind.as_str()
                            ))
                            .with_file(self.file.clone())
                            .with_span(attr.span),
                        );
                        continue;
                    }
                    if event_press_seen {
                        self.diagnostics.push(self.duplicate_attr(attr));
                        continue;
                    }
                    event_press_seen = true;
                    let value = attr_string(attr);
                    if !is_identifier(&value) {
                        self.diagnostics.push(
                            Diagnostic::error(format!(
                                "`onPress` handler `{value}` is not a valid identifier"
                            ))
                            .with_file(self.file.clone())
                            .with_span(attr.span),
                        );
                        continue;
                    }
                    widget.event_press = Some(value);
                }
                "bind" => {
                    self.diagnostics.push(
                        Diagnostic::error("bindings are not ratified in the first slice")
                            .with_file(self.file.clone())
                            .with_span(attr.span)
                            .with_hint(
                                "see docs/LANGUAGE_SPEC.md → Bindings (Explicitly Rejected)",
                            ),
                    );
                }
                other => {
                    self.diagnostics.push(
                        Diagnostic::error(format!("unsupported attribute `{other}`"))
                            .with_file(self.file.clone())
                            .with_span(attr.span)
                            .with_hint(
                                "ratified attributes: id, class, text (Text), onPress (Button)",
                            ),
                    );
                }
            }
        }

        // Required-attribute checks.
        if kind == WidgetKind::Text && widget.text.is_none() {
            self.diagnostics.push(
                Diagnostic::error("`Text` requires a `text` attribute")
                    .with_file(self.file.clone())
                    .with_span(node.span),
            );
        }

        // Children rules.
        match kind {
            WidgetKind::Screen if node.children.len() != 1 => {
                self.diagnostics.push(
                    Diagnostic::error(format!(
                        "`Screen` must contain exactly one child, found {}",
                        node.children.len()
                    ))
                    .with_file(self.file.clone())
                    .with_span(node.span),
                );
            }
            WidgetKind::Text if !node.children.is_empty() => {
                self.diagnostics.push(
                    Diagnostic::error("`Text` must not contain children")
                        .with_file(self.file.clone())
                        .with_span(node.span),
                );
            }
            WidgetKind::Button if node.children.len() > 1 => {
                self.diagnostics.push(
                    Diagnostic::error(format!(
                        "`Button` may contain at most one child, found {}",
                        node.children.len()
                    ))
                    .with_file(self.file.clone())
                    .with_span(node.span),
                );
            }
            _ => {}
        }

        for child in &node.children {
            if let Some(c) = self.lower_widget(child) {
                widget.children.push(c);
            }
        }

        // Apply styles in deterministic order, later wins.
        widget.applied_styles = self.resolve_styles(&widget);
        Some(widget)
    }

    fn duplicate_attr(&self, attr: &Attribute) -> Diagnostic {
        Diagnostic::error(format!("duplicate attribute `{}`", attr.name))
            .with_file(self.file.clone())
            .with_span(attr.span)
    }

    fn resolve_styles(&self, widget: &Widget) -> AppliedStyles {
        let mut applied = AppliedStyles::default();
        for rule in self.style_rules {
            let matches = match (&rule.selector, &widget.id, &widget.class) {
                (SelectorMatch::Class(c), _, Some(wc)) => c == wc,
                (SelectorMatch::Id(i), Some(wi), _) => i == wi,
                _ => false,
            };
            if !matches {
                continue;
            }
            for decl in &rule.declarations {
                apply_declaration(&mut applied, decl);
            }
            // Touch unused fields to silence dead-code lint until reporting consumers exist.
            let _ = (&rule.file, &rule.span);
        }
        applied
    }
}

fn apply_declaration(applied: &mut AppliedStyles, decl: &Declaration) {
    match (decl.name.as_str(), &decl.value) {
        ("padding", DeclarationValue::Number(n)) => applied.padding = Some(*n),
        ("width", DeclarationValue::Number(n)) => applied.width = Some(*n),
        ("height", DeclarationValue::Number(n)) => applied.height = Some(*n),
        ("background-color", DeclarationValue::HexColor(s)) => {
            applied.background_color = Some(HexColor(s.clone()))
        }
        ("text-color", DeclarationValue::HexColor(s)) => {
            applied.text_color = Some(HexColor(s.clone()))
        }
        _ => {} // unreachable for ratified pairs.
    }
}

fn attr_string(attr: &Attribute) -> String {
    let AttributeValue::String(s) = &attr.value;
    s.clone()
}

fn is_identifier(s: &str) -> bool {
    let mut chars = s.chars();
    let Some(first) = chars.next() else {
        return false;
    };
    if !(first.is_ascii_alphabetic() || first == '_') {
        return false;
    }
    chars.all(|c| c.is_ascii_alphanumeric() || c == '_')
}

#[cfg(test)]
mod tests {
    use super::*;
    use lumaui_parser::parse_document;
    use std::path::PathBuf;

    fn parse(file: &str, src: &str) -> Document {
        parse_document(&PathBuf::from(file), src).expect("parse must succeed")
    }

    fn input() -> AnalysisInput {
        AnalysisInput {
            project_name: "test".into(),
            symbol_prefix: "lumaui_".into(),
        }
    }

    #[test]
    fn accepts_minimal_screen_with_styles() {
        let m = parse(
            "home.lui",
            "<Screen id=\"home\"><Column class=\"root\"><Text id=\"title\" text=\"Hi\"/></Column></Screen>",
        );
        let s = parse(
            "theme.lus",
            ".root { padding: 16; background-color: #20242b; } #title { text-color: #f5f7fa; }",
        );
        let outcome = analyze_documents(input(), &[m, s]);
        assert!(!outcome.has_errors(), "diags: {:?}", outcome.diagnostics);
        let project = outcome.project.unwrap();
        assert_eq!(project.screens.len(), 1);
        let screen = &project.screens[0];
        assert_eq!(screen.name, "home");
        let column = &screen.root.children[0];
        assert_eq!(column.applied_styles.padding, Some(16));
        assert!(column.applied_styles.background_color.is_some());
        let text = &column.children[0];
        assert_eq!(
            text.applied_styles.text_color.as_ref().unwrap().0,
            "#f5f7fa"
        );
    }

    #[test]
    fn rejects_unsupported_widget() {
        let m = parse("x.lui", "<Screen><Container/></Screen>");
        let outcome = analyze_documents(input(), &[m]);
        assert!(outcome
            .diagnostics
            .iter()
            .any(|d| d.message.contains("unsupported widget `Container`")));
    }

    #[test]
    fn rejects_bindings() {
        let m = parse(
            "x.lui",
            "<Screen><Text bind=\"title\" text=\"x\"/></Screen>",
        );
        let outcome = analyze_documents(input(), &[m]);
        assert!(outcome
            .diagnostics
            .iter()
            .any(|d| d.message.contains("bindings are not ratified")));
    }

    #[test]
    fn rejects_duplicate_ids_across_files() {
        let a = parse("a.lui", "<Screen id=\"home\"/>");
        let b = parse("b.lui", "<Screen id=\"home\"/>");
        let outcome = analyze_documents(input(), &[a, b]);
        assert!(outcome
            .diagnostics
            .iter()
            .any(|d| d.message.contains("duplicate id `home`")));
    }

    #[test]
    fn rejects_unsupported_style_property() {
        let m = parse("home.lui", "<Screen><Column class=\"root\"/></Screen>");
        let s = parse("theme.lus", ".root { margin: 4; }");
        let outcome = analyze_documents(input(), &[m, s]);
        assert!(outcome
            .diagnostics
            .iter()
            .any(|d| d.message.contains("unsupported style property `margin`")));
    }

    #[test]
    fn rejects_unsupported_selector() {
        let m = parse("home.lui", "<Screen/>");
        let s = parse("theme.lus", "Screen { padding: 4; }");
        let outcome = analyze_documents(input(), &[m, s]);
        assert!(outcome
            .diagnostics
            .iter()
            .any(|d| d.message.contains("unsupported selector `Screen`")));
    }

    #[test]
    fn captures_event_handler_for_button() {
        let m = parse(
            "home.lui",
            "<Screen><Button id=\"go\" onPress=\"open_settings\"/></Screen>",
        );
        let outcome = analyze_documents(input(), &[m]);
        assert!(!outcome.has_errors(), "diags: {:?}", outcome.diagnostics);
        let project = outcome.project.unwrap();
        let button = &project.screens[0].root.children[0];
        assert_eq!(button.event_press.as_deref(), Some("open_settings"));
    }

    #[test]
    fn rejects_text_without_text_attribute() {
        let m = parse("home.lui", "<Screen><Text id=\"t\"/></Screen>");
        let outcome = analyze_documents(input(), &[m]);
        assert!(outcome
            .diagnostics
            .iter()
            .any(|d| d.message.contains("`Text` requires a `text` attribute")));
    }
}
