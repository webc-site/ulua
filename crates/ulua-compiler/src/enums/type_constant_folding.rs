#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum Type {
  #[default]
  Unknown,
  Nil,
  Boolean,
  Number,
  Integer,
  Vector,
  String,
  Table,
}
