use crate::{
  functions::get_op_ir_data::get_op_mut,
  records::{ir_inst::IrInst, ir_op::IrOp},
};

#[inline]
pub fn op_d(mut inst: IrInst) -> IrOp {
  *get_op_mut(&mut inst, 3)
}
