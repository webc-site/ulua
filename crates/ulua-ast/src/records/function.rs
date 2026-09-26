#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct Function {
  pub vararg: bool,
  pub loop_depth: u32,
}
