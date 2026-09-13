use alloc::string::String;
use std::io::{Read, stdin};

pub fn read_stdin() -> Option<String> {
  let mut buffer = String::new();
  let mut input = stdin();

  match input.read_to_string(&mut buffer) {
    Ok(_) => Some(buffer),
    Err(_) => None,
  }
}
