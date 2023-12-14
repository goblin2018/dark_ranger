use lazy_static::lazy_static;
use std::{
  collections::{HashMap, HashSet},
  fmt,
};

use crate::identifier::Identifier;

#[derive(Clone)]
pub struct Token {
  pub line: usize,
  pub column: usize,
  pub filename: String,
  pub lexture: String,
  pub token_type: TokenType,
}

impl fmt::Debug for Token {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(
      f,
      "{:?} {} {}:{}:{}",
      self.token_type, self.lexture, self.filename, self.line, self.column
    )
  }
}

impl Token {
  pub fn new() -> Token {
    Token {
      line: 0,
      column: 0,
      filename: String::new(),
      lexture: String::new(),
      token_type: TokenType::None,
    }
  }
}

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub enum TokenType {
  Package,
  Type,

  Api,
  Comment,

  LBrace,
  RBrace,
  LBracket,
  RBracket,
  LBrack,
  RBrack,
  Comma,
  Semicolon,
  Dot,
  Plus,
  Minus,
  Indentifier,
  Int,
  Float,
  Str,

  None,
}

type IdentifierType = &'static str;
static PACKAGE_NAME: IdentifierType = "PACKAGE_NAME";

// #[derive(Debug, Eq, PartialEq, Hash, Clone)]
// pub enum IdentifierType {
//   PackageName,
// }
