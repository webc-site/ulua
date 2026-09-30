#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default, strum::FromRepr)]
#[repr(u32)]
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
