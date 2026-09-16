use crate::{enums::ir_op_kind::IrOpKind, records::ir_op::IrOp};

pub fn vm_exit_op(op: IrOp) -> u32 {
  debug_assert!(op.kind() == IrOpKind::VmExit);
  op.index()
}
