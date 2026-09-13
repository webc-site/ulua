use crate::{
  functions::get_op_ir_data::get_op_mut,
  records::{ir_inst::IrInst, ir_op::IrOp},
};

/// C++ `OP_A(inst)` — the instruction's first operand.
#[inline]
pub fn op_a(inst: &mut IrInst) -> IrOp {
  *get_op_mut(inst, 0)
}
