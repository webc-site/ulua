use crate::records::{ir_inst::IrInst, ir_op::IrOp};

/// C++ `OP_F` 只读形态：越界时返回缺省操作数
/// （C++ 按值版 `OP_F` 在克隆体上的 resize 外部不可见，Rust 侧仅保留只读形态）
#[inline]
pub fn op_f_ref(inst: &IrInst) -> IrOp {
  if 5 < inst.ops.size() {
    inst.ops[5]
  } else {
    IrOp::default()
  }
}
