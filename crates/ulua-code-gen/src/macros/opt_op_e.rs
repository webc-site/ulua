use crate::{
  enums::ir_op_kind::IrOpKind,
  records::{ir_inst::IrInst, ir_op::IrOp},
};

pub fn opt_op_e(inst: IrInst) -> IrOp {
  if 4 < inst.ops.size() && inst.ops[4].kind() != IrOpKind::None {
    inst.ops[4]
  } else {
    IrOp { kind_and_index: 0 }
  }
}
