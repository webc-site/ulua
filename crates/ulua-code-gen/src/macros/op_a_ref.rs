use crate::records::{ir_inst::IrInst, ir_op::IrOp};

/// C++ `OP_A` 只读形态：越界时返回缺省操作数
/// （`op_a` 引用版越界时 resize 原指令，只读调用点不应有此副作用）
#[inline]
pub fn op_a_ref(inst: &IrInst) -> IrOp {
  if 0 < inst.ops.size() {
    inst.ops[0]
  } else {
    IrOp::default()
  }
}
