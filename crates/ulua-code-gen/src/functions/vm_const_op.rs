use crate::{enums::ir_op_kind::IrOpKind, records::ir_op::IrOp};

pub fn vm_const_op(op: IrOp) -> i32 {
  // 本 crate 当前状态下 CODEGEN_ASSERT 宏会因其内部调用
  // ulua_common 与 core::arch 而产生编译问题。
  // 这里参照 vmUpvalueOp 的做法，用标准 debug_assert!
  // 在返回下标前确保 IR 操作数是预期种类。
  debug_assert!(op.kind() == IrOpKind::VmConst);
  op.index() as i32
}
