use lumaui_compiler::Diagnostic;
use lumaui_ir::Project;
use lumaui_parser::{Document, TopLevel, WidgetNode};
use std::collections::BTreeMap;

#[derive(Debug, Clone)]
pub struct AnalysisOutcome {
    pub project: Project,
    pub diagnostics: Vec<Diagnostic>,
}

pub fn analyze_documents(project_name: &str, documents: &[Document]) -> AnalysisOutcome {
    let mut diagnostics = Vec::new();

    for document in documents {
        diagnostics.extend(validate_duplicate_ids(document));
    }

    if documents.iter().all(|document| document.items.is_empty()) {
        diagnostics.push(
            Diagnostic::warning("semantic lowering is deferred until the authored grammar is ratified")
                .with_hint("the current pass validates repo shape, config, fixtures, and compiler boundaries"),
        );
    }

    AnalysisOutcome {
        project: Project::new(project_name),
        diagnostics,
    }
}

pub fn validate_duplicate_ids(document: &Document) -> Vec<Diagnostic> {
    let mut seen = BTreeMap::<String, usize>::new();
    let mut diagnostics = Vec::new();

    for item in &document.items {
        if let TopLevel::Widget(widget) = item {
            visit_widget(widget, &mut seen, &mut diagnostics, &document.source_name);
        }
    }

    diagnostics
}

fn visit_widget(
    widget: &WidgetNode,
    seen: &mut BTreeMap<String, usize>,
    diagnostics: &mut Vec<Diagnostic>,
    source_name: &str,
) {
    if let Some(id) = &widget.id {
        let count = seen.entry(id.clone()).or_insert(0);
        *count += 1;

        if *count > 1 {
            diagnostics.push(
                Diagnostic::error(format!("duplicate id `{id}`"))
                    .with_file(source_name.to_string())
                    .with_hint("ids must be unique within a compiled project"),
            );
        }
    }

    for child in &widget.children {
        visit_widget(child, seen, diagnostics, source_name);
    }
}

#[cfg(test)]
mod tests {
    use super::validate_duplicate_ids;
    use lumaui_parser::{Document, DocumentKind, TopLevel, WidgetNode};

    #[test]
    fn reports_duplicate_widget_ids() {
        let mut document = Document::new("duplicate_ids.lui", DocumentKind::Markup);
        document.items.push(TopLevel::Widget(WidgetNode {
            widget_type: "Column".to_string(),
            id: Some("root".to_string()),
            classes: Vec::new(),
            attributes: Vec::new(),
            children: vec![
                WidgetNode {
                    widget_type: "Text".to_string(),
                    id: Some("title".to_string()),
                    classes: Vec::new(),
                    attributes: Vec::new(),
                    children: Vec::new(),
                },
                WidgetNode {
                    widget_type: "Button".to_string(),
                    id: Some("title".to_string()),
                    classes: Vec::new(),
                    attributes: Vec::new(),
                    children: Vec::new(),
                },
            ],
        }));

        let diagnostics = validate_duplicate_ids(&document);

        assert_eq!(diagnostics.len(), 1);
        assert!(diagnostics[0].message.contains("duplicate id"));
    }
}
