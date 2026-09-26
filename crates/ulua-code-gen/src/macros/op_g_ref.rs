use crate::records::{ir_inst::IrInst, ir_op::IrOp};

/// C++ `OP_G` 只读形态：越界时返回缺省操作数
/// （C++ 按值版 `OP_G` 在克隆体上的 resize 外部不可见，Rust 侧仅保留只读形态）
#[inline]
pub fn op_g_ref(inst: &IrInst) -> IrOp {
  if 6 < inst.ops.size() {
    inst.ops[6]
  } else {
    IrOp::default()
  }
}
