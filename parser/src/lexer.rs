//! Lexer for LumaUI markup (`.lui`) and style (`.lus`) sources.
//!
//! The lexer is intentionally shared between markup and style documents because
//! their token sets overlap heavily. Document-kind disambiguation happens in the
//! parser, not here.

use lumaui_compiler::{Diagnostic, Span};
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Token {
    pub kind: TokenKind,
    pub lexeme: String,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TokenKind {
    /// `[A-Za-z_][A-Za-z0-9_]*`
    Identifier,
    /// Double-quoted string with no embedded newline and no escape processing.
    StringLiteral,
    /// `[0-9]+`
    Number,
    /// `#` followed by exactly six hex digits.
    HexColor,
    OpenAngle,
    CloseAngle,
    Slash,
    Equals,
    OpenBrace,
    CloseBrace,
    Colon,
    Semicolon,
    Dot,
    /// `#` not followed by exactly six hex digits.
    Hash,
    Hyphen,
}

pub fn lex(path: &Path, input: &str) -> Result<Vec<Token>, Vec<Diagnostic>> {
    let mut lexer = Lexer::new(path.to_path_buf(), input);
    lexer.run();
    if lexer.diagnostics.is_empty() {
        Ok(lexer.tokens)
    } else {
        Err(lexer.diagnostics)
    }
}

struct Lexer {
    file: PathBuf,
    chars: Vec<char>,
    cursor: usize,
    line: usize,
    column: usize,
    tokens: Vec<Token>,
    diagnostics: Vec<Diagnostic>,
}

impl Lexer {
    fn new(file: PathBuf, input: &str) -> Self {
        Self {
            file,
            chars: input.chars().collect(),
            cursor: 0,
            line: 1,
            column: 1,
            tokens: Vec::new(),
            diagnostics: Vec::new(),
        }
    }

    fn peek(&self, offset: usize) -> Option<char> {
        self.chars.get(self.cursor + offset).copied()
    }

    fn advance(&mut self) -> Option<char> {
        let ch = self.chars.get(self.cursor).copied()?;
        self.cursor += 1;
        if ch == '\n' {
            self.line += 1;
            self.column = 1;
        } else {
            self.column += 1;
        }
        Some(ch)
    }

    fn span_at(&self, line: usize, column: usize, length: usize) -> Span {
        Span {
            line,
            column,
            length,
        }
    }

    fn push(&mut self, kind: TokenKind, lexeme: String, span: Span) {
        self.tokens.push(Token { kind, lexeme, span });
    }

    fn diag(&mut self, message: impl Into<String>, span: Span, hint: Option<&str>) {
        let mut d = Diagnostic::error(message)
            .with_file(self.file.clone())
            .with_span(span);
        if let Some(h) = hint {
            d = d.with_hint(h);
        }
        self.diagnostics.push(d);
    }

    fn run(&mut self) {
        while let Some(ch) = self.peek(0) {
            let line = self.line;
            let column = self.column;

            match ch {
                ' ' | '\t' | '\r' | '\n' => {
                    self.advance();
                }
                '/' if self.peek(1) == Some('/') => {
                    // line comment
                    while let Some(c) = self.peek(0) {
                        if c == '\n' {
                            break;
                        }
                        self.advance();
                    }
                }
                '/' => {
                    self.advance();
                    self.push(TokenKind::Slash, "/".into(), self.span_at(line, column, 1));
                }
                '<' => {
                    self.advance();
                    self.push(
                        TokenKind::OpenAngle,
                        "<".into(),
                        self.span_at(line, column, 1),
                    );
                }
                '>' => {
                    self.advance();
                    self.push(
                        TokenKind::CloseAngle,
                        ">".into(),
                        self.span_at(line, column, 1),
                    );
                }
                '=' => {
                    self.advance();
                    self.push(TokenKind::Equals, "=".into(), self.span_at(line, column, 1));
                }
                '{' => {
                    self.advance();
                    self.push(
                        TokenKind::OpenBrace,
                        "{".into(),
                        self.span_at(line, column, 1),
                    );
                }
                '}' => {
                    self.advance();
                    self.push(
                        TokenKind::CloseBrace,
                        "}".into(),
                        self.span_at(line, column, 1),
                    );
                }
                ':' => {
                    self.advance();
                    self.push(TokenKind::Colon, ":".into(), self.span_at(line, column, 1));
                }
                ';' => {
                    self.advance();
                    self.push(
                        TokenKind::Semicolon,
                        ";".into(),
                        self.span_at(line, column, 1),
                    );
                }
                '.' => {
                    self.advance();
                    self.push(TokenKind::Dot, ".".into(), self.span_at(line, column, 1));
                }
                '-' => {
                    self.advance();
                    self.push(TokenKind::Hyphen, "-".into(), self.span_at(line, column, 1));
                }
                '#' => self.lex_hash(line, column),
                '"' => self.lex_string(line, column),
                c if c.is_ascii_digit() => self.lex_number(line, column),
                c if is_ident_start(c) => self.lex_identifier(line, column),
                other => {
                    self.advance();
                    self.diag(
                        format!("unexpected character `{other}`"),
                        self.span_at(line, column, 1),
                        Some("only ratified ASCII syntax is accepted in the first slice"),
                    );
                }
            }
        }
    }

    fn lex_string(&mut self, line: usize, column: usize) {
        self.advance(); // consume opening quote
        let mut value = String::new();
        let mut closed = false;
        while let Some(c) = self.peek(0) {
            match c {
                '"' => {
                    self.advance();
                    closed = true;
                    break;
                }
                '\n' => {
                    self.diag(
                        "unterminated string literal",
                        self.span_at(line, column, value.len() + 1),
                        Some("string literals must close on the same line"),
                    );
                    return;
                }
                _ => {
                    value.push(c);
                    self.advance();
                }
            }
        }
        if !closed {
            self.diag(
                "unterminated string literal at end of file",
                self.span_at(line, column, value.len() + 1),
                None,
            );
            return;
        }
        let length = value.len() + 2;
        self.push(
            TokenKind::StringLiteral,
            value,
            self.span_at(line, column, length),
        );
    }

    fn lex_number(&mut self, line: usize, column: usize) {
        let mut value = String::new();
        while let Some(c) = self.peek(0) {
            if c.is_ascii_digit() {
                value.push(c);
                self.advance();
            } else {
                break;
            }
        }
        let length = value.len();
        self.push(TokenKind::Number, value, self.span_at(line, column, length));
    }

    fn lex_identifier(&mut self, line: usize, column: usize) {
        let mut value = String::new();
        while let Some(c) = self.peek(0) {
            if is_ident_continue(c) {
                value.push(c);
                self.advance();
            } else {
                break;
            }
        }
        let length = value.len();
        self.push(
            TokenKind::Identifier,
            value,
            self.span_at(line, column, length),
        );
    }

    fn lex_hash(&mut self, line: usize, column: usize) {
        // Lookahead: exactly six hex digits followed by non-ident-continue → HexColor.
        let mut hex = String::new();
        let mut idx = 1;
        while let Some(c) = self.peek(idx) {
            if c.is_ascii_hexdigit() {
                hex.push(c);
                idx += 1;
                if hex.len() > 6 {
                    break;
                }
            } else {
                break;
            }
        }
        let next_after = self.peek(1 + hex.len());
        let next_is_ident = next_after.map(is_ident_continue).unwrap_or(false);

        if hex.len() == 6 && !next_is_ident {
            // consume `#` and six hex chars
            for _ in 0..7 {
                self.advance();
            }
            let lexeme = format!("#{hex}");
            self.push(TokenKind::HexColor, lexeme, self.span_at(line, column, 7));
        } else {
            self.advance();
            self.push(TokenKind::Hash, "#".into(), self.span_at(line, column, 1));
        }
    }
}

fn is_ident_start(c: char) -> bool {
    c.is_ascii_alphabetic() || c == '_'
}

fn is_ident_continue(c: char) -> bool {
    c.is_ascii_alphanumeric() || c == '_'
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    fn tokens(input: &str) -> Vec<Token> {
        lex(&PathBuf::from("test.lui"), input).expect("lex should succeed")
    }

    #[test]
    fn lexes_markup_punctuation_and_identifier() {
        let toks = tokens("<Screen id=\"home\"/>");
        let kinds: Vec<_> = toks.iter().map(|t| t.kind.clone()).collect();
        assert_eq!(
            kinds,
            vec![
                TokenKind::OpenAngle,
                TokenKind::Identifier,
                TokenKind::Identifier,
                TokenKind::Equals,
                TokenKind::StringLiteral,
                TokenKind::Slash,
                TokenKind::CloseAngle,
            ]
        );
        assert_eq!(toks[1].lexeme, "Screen");
        assert_eq!(toks[4].lexeme, "home");
    }

    #[test]
    fn lexes_hex_color_when_six_hex_digits() {
        let toks = tokens(".root { background-color: #20242b; }");
        let hex = toks
            .iter()
            .find(|t| t.kind == TokenKind::HexColor)
            .expect("expected HexColor");
        assert_eq!(hex.lexeme, "#20242b");
    }

    #[test]
    fn lexes_hash_when_followed_by_id_selector() {
        let toks = tokens("#title { padding: 4; }");
        assert_eq!(toks[0].kind, TokenKind::Hash);
        assert_eq!(toks[1].kind, TokenKind::Identifier);
        assert_eq!(toks[1].lexeme, "title");
    }

    #[test]
    fn line_and_column_track_through_newlines() {
        let toks = tokens("\n  <Screen/>");
        let open = &toks[0];
        assert_eq!(open.kind, TokenKind::OpenAngle);
        assert_eq!(open.span.line, 2);
        assert_eq!(open.span.column, 3);
    }

    #[test]
    fn rejects_unterminated_string() {
        let err = lex(&PathBuf::from("t.lui"), "<Text text=\"oops\n").unwrap_err();
        assert!(err
            .iter()
            .any(|d| d.message.contains("unterminated string")));
    }

    #[test]
    fn rejects_unknown_character() {
        let err = lex(&PathBuf::from("t.lui"), "<Screen ?/>").unwrap_err();
        assert!(err
            .iter()
            .any(|d| d.message.contains("unexpected character")));
    }

    #[test]
    fn line_comments_are_skipped() {
        let toks = tokens("// hello\n<Screen/>");
        assert_eq!(toks[0].kind, TokenKind::OpenAngle);
        assert_eq!(toks[0].span.line, 2);
    }

    #[test]
    fn hyphen_is_its_own_token_for_property_names() {
        let toks = tokens("background-color");
        assert_eq!(toks[0].kind, TokenKind::Identifier);
        assert_eq!(toks[1].kind, TokenKind::Hyphen);
        assert_eq!(toks[2].kind, TokenKind::Identifier);
    }
}
