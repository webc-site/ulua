#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(C)]
pub struct Label {
  pub id: u32,
  pub location: u32,
}

impl Default for Label {
  fn default() -> Self {
    Self {
      id: 0,
      location: !0u32,
    }
  }
}
