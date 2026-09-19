#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
pub enum Type {
  Nil,
  Boolean,
  Number,
  Integer,
  /// cpp `Constant::Type_Vectorf`
  Vector,
  /// cpp `Constant::Type_Vectord`（`BytecodeBuilder.h:203-214`）
  Vectord,
  String,
  Import,
  Table,
  Closure,
  ClassShape,
}
