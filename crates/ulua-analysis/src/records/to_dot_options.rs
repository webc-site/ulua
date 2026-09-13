#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ToDotOptions {
  pub show_pointers: bool,
  pub duplicate_primitives: bool,
}

impl Default for ToDotOptions {
  fn default() -> Self {
    Self {
      show_pointers: true,
      duplicate_primitives: true,
    }
  }
}
