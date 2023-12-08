pub enum Keyword {
  Package(&'static str),
  Import(&'static str),
  Api(&'static str),
  
}

pub static PACKAGE: Keyword = Keyword::Package("package");
pub static IMPORT: Keyword = Keyword::Import("import");
pub static API: Keyword = Keyword::Api("api");

