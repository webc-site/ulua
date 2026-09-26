use crate::{
  functions::get_op_ir_data::get_op_mut,
  records::{ir_inst::IrInst, ir_op::IrOp},
};

/// C++ 的 `OP_A(inst)`——指令的第一个操作数。
#[inline]
pub fn op_a(inst: &mut IrInst) -> IrOp {
  *get_op_mut(inst, 0)
}
