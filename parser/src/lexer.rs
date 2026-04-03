use lumaui_compiler::{Diagnostic, Span};
use std::path::Path;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Token {
    pub kind: TokenKind,
    pub lexeme: String,
    pub line: usize,
    pub column: usize,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TokenKind {
    Identifier,
    StringLiteral,
    Number,
    HexColor,
    Punctuation(char),
}

pub fn lex_provisional(path: &Path, input: &str) -> Result<Vec<Token>, Vec<Diagnostic>> {
    let mut chars = input.chars().peekable();
    let mut tokens = Vec::new();
    let mut diagnostics = Vec::new();
    let mut line = 1usize;
    let mut column = 1usize;

    while let Some(ch) = chars.next() {
        let token_line = line;
        let token_column = column;

        match ch {
            '\n' => {
                line += 1;
                column = 1;
            }
            ' ' | '\t' | '\r' => {
                column += 1;
            }
            '/' if matches!(chars.peek(), Some('/')) => {
                chars.next();
                column += 2;

                while let Some(next) = chars.peek() {
                    if *next == '\n' {
                        break;
                    }

                    chars.next();
                    column += 1;
                }
            }
            '"' => {
                column += 1;
                let mut value = String::new();
                let mut terminated = false;

                for next in chars.by_ref() {
                    match next {
                        '"' => {
                            column += 1;
                            terminated = true;
                            break;
                        }
                        '\n' => {
                            diagnostics.push(
                                Diagnostic::error("unterminated string literal")
                                    .with_file(path.to_path_buf())
                                    .with_span(Span {
                                        line: token_line,
                                        column: token_column,
                                        length: value.len().max(1),
                                    }),
                            );
                            line += 1;
                            column = 1;
                            break;
                        }
                        other => {
                            value.push(other);
                            column += 1;
                        }
                    }
                }

                if terminated {
                    tokens.push(Token {
                        kind: TokenKind::StringLiteral,
                        lexeme: value,
                        line: token_line,
                        column: token_column,
                    });
                }
            }
            '#' => {
                column += 1;
                let mut value = String::from("#");

                while let Some(next) = chars.peek() {
                    if next.is_ascii_hexdigit() {
                        value.push(*next);
                        chars.next();
                        column += 1;
                    } else {
                        break;
                    }
                }

                if value.len() > 1 {
                    tokens.push(Token {
                        kind: TokenKind::HexColor,
                        lexeme: value,
                        line: token_line,
                        column: token_column,
                    });
                } else {
                    tokens.push(Token {
                        kind: TokenKind::Punctuation('#'),
                        lexeme: "#".to_string(),
                        line: token_line,
                        column: token_column,
                    });
                }
            }
            '0'..='9' => {
                column += 1;
                let mut value = String::from(ch);

                while let Some(next) = chars.peek() {
                    if next.is_ascii_digit() {
                        value.push(*next);
                        chars.next();
                        column += 1;
                    } else {
                        break;
                    }
                }

                tokens.push(Token {
                    kind: TokenKind::Number,
                    lexeme: value,
                    line: token_line,
                    column: token_column,
                });
            }
            '<' | '>' | '/' | '=' | '{' | '}' | ':' | ';' | '.' | ',' | '(' | ')' | '[' | ']' => {
                column += 1;
                tokens.push(Token {
                    kind: TokenKind::Punctuation(ch),
                    lexeme: ch.to_string(),
                    line: token_line,
                    column: token_column,
                });
            }
            _ if is_identifier_start(ch) => {
                column += 1;
                let mut value = String::from(ch);

                while let Some(next) = chars.peek() {
                    if is_identifier_continue(*next) {
                        value.push(*next);
                        chars.next();
                        column += 1;
                    } else {
                        break;
                    }
                }

                tokens.push(Token {
                    kind: TokenKind::Identifier,
                    lexeme: value,
                    line: token_line,
                    column: token_column,
                });
            }
            _ => {
                diagnostics.push(
                    Diagnostic::error(format!("unexpected character `{ch}`"))
                        .with_file(path.to_path_buf())
                        .with_span(Span {
                            line: token_line,
                            column: token_column,
                            length: 1,
                        }),
                );
                column += 1;
            }
        }
    }

    if diagnostics.is_empty() {
        Ok(tokens)
    } else {
        Err(diagnostics)
    }
}

fn is_identifier_start(ch: char) -> bool {
    ch.is_ascii_alphabetic() || ch == '_'
}

fn is_identifier_continue(ch: char) -> bool {
    ch.is_ascii_alphanumeric() || matches!(ch, '_' | '-')
}

#[cfg(test)]
mod tests {
    use super::{lex_provisional, TokenKind};
    use std::path::Path;

    #[test]
    fn lexes_the_minimal_example_fixture() {
        let fixture = include_str!("../../examples/minimal/ui/screens/home.lui");
        let tokens = lex_provisional(Path::new("home.lui"), fixture).expect("fixture should lex");

        assert!(tokens.len() > 10);
        assert!(tokens.iter().any(|token| token.lexeme == "Screen"));
        assert!(tokens
            .iter()
            .any(|token| token.kind == TokenKind::StringLiteral));
    }

    #[test]
    fn lexes_style_hex_colors() {
        let fixture = include_str!("../../examples/minimal/ui/styles/theme.lus");
        let tokens = lex_provisional(Path::new("theme.lus"), fixture).expect("fixture should lex");

        assert!(tokens.iter().any(|token| token.kind == TokenKind::HexColor));
    }
}
