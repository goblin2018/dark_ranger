use crate::{
  module::Module,
  token::{Token, TokenType},
};

pub struct Analyzer {
  pub filename: String,
  pub module: Module,
  stage: Stage,
}

pub enum Stage {
  Init,
  Package,
  Struct,
  Api,
  Db,
}

impl Analyzer {
  pub fn new(filename: String) -> Analyzer {
    Analyzer {
      filename,
      module: Module::new(),
      stage: Stage::Init,
    }
  }

  pub fn analyze(&mut self, tokens: Vec<Token>) -> Result<(), String> {
    for token in tokens {
      match token.token_type {
        TokenType::Package => self.add_package(token)?,
        TokenType::Type => todo!(),
        TokenType::Api => todo!(),
        TokenType::Comment => todo!(),
        TokenType::LBrace => todo!(),
        TokenType::RBrace => todo!(),
        TokenType::LBracket => todo!(),
        TokenType::RBracket => todo!(),
        TokenType::LBrack => todo!(),
        TokenType::RBrack => todo!(),
        TokenType::Comma => todo!(),
        TokenType::Semicolon => todo!(),
        TokenType::Dot => todo!(),
        TokenType::Plus => todo!(),
        TokenType::Minus => todo!(),
        TokenType::Indentifier => todo!(),
        TokenType::None => todo!(),
      }
    }

    Ok(())
  }

  fn add_package(&mut self, token: Token) -> Result<(), String> {
    self.stage = Stage::Package;
    Ok(())
  }
}
