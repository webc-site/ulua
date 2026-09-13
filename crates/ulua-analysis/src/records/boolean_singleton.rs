#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
pub struct BooleanSingleton {
  pub value: bool,
}
impl BooleanSingleton {
  pub const fn new(value: bool) -> Self {
    Self { value }
  }
}
unsafe impl Send for BooleanSingleton {}
unsafe impl Sync for BooleanSingleton {}
