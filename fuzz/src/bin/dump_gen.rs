use std::io::{Read, stdin};
fn main() {
  let mut d = Vec::new();
  stdin().read_to_end(&mut d).unwrap();
  print!("{}", ulua_fuzz::generate(&d));
}
