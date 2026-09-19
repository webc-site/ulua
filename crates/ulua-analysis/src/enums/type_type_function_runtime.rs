#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Type {
  NilType,
  Boolean,
  Number,
  Integer,
  String,
  Thread,
  Buffer,
}
