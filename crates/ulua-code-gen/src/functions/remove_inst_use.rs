use crate::{
  functions::kill_ir_utils::kill_ir_function_ir_inst_at, macros::codegen_assert::CODEGEN_ASSERT,
  records::ir_function::IrFunction,
};

pub fn remove_inst_use(function: &mut IrFunction, inst_idx: u32) {
  let inst = &mut function.instructions[inst_idx as usize];

  CODEGEN_ASSERT!(inst.use_count != 0);
  inst.use_count -= 1;

  // 借用止于末次读；置零后走索引化 kill，函数可变借用不再与指令借用重叠
  let drained = inst.use_count == 0;
  if drained {
    kill_ir_function_ir_inst_at(function, inst_idx);
  }
}
