use crate::{
  functions::apply_substitutions_ir_utils::apply_substitutions_ir_function_ir_op,
  records::{ir_function::IrFunction, ir_inst::IrInst},
};

pub fn apply_substitutions_ir_function_ir_inst(function: &mut IrFunction, inst: &mut IrInst) {
  let n = inst.ops.size();
  for i in 0..n {
    let op = &mut inst.ops.as_mut_slice()[i as usize];
    apply_substitutions_ir_function_ir_op(function, op);
  }
}
