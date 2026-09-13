use core::ptr;

use crate::{
  enums::bc_op_kind::BcOpKind,
  records::{bc_function::BcFunction, bc_inst::BcInst, bc_op::BcOp},
};

impl BcFunction {
  pub fn as_inst_op(&self, op: BcOp) -> *mut BcInst {
    if op.kind == BcOpKind::Inst {
      if (op.index as usize) < self.instructions.len() {
        &self.instructions[op.index as usize] as *const BcInst as *mut BcInst
      } else {
        ptr::null_mut()
      }
    } else {
      ptr::null_mut()
    }
  }
}
