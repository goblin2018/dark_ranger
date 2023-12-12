pub mod analyzer;
pub mod cmd;
pub mod field;
pub mod file;
pub mod module;
pub mod package;
pub mod scanner;
pub mod stage;
pub mod token;

fn main() {
  // fs::create_dir("./test");

  let mut scanner = scanner::Scanner::new();
  match scanner.scan_file("./test/user.api") {
    Ok(_) => {
      for token in scanner.tokens {
        println!(
          "{:?}  {}:{}:{}",
          token, scanner.file_name, token.line, token.column
        );
      }

      println!("OK");
    }
    Err(e) => {
      println!("{:?}", e.to_string());
    }
  }
}
