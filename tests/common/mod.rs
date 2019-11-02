use std::fs::File;
use std::io::prelude::*;
use std::io::Error;

pub fn create_and_write_file(name: &str, content: &str) -> Result<(), Error> {
  let mut file = File::create(name)?;
  file.write_all(content.as_bytes())?;
  Ok(())
} 