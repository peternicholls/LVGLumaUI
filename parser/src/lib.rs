//! Parser stage for LumaUI: lexer, AST, and recursive-descent parser.
//!
//! The parser owns syntactic validity. Anything past syntax — supported widget
//! kinds, supported style properties, binding policy — belongs in
//! `lumaui-semantic`.

pub mod ast;
pub mod lexer;
pub mod parse;

pub use ast::{
    Attribute, AttributeValue, Declaration, DeclarationValue, Document, DocumentKind, Selector,
    StyleRule, TopLevel, WidgetNode,
};
pub use lexer::{lex, Token, TokenKind};
pub use parse::parse_document;
