use crate::{enums::ir_op_kind::IrOpKind, records::ir_op::IrOp};

pub fn vm_const_op(op: IrOp) -> i32 {
  // The CODEGEN_ASSERT macro in this crate's current state has compilation issues
  // with its internal calls to ulua_common and core::arch.
  // We mirror the behavior of vmUpvalueOp by using a standard debug_assert!
  // to ensure the IR operand is of the expected kind before returning the index.
  debug_assert!(op.kind() == IrOpKind::VmConst);
  op.index() as i32
}
