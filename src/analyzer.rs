use std::collections::HashSet;

use crate::{
  module::Module,
  token::{Token, TokenType},
};

pub struct Analyzer {
  pub filename: String,
  pub module: Module,
  stage: Stage,
  tokens: Vec<Token>,
  next: HashSet<TokenType>,
}

#[derive(Debug)]
pub enum Stage {
  Init,
  Package,
  Import,
  Struct,
  Api,
  Db,
}

impl Analyzer {
  pub fn new(filename: String, tokens: Vec<Token>) -> Analyzer {
    Analyzer {
      filename,
      module: Module::new(),
      stage: Stage::Init,
      tokens,
    }
  }

  pub fn analyze(&mut self) -> Result<(), String> {
    for token in self.tokens.clone().iter() {
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
        TokenType::Indentifier => self.add_indentifier(token)?,
        TokenType::None => todo!(),
        TokenType::Int => todo!(),
        TokenType::Float => todo!(),
        TokenType::Str => todo!(),
      }
    }

    Ok(())
  }

  fn add_package(&mut self, token: &Token) -> Result<(), String> {
    match self.stage {
      Stage::Init => {
        self.stage = Stage::Package;
        Ok(())
      }
      _ => Err(format!(
        "Unexpected token {:?} in stage {:?}",
        token, self.stage
      )),
    }
  }

  fn add_indentifier(&mut self, token: &Token) -> Result<(), String> {
    match self.stage {
      Stage::Package => {
        self.module.package = token.lexture.clone();
        self.stage = Stage::Import;
        Ok(())
      }

      Stage::Import => {
        match token.token_type {
          // 如果第一个token 是 type 则直接转入 struct
          TokenType::Type => {
            // todo 检查 import 是否结束 如果结束 则ok
            self.stage = Stage::Struct;
            self.next = Ok(())
          }

          _ => Err(format!(
            "Unexpected token {:?} in stage {:?}",
            token, self.stage
          )),
        }
      }

      Stage::Struct => {}
      _ => Err(format!(
        "Unexpected token {:?} in stage {:?}",
        token, self.stage
      )),
    }
  }
}
