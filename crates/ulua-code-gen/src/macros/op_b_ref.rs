use crate::records::{ir_inst::IrInst, ir_op::IrOp};

/// C++ `OP_B` 只读形态：越界时返回缺省操作数
/// （按值版 `op_b` 克隆体上的 resize 外部不可见，故语义等价）
#[inline]
pub fn op_b_ref(inst: &IrInst) -> IrOp {
  if 1 < inst.ops.size() {
    inst.ops[1]
  } else {
    IrOp::default()
  }
}
