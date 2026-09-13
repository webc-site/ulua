#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
pub enum Type {
  Nil,
  Boolean,
  Number,
  Integer,
  Vector,
  String,
  Import,
  Table,
  Closure,
  ClassShape,
}
