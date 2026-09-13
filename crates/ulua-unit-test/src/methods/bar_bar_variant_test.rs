use crate::records::bar::Bar;

impl Bar {
  pub fn new(x: i32) -> Self {
    Self::from_prop(x * 2)
  }
}
