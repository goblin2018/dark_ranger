pub mod field;
use std::env;
pub mod cmd;
pub mod package;

fn main() {
  let args: Vec<String> = env::args().collect();
  println!("{:#?}", args);
}
