use crate::{
  enums::ir_op_kind::IrOpKind,
  records::{ir_inst::IrInst, ir_op::IrOp},
};

pub fn opt_op_a(inst: IrInst) -> IrOp {
  if 0 < inst.ops.size() && inst.ops[0].kind() != IrOpKind::None {
    inst.ops[0]
  } else {
    IrOp { kind_and_index: 0 }
  }
}
