use crate::{
  enums::ir_op_kind::IrOpKind,
  records::{ir_inst::IrInst, ir_op::IrOp},
};

pub fn opt_op_b(inst: IrInst) -> IrOp {
  if 1 < inst.ops.size() && inst.ops[1].kind() != IrOpKind::None {
    inst.ops[1]
  } else {
    IrOp { kind_and_index: 0 }
  }
}
