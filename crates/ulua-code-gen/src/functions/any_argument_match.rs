use crate::{
  functions::is_pseudo::is_pseudo,
  records::{ir_inst::IrInst, ir_op::IrOp},
};

pub fn any_argument_match<F>(inst: &IrInst, mut func: F) -> bool
where
  F: FnMut(&IrOp) -> bool,
{
  if is_pseudo(inst.cmd) {
    return false;
  }

  for op in &inst.ops {
    if func(op) {
      return true;
    }
  }
  false
}
