use crate::{
  functions::is_pseudo::is_pseudo,
  records::{ir_inst::IrInst, ir_op::IrOp},
};

pub fn visit_arguments<F>(inst: &mut IrInst, mut func: F)
where
  F: FnMut(IrOp),
{
  if is_pseudo(inst.cmd) {
    return;
  }

  for op in inst.ops.iter() {
    func(*op);
  }
}
