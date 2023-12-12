pub struct Module {
  pub package: String,
}

impl Module {
  pub fn new() -> Self {
    Self {
      package: String::new(),
    }
  }
}
