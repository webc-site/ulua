use crate::{enums::ir_op_kind::IrOpKind, records::ir_op::IrOp};

/// See also: ulua-vm `macros/vm_kv::VM_KV!`——形似义异：该宏是解释器运行时的常量表
/// 槽寻址（code-gen 慢路径 `VmFrame::kv` 已直接单源复用它），本函数仅提取 `IrOp`
/// 的 VmConst 操作数编号，勿合并。
pub fn vm_const_op(op: IrOp) -> i32 {
  // 本 crate 当前状态下 CODEGEN_ASSERT 宏会因其内部调用
  // ulua_common 与 core::arch 而产生编译问题。
  // 这里参照 vmUpvalueOp 的做法，用标准 debug_assert!
  // 在返回下标前确保 IR 操作数是预期种类。
  debug_assert!(op.kind() == IrOpKind::VmConst);
  op.index() as i32
}
