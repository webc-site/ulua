use crate::{
  enums::ir_op_kind::IrOpKind,
  records::{ir_inst::IrInst, ir_op::IrOp},
};

pub fn opt_op_f(inst: IrInst) -> IrOp {
  if 5 < inst.ops.size() && inst.ops[5].kind() != IrOpKind::None {
    inst.ops[5]
  } else {
    IrOp { kind_and_index: 0 }
  }
}
