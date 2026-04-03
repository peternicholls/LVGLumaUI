use std::fmt;
use std::path::PathBuf;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Severity {
    Error,
    Warning,
    Note,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Span {
    pub line: usize,
    pub column: usize,
    pub length: usize,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Diagnostic {
    pub severity: Severity,
    pub message: String,
    pub file: Option<PathBuf>,
    pub span: Option<Span>,
    pub hint: Option<String>,
}

impl Diagnostic {
    pub fn error(message: impl Into<String>) -> Self {
        Self {
            severity: Severity::Error,
            message: message.into(),
            file: None,
            span: None,
            hint: None,
        }
    }

    pub fn warning(message: impl Into<String>) -> Self {
        Self {
            severity: Severity::Warning,
            message: message.into(),
            file: None,
            span: None,
            hint: None,
        }
    }

    pub fn note(message: impl Into<String>) -> Self {
        Self {
            severity: Severity::Note,
            message: message.into(),
            file: None,
            span: None,
            hint: None,
        }
    }

    pub fn with_file(mut self, file: impl Into<PathBuf>) -> Self {
        self.file = Some(file.into());
        self
    }

    pub fn with_span(mut self, span: Span) -> Self {
        self.span = Some(span);
        self
    }

    pub fn with_hint(mut self, hint: impl Into<String>) -> Self {
        self.hint = Some(hint.into());
        self
    }
}

impl fmt::Display for Diagnostic {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let severity = match self.severity {
            Severity::Error => "error",
            Severity::Warning => "warning",
            Severity::Note => "note",
        };

        write!(f, "{severity}: {}", self.message)?;

        if let Some(file) = &self.file {
            write!(f, " [{}]", file.display())?;
        }

        if let Some(span) = self.span {
            write!(f, " at {}:{}", span.line, span.column)?;
        }

        if let Some(hint) = &self.hint {
            write!(f, " | hint: {hint}")?;
        }

        Ok(())
    }
}
