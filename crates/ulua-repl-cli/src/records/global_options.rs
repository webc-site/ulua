#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct GlobalOptions {
  pub optimization_level: i32,
  pub debug_level: i32,
}

impl Default for GlobalOptions {
  fn default() -> Self {
    Self {
      optimization_level: 1,
      debug_level: 1,
    }
  }
}
