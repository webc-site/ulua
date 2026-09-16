use crate::{
  enums::ir_op_kind::IrOpKind,
  functions::{remove_block_use::remove_block_use, remove_inst_use::remove_inst_use},
  records::{ir_function::IrFunction, ir_op::IrOp},
};

pub fn remove_use(function: &mut IrFunction, op: IrOp) {
  if op.kind() == IrOpKind::Inst {
    remove_inst_use(function, op.index());
  } else if op.kind() == IrOpKind::Block {
    remove_block_use(function, op.index());
  }
}
