use crate::{
  functions::kill_ir_utils::kill_ir_function_ir_block_at, macros::codegen_assert::CODEGEN_ASSERT,
  records::ir_function::IrFunction,
};

pub fn remove_block_use(function: &mut IrFunction, block_idx: u32) {
  let block = &mut function.blocks[block_idx as usize];

  CODEGEN_ASSERT!(block.use_count != 0);
  block.use_count -= 1;

  // entry block 有隐式 use，永远不会被移除
  let kill = block.use_count == 0 && block_idx != 0;
  if kill {
    kill_ir_function_ir_block_at(function, block_idx as usize);
  }
}
