use crate::records::position::Position;

impl Position {
  pub fn new(line: u32, column: u32) -> Self {
    Self { line, column }
  }
}
