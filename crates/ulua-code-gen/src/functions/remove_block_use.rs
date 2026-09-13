use crate::{
  functions::kill_ir_utils_alt_c::kill_ir_function_ir_block,
  macros::codegen_assert::CODEGEN_ASSERT,
  records::{ir_block::IrBlock, ir_function::IrFunction},
};

pub fn remove_block_use(function: &mut IrFunction, block_idx: u32) {
  let block: *mut IrBlock = &mut function.blocks[block_idx as usize];

  CODEGEN_ASSERT!(unsafe { (*block).use_count } != 0);
  unsafe {
    (*block).use_count -= 1;
  }

  // Entry block is never removed because is has an implicit use
  if unsafe { (*block).use_count } == 0 && block_idx != 0 {
    kill_ir_function_ir_block(function, unsafe { &mut *block });
  }
}
