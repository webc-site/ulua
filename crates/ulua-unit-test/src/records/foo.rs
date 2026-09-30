#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Foo {
  pub x: i32,
}

impl Default for Foo {
  fn default() -> Self {
    Self { x: 42 }
  }
}
