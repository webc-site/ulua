use crate::{
  enums::ir_op_kind::IrOpKind,
  records::{ir_inst::IrInst, ir_op::IrOp},
};

pub fn opt_op_d(inst: IrInst) -> IrOp {
  if 3 < inst.ops.size() && inst.ops[3].kind() != IrOpKind::None {
    inst.ops[3]
  } else {
    IrOp { kind_and_index: 0 }
  }
}
