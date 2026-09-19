use crate::{
  enums::ir_op_kind::IrOpKind,
  records::{ir_inst::IrInst, ir_op::IrOp},
};

pub fn opt_op_c(inst: IrInst) -> IrOp {
  if 2 < inst.ops.size() && inst.ops[2].kind() != IrOpKind::None {
    inst.ops[2]
  } else {
    IrOp { kind_and_index: 0 }
  }
}
