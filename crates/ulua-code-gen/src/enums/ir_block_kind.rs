#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum IrBlockKind {
  Bytecode,
  Fallback,
  Internal,
  Linearized,
  ExitSync,
  Dead,
}
