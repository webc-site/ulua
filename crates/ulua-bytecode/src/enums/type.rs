#[derive(
  Debug, Clone, Copy, PartialEq, Eq, Hash, strum::FromRepr, strum::IntoStaticStr, strum::Display,
)]
#[repr(i32)]
pub enum Type {
  Nil = 0,
  Boolean = 1,
  Number = 2,
  Integer = 3,
  /// cpp `Constant::Type_Vectorf`
  Vector = 4,
  /// cpp `Constant::Type_Vectord`（`BytecodeBuilder.h:203-214`）
  Vectord = 5,
  String = 6,
  Import = 7,
  Table = 8,
  Closure = 9,
  ClassShape = 10,
}
