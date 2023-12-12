use std::fmt;

#[derive(Clone)]
pub struct Token {
  pub line: usize,
  pub column: usize,
  pub lexture: String,
  pub token_type: TokenType,
}

impl fmt::Debug for Token {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(
      f,
      "{:?} {} {}:{}",
      self.token_type, self.lexture, self.line, self.column
    )
  }
}

impl Token {
  pub fn new() -> Token {
    Token {
      line: 0,
      column: 0,
      lexture: String::new(),
      token_type: TokenType::None,
    }
  }
}

#[derive(Clone, Debug)]
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
  None,
}
