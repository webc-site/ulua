use crate::{enums::ir_op_kind::IrOpKind, records::ir_op::IrOp};

/// See also: ulua-vm `macros/vm_reg::VM_REG!`——形似义异：该宏是解释器运行时的
/// 栈槽寻址（base+i 解引用窗口），本函数仅提取 `IrOp` 的 VmReg 操作数编号，勿合并。
pub fn vm_reg_op(op: IrOp) -> i32 {
  debug_assert!(op.kind() == IrOpKind::VmReg);
  op.index() as i32
}
