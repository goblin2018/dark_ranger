use std::fmt;
use std::io::{self, BufRead, BufReader};
use std::{fs, usize};

use crate::token::{Token, TokenType};

pub struct Scanner {
  pub line: usize,
  pub column: usize,
  pub position: usize,
  pub file_name: String,
  state: State,
  pub tokens: Vec<Token>,

  pub curr_str: String,
  pub curr_token: Token,
}

enum State {
  Init,
  Str,
  Float,
  Indentifier,
  Int,
}

impl Scanner {
  pub fn new() -> Scanner {
    Scanner {
      line: 1,
      column: 1,
      file_name: String::new(),
      position: 0,
      state: State::Init,
      tokens: vec![],
      curr_str: String::new(),
      curr_token: Token::new(),
    }
  }
  pub fn scan_file(&mut self, path: &str) -> io::Result<()> {
    let file = fs::File::open(path)?;
    let reader = BufReader::new(file);

    self.file_name = path.to_string();

    for (index, line) in reader.lines().enumerate() {
      self.line = index + 1;
      let line = line?;
      for (column, ch) in line.chars().enumerate() {
        self.column = column + 1;
        match self.process_character(ch) {
          Ok(()) => {}
          Err(err) => return Err(io::Error::new(io::ErrorKind::Other, err)),
        }
      }

      // 处理换行
      match self.handle_space() {
        Ok(()) => {}
        Err(err) => return Err(io::Error::new(io::ErrorKind::Other, err)),
      }
    }

    Ok(())
  }

  fn process_character(&mut self, ch: char) -> Result<(), String> {
    match ch {
      ' ' => self.handle_space(),
      // 处理字母
      'a'..='z' | 'A'..='Z' => self.handle_aplha(ch),
      '_' => self.handle_(ch),
      // 处理数字
      '0'..='9' => self.handle_number(),
      '/' => self.handle_slash(),
      '.' => self.handle_dot(),
      '"' => self.handle_quote(),
      ':' => self.handle_colon(),
      '@' => self.handle_at(),
      '(' => self.handle_lparen(),
      ')' => self.handle_rparen(),
      '{' => self.handle_lbrace(),
      '}' => self.handle_rbrace(),
      '[' => self.handle_lbracket(),
      ']' => self.handle_rbracket(),
      _ => self.err(format_args!("invalid character: {}", ch)),
    }

    // 处理每个字符的逻辑
  }

  fn err(&mut self, args: fmt::Arguments) -> Result<(), String> {
    Err(format!(
      "{} {}:{}:{}",
      fmt::format(args),
      self.file_name,
      self.line,
      self.column,
    ))
  }

  // 处理字母
  fn handle_aplha(&mut self, ch: char) -> Result<(), String> {
    match self.state {
      State::Init => {
        self.state = State::Indentifier;
        self.curr_token = Token {
          line: self.line,
          column: self.column,
          lexture: String::new(),
          token_type: TokenType::None,
        };
        self.curr_str.push(ch);
        Ok(())
      }
      State::Str | State::Indentifier => {
        self.curr_str.push(ch);
        Ok(())
      }
      State::Float | State::Int => self.err(format_args!("char after number:{}", ch)),
    }
  }

  fn handle_(&mut self, ch: char) -> Result<(), String> {
    // 处理下划线
    match self.state {
      State::Init => self.err(format_args!("invalid first _")),
      State::Str | State::Indentifier => {
        self.curr_str.push(ch);
        Ok(())
      }
      State::Float | State::Int => Err("char _ after number".to_string()),
    }
  }

  fn handle_space(&mut self) -> Result<(), String> {
    // 处理空格
    match self.state {
      State::Init => Ok(()),
      State::Str | State::Indentifier | State::Float | State::Int => {
        self.state = State::Init;
        // 处理一下 token 已经完成了
        match self.curr_str.as_str() {
          "package" => self.curr_token.token_type = TokenType::Package,
          "type" => self.curr_token.token_type = TokenType::Type,
          "api" => self.curr_token.token_type = TokenType::Api,
          "//" => self.curr_token.token_type = TokenType::Comment,

          // 什么都没匹配到 认为是 用户定义的变量
          _ => {
            self.curr_token.token_type = TokenType::Indentifier;
            self.curr_token.lexture = self.curr_str.clone();
          }
        }

        self.tokens.push(self.curr_token.clone());
        self.curr_str = String::new();
        self.curr_token = Token::new();

        Ok(())
      }
    }
  }

  fn handle_number(&mut self) -> Result<(), String> {
    // 处理数字
    Ok(())
  }

  fn handle_slash(&mut self) -> Result<(), String> {
    // 处理斜杠
    Ok(())
  }

  fn handle_dot(&mut self) -> Result<(), String> {
    // 处理点
    Ok(())
  }

  fn handle_quote(&mut self) -> Result<(), String> {
    // 处理字符串
    Ok(())
  }

  fn handle_colon(&mut self) -> Result<(), String> {
    // 处理冒号
    Ok(())
  }

  fn handle_at(&mut self) -> Result<(), String> {
    // 处理@
    Ok(())
  }

  fn handle_lparen(&mut self) -> Result<(), String> {
    // 处理(
    Ok(())
  }

  fn handle_rparen(&mut self) -> Result<(), String> {
    // 处理)
    Ok(())
  }

  fn handle_lbrace(&mut self) -> Result<(), String> {
    // 处理{
    Ok(())
  }

  fn handle_rbrace(&mut self) -> Result<(), String> {
    // 处理}
    Ok(())
  }

  fn handle_lbracket(&mut self) -> Result<(), String> {
    // 处理[
    Ok(())
  }

  fn handle_rbracket(&mut self) -> Result<(), String> {
    // 处理]
    Ok(())
  }
}
