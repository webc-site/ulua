use crate::{enums::ir_value_kind::IrValueKind, records::ir_op::IrOp};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(C)]
pub struct StoreLocationHint {
  pub op: IrOp,
  pub inst_idx: u32,
  pub kind: IrValueKind,
}
