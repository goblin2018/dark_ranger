use std::io::{self, BufRead, BufReader};
use std::{fs, usize};

pub struct Scanner {
  pub line: usize,
  pub column: usize,
  pub position: usize,
  pub file_name: String,
}

impl Scanner {
  pub fn new() -> Scanner {
    Scanner {
      line: 1,
      column: 1,
      file_name: String::new(),
      position: 0,
    }
  }
  pub fn scan_file(&mut self, path: &str) -> io::Result<()> {
    let file = fs::File::open(path)?;
    let reader = BufReader::new(file);

    self.file_name = path.to_string();

    for (index, line) in reader.lines().enumerate() {
      let line = line?;
      for (column, ch) in line.chars().enumerate() {
        self.process_character(ch, index, column);
      }
    }

    Ok(())
  }

  fn process_character(&mut self, ch: char, line: usize, column: usize) {
    match ch {
      _ => {}
    }

    self.column += 1;
    // 处理每个字符的逻辑
  }
}
