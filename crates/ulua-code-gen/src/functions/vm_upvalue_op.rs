use crate::{enums::ir_op_kind::IrOpKind, records::ir_op::IrOp};

pub fn vm_upvalue_op(op: IrOp) -> u32 {
  // 与原 C++ helper 保持相同的运行期检查行为。
  // 注：本 crate 中 `CODEGEN_ASSERT` 预期可用；此前报告的
  // 编译错误源于宏在本文件内展开，故这里不依赖它，
  // 改用局部断言。
  debug_assert!(op.kind() == IrOpKind::VmUpvalue);
  op.index()
}
