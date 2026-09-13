#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u32)]
#[derive(Default)]
pub enum IrOpKind {
  #[default]
  None,
  Undef,
  Constant,
  Condition,
  Inst,
  Block,
  VmReg,
  VmConst,
  VmUpvalue,
  VmExit,
}
