#[derive(Debug)]
pub struct Module {
  pub package: String,
  pub structs: Vec<String>,
}

impl Module {
  pub fn new() -> Self {
    Self {
      package: String::new(),
      structs: Vec::new(),
    }
  }
}
