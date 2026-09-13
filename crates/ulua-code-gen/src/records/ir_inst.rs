use crate::{
  enums::ir_cmd::IrCmd,
  records::{ir_op::IrOp, register_a_64::RegisterA64, register_x_64::RegisterX64},
  type_aliases::ir_ops::IrOps,
};

#[derive(Debug, Clone)]
#[repr(C)]
// Public in the C++ `IrInst`; exposed so the cross-crate test harness (and the
// pub `apply_substitutions`/`IrFunction::instructions` that already reference it)
// can name it.
pub struct IrInst {
  pub cmd: IrCmd,
  pub ops: IrOps,
  pub last_use: u32,
  pub use_count: u16,
  pub reg_x64: RegisterX64,
  pub reg_a64: RegisterA64,
  pub reused_reg: bool,
  pub spilled: bool,
  pub needs_reload: bool,
}

impl IrInst {
  /// Construct an `IrInst` from a command and an operand list, mirroring the
  /// C++ aggregate initialization `IrInst{cmd, {ops...}}`. Used by the test
  /// fixture's `checkEq` to build the expected instruction for comparison.
  pub fn ir_inst_new(cmd: IrCmd, ops: &[IrOp]) -> Self {
    Self {
      cmd,
      ops: ops.iter().cloned().collect(),
      ..Self::default()
    }
  }
}

impl Default for IrInst {
  fn default() -> Self {
    Self {
      cmd: IrCmd::NOP,
      ops: IrOps::new(),
      last_use: 0,
      use_count: 0,
      reg_x64: RegisterX64::NOREG,
      reg_a64: RegisterA64::NOREG,
      reused_reg: false,
      spilled: false,
      needs_reload: false,
    }
  }
}
