use crate::{
  enums::ir_op_kind::IrOpKind,
  records::{ir_inst::IrInst, ir_op::IrOp},
};

/// C++ `OPT_OP_D` 只读形态：缺槽或 None 时返回空操作数
#[inline]
pub fn opt_op_d_ref(inst: &IrInst) -> IrOp {
  if 3 < inst.ops.size() && inst.ops[3].kind() != IrOpKind::None {
    inst.ops[3]
  } else {
    IrOp { kind_and_index: 0 }
  }
}
