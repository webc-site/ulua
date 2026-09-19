#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Kind {
  Local,
  Upvalue,
  Global,
  IndexName,
  IndexNumber,
  IndexExpr,
}
