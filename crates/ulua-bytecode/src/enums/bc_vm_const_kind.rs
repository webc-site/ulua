#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BcVmConstKind {
  Nil,
  Boolean,
  Number,
  /// cpp `BcVmConstKind::Vectorf`
  Vector,
  /// cpp `BcVmConstKind::Vectord`（`BytecodeGraph.h:137-146`）
  Vectord,
  String,
  Import,
  Table,
  Closure,
  Integer,
  /// cpp: `BcVmConstKind::ClassShape`（`cpp/Bytecode/include/Luau/BytecodeGraph.h:148`）
  ClassShape,
}
