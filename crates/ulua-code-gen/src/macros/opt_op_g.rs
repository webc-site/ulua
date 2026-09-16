use crate::{
  enums::ir_op_kind::IrOpKind,
  records::{ir_inst::IrInst, ir_op::IrOp},
};

pub fn opt_op_g(inst: IrInst) -> IrOp {
  if 6 < inst.ops.size() && inst.ops[6].kind() != IrOpKind::None {
    inst.ops[6]
  } else {
    IrOp { kind_and_index: 0 }
  }
}
