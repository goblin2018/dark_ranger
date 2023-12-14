pub mod analyzer;
pub mod cmd;
pub mod entity;
pub mod field;
pub mod file;
pub mod module;
pub mod package;
pub mod scanner;
pub mod stage;
pub mod token;
pub mod identifier;

fn main() {
  // fs::create_dir("./test");

  let mut scanner = scanner::Scanner::new();
  match scanner.scan_file("./test/user.api") {
    Ok(_) => {
      for token in &scanner.tokens {
        println!("{:?}", token);
      }

      let mut analyzer = analyzer::Analyzer::new(scanner.file_name, scanner.tokens.clone());
      match analyzer.analyze() {
        Ok(_) => {
          println!("{:?}", analyzer.module);
        }
        Err(e) => {
          println!("{:?}", e.to_string());
        }
      }
    }
    Err(e) => {
      println!("{:?}", e.to_string());
    }
  }
}
