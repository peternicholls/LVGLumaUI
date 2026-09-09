use crate::ast::*;
use lumaui_compiler::{Diagnostic, Span};
use std::{collections::BTreeSet, path::Path};

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
    let mut parser = Parser {
        path,
        text,
        offset: 0,
        tokens: 0,
    };
    let result = (|| {
        let mut document = Document::new(path.display().to_string(), kind);
        parser.space()?;
        match kind {
            DocumentKind::Markup => {
                document.items.push(TopLevel::Widget(parser.widget(0)?));
                parser.space()?;
                if !parser.rest().is_empty() {
                    return Err(parser.error("expected exactly one top-level widget"));
                }
            }
            DocumentKind::Style => {
                while !parser.rest().is_empty() {
                    document.items.push(TopLevel::StyleRule(parser.rule()?));
                    parser.space()?;
                }
            }
        }
        Ok(ParseOutcome {
            document,
            diagnostics: Vec::new(),
            token_count: parser.tokens,
        })
    })();
    result.map_err(|error| vec![error])
}

struct Parser<'a> {
    path: &'a Path,
    text: &'a str,
    offset: usize,
    tokens: usize,
}

impl Parser<'_> {
    fn rest(&self) -> &str {
        &self.text[self.offset..]
    }
    fn error(&self, message: &str) -> Diagnostic {
        let before = &self.text[..self.offset];
        Diagnostic::error(message)
            .with_file(self.path)
            .with_span(Span {
                line: before.bytes().filter(|b| *b == b'\n').count() + 1,
                column: before.rsplit('\n').next().unwrap_or("").chars().count() + 1,
                length: 1,
            })
    }
    fn space(&mut self) -> Result<(), Diagnostic> {
        loop {
            let trimmed = self.rest().trim_start();
            self.offset = self.text.len() - trimmed.len();
            if self.rest().starts_with("//") {
                self.offset += self.rest().find('\n').unwrap_or(self.rest().len());
            } else if self.rest().starts_with("/*") {
                let end = self
                    .rest()
                    .find("*/")
                    .ok_or_else(|| self.error("unterminated comment"))?;
                self.offset += end + 2;
            } else {
                return Ok(());
            }
        }
    }
    fn take(&mut self, value: &str) -> bool {
        if self.rest().starts_with(value) {
            self.offset += value.len();
            self.tokens += 1;
            true
        } else {
            false
        }
    }
    fn expect(&mut self, value: &str) -> Result<(), Diagnostic> {
        if self.take(value) {
            Ok(())
        } else {
            Err(self.error(&format!("expected `{value}`")))
        }
    }
    fn identifier(&mut self) -> Result<String, Diagnostic> {
        let mut chars = self.rest().char_indices();
        if !chars
            .next()
            .is_some_and(|(_, c)| c.is_ascii_alphabetic() || c == '_')
        {
            return Err(self.error("expected an identifier"));
        }
        let length = chars
            .find(|(_, c)| !(c.is_ascii_alphanumeric() || matches!(c, '_' | '-')))
            .map_or(self.rest().len(), |(i, _)| i);
        let value = self.rest()[..length].to_owned();
        self.offset += length;
        self.tokens += 1;
        Ok(value)
    }
    fn string(&mut self) -> Result<String, Diagnostic> {
        self.expect("\"")?;
        let end = self
            .rest()
            .find('"')
            .ok_or_else(|| self.error("unterminated string literal"))?;
        let value = &self.rest()[..end];
        if value.chars().any(|c| c.is_control()) {
            return Err(self.error("control characters are not allowed in string literals"));
        }
        let value = value.to_owned();
        self.offset += end + 1;
        self.tokens += 1;
        Ok(value)
    }
    fn widget(&mut self, depth: usize) -> Result<WidgetNode, Diagnostic> {
        if depth >= 128 {
            return Err(self.error("widget nesting exceeds the 128-level compiler limit"));
        }
        self.expect("<")?;
        let widget_type = self.identifier()?;
        let mut node = WidgetNode {
            widget_type,
            id: None,
            classes: vec![],
            attributes: vec![],
            children: vec![],
        };
        let mut names = BTreeSet::new();
        loop {
            let before = self.offset;
            self.space()?;
            if self.take("/>") {
                return Ok(node);
            }
            if self.take(">") {
                break;
            }
            if self.offset == before {
                return Err(self.error("expected whitespace before attribute"));
            }
            let name = self.identifier()?;
            if !names.insert(name.clone()) {
                return Err(self.error(&format!("duplicate attribute `{name}`")));
            }
            self.space()?;
            self.expect("=")?;
            self.space()?;
            let value = self.string()?;
            match name.as_str() {
                "id" => node.id = Some(value),
                "class" => node.classes = value.split_whitespace().map(str::to_owned).collect(),
                _ => node.attributes.push(Attribute {
                    name,
                    value: AttributeValue::String(value),
                }),
            }
        }
        loop {
            self.space()?;
            if self.take("</") {
                let closing = self.identifier()?;
                if closing != node.widget_type {
                    return Err(
                        self.error(&format!("expected closing tag `</{}>`", node.widget_type))
                    );
                }
                self.space()?;
                self.expect(">")?;
                return Ok(node);
            }
            if !self.rest().starts_with('<') {
                return Err(self.error("expected a child widget or closing tag; use the text attribute for literal text"));
            }
            node.children.push(self.widget(depth + 1)?);
        }
    }
    fn rule(&mut self) -> Result<StyleRule, Diagnostic> {
        let selector = self.until('{')?;
        self.expect("{")?;
        let mut declarations = Vec::new();
        loop {
            self.space()?;
            if self.take("}") {
                return Ok(StyleRule {
                    selector,
                    declarations,
                });
            }
            let name = self.identifier()?;
            self.space()?;
            self.expect(":")?;
            let value = self.until(';')?;
            self.expect(";")?;
            declarations.push(Declaration { name, value });
        }
    }
    fn until(&mut self, delimiter: char) -> Result<String, Diagnostic> {
        let start = self.offset;
        while let Some(ch) = self.rest().chars().next() {
            if ch == delimiter {
                let value = self.text[start..self.offset].trim().to_owned();
                if value.is_empty() {
                    return Err(self.error("expected a non-empty selector or declaration value"));
                }
                self.tokens += 1;
                return Ok(value);
            }
            if matches!(ch, '{' | '}' | ';') {
                return Err(self.error(&format!("expected `{delimiter}`")));
            }
            self.offset += ch.len_utf8();
        }
        Err(self.error(&format!("expected `{delimiter}` before end of file")))
    }
}
