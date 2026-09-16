use std::ptr::null_mut;

use crate::{
  enums::ir_op_kind::IrOpKind,
  records::{ir_function::IrFunction, ir_inst::IrInst, ir_op::IrOp},
};

impl IrFunction {
  pub fn as_inst_op(&mut self, op: IrOp) -> *mut IrInst {
    if op.kind() == IrOpKind::Inst {
      // Safety: the C++ source directly indexes into instructions[op.index] without bounds
      // checking, assuming the index is valid. We match that behavior here.
      unsafe { self.instructions.as_mut_ptr().add(op.index() as usize) }
    } else {
      null_mut()
    }
  }
}
