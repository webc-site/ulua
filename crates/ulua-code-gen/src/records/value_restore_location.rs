use crate::{
  enums::{ir_cmd::IrCmd, ir_value_kind::IrValueKind},
  records::ir_op::IrOp,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(C)]
pub struct ValueRestoreLocation {
  pub op: IrOp,
  pub kind: IrValueKind,
  pub conversion_cmd: IrCmd,
  pub lazy: bool,
}

impl Default for ValueRestoreLocation {
  fn default() -> Self {
    Self {
      op: IrOp { kind_and_index: 0 },
      kind: IrValueKind::Unknown,
      conversion_cmd: IrCmd::NOP,
      lazy: false,
    }
  }
}
