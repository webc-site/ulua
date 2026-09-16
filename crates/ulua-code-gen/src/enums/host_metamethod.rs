#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum HostMetamethod {
  Add,
  Sub,
  Mul,
  Div,
  Idiv,
  Mod,
  Pow,
  Minus,
  Equal,
  LessThan,
  LessEqual,
  Length,
  Concat,
}
