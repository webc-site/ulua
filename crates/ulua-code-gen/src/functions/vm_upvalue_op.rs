use crate::{enums::ir_op_kind::IrOpKind, records::ir_op::IrOp};

pub fn vm_upvalue_op(op: IrOp) -> u32 {
  // Keep the same runtime check behavior as the original C++ helper.
  // Note: `CODEGEN_ASSERT` is expected to work in this crate; the previously reported
  // compile errors were due to the macro expansion inside this file, so we avoid
  // relying on it and use a local assertion instead.
  debug_assert!(op.kind() == IrOpKind::VmUpvalue);
  op.index()
}
