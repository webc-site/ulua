use crate::{
  enums::ir_op_kind::IrOpKind,
  records::{ir_inst::IrInst, ir_op::IrOp},
};

/// C++ `OPT_OP_C` 只读形态：缺槽或 None 时返回空操作数
#[inline]
pub fn opt_op_c_ref(inst: &IrInst) -> IrOp {
  if 2 < inst.ops.size() && inst.ops[2].kind() != IrOpKind::None {
    inst.ops[2]
  } else {
    IrOp { kind_and_index: 0 }
  }
}
