use core::ptr::null_mut;

use ulua_common::FInt::{LuauCodeGenBlockSize, LuauCodeGenMaxTotalSize};

use crate::{
  functions::create_shared_code_gen_context_code_gen_context_alt_c::create_shared_code_gen_context_usize_usize_allocation_callback_void,
  type_aliases::unique_shared_code_gen_context::UniqueSharedCodeGenContext,
};

pub fn create_shared_code_gen_context() -> UniqueSharedCodeGenContext {
  let block_size = LuauCodeGenBlockSize.get() as usize;
  let max_total_size = LuauCodeGenMaxTotalSize.get() as usize;

  unsafe {
    create_shared_code_gen_context_usize_usize_allocation_callback_void(
      block_size,
      max_total_size,
      null_mut(),
      null_mut(),
    )
  }
}
