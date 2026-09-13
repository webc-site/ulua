use crate::{
  functions::{get_op_ir_data::get_op_mut, replace_ir_utils::replace_ir_function_ir_op_ir_op},
  records::{ir_function::IrFunction, ir_op::IrOp},
};

pub fn replace_ir_function_ir_inst_operand(
  function: &mut IrFunction,
  inst_idx: u32,
  op_idx: u32,
  replacement: IrOp,
) {
  let function_ptr = function as *mut IrFunction;

  unsafe {
    let original = get_op_mut(
      &mut (&mut (*function_ptr).instructions)[inst_idx as usize],
      op_idx,
    ) as *mut IrOp;
    replace_ir_function_ir_op_ir_op(&mut *function_ptr, &mut *original, replacement);
  }
}
