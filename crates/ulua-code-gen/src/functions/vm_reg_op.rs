use crate::{enums::ir_op_kind::IrOpKind, records::ir_op::IrOp};

pub fn vm_reg_op(op: IrOp) -> i32 {
  debug_assert!(op.kind() == IrOpKind::VmReg);
  op.index() as i32
}
