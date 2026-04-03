use crate::ast::{Document, DocumentKind};
use crate::lexer::lex_provisional;
use lumaui_compiler::Diagnostic;
use std::path::Path;

#[derive(Debug, Clone)]
pub struct ParseOutcome {
    pub document: Document,
    pub diagnostics: Vec<Diagnostic>,
    pub token_count: usize,
}

pub fn parse_document(
    path: &Path,
    text: &str,
    kind: DocumentKind,
) -> Result<ParseOutcome, Vec<Diagnostic>> {
    let tokens = lex_provisional(path, text)?;
    let diagnostics =
        vec![
            Diagnostic::warning("full source parsing is deferred in the first repository pass")
                .with_file(path.to_path_buf())
                .with_hint(
                    "use this pass to evolve docs, fixtures, lexer coverage, and AST contracts",
                ),
        ];

    Ok(ParseOutcome {
        document: Document::new(path.display().to_string(), kind),
        diagnostics,
        token_count: tokens.len(),
    })
}
