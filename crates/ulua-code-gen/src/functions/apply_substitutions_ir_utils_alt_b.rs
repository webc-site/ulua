use crate::{
  functions::apply_substitutions_ir_utils::apply_substitutions_ir_function_ir_op,
  records::{ir_function::IrFunction, ir_inst::IrInst},
};

pub fn apply_substitutions_ir_function_ir_inst(function: &mut IrFunction, inst: &mut IrInst) {
  // 单次可变借用切片遍历，替代 C 风格索引循环
  for op in inst.ops.as_mut_slice() {
    apply_substitutions_ir_function_ir_op(function, op);
  }
}
