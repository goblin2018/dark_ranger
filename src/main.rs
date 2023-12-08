pub mod field;
use std::io::BufRead;
pub mod cmd;
pub mod file;
pub mod package;
pub mod scanner;
pub mod stage;
pub mod token;

fn main() {
  // fs::create_dir("./test");

  match file::read_file("./test/user.api") {
    Ok(reader) => {
      for (index, line) in reader.lines().enumerate() {
        for (char_index, character) in line.unwrap().chars().enumerate() {
          println!("Char {}: {}", char_index + 1, character);
          match character {
            ' ' => {
              println!("space");
            }
            _ => {
              println!("not space");
            }
          }

          match current_state {
            State::Initial => {
              if c == 'p' {
                current_state = State::Package;
              } else {
                break; // 不匹配，提前结束
              }
            }
            State::Package => {
              if c == 'a' {
                current_state = State::Space;
              } else {
                break;
              }
            }
            State::Space => {
              if c == ' ' {
                current_state = State::User;
              } else {
                break;
              }
            }
            State::User => {
              if c == 'u' {
                current_state = State::Matched;
              } else {
                break;
              }
            }
            State::Matched => {
              // 已经匹配完成，不需要继续处理字符
              break;
            }
          }
        }
      }
    }
    Err(e) => {
      eprintln!("{:?}", e);
    }
  }
}
