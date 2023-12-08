use std::fs;
use std::io::{self, BufReader};

pub fn read_file(path: &str) -> io::Result<BufReader<fs::File>> {
  let file = fs::File::open(path)?;
  let metadata = file.metadata()?;
  let reader: BufReader<fs::File> = io::BufReader::new(file);
  Ok(reader)
}
