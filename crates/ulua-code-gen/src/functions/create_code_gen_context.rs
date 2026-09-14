use core::ptr::null_mut;

use ulua_common::FInt::{LuauCodeGenBlockSize, LuauCodeGenMaxTotalSize};
use ulua_vm::records::lua_state::lua_State;

use crate::functions::create_code_gen_context_alt_c::create_lua_state_usize_usize_allocation_callback_void;

pub fn create(l: *mut lua_State) {
  let block_size = LuauCodeGenBlockSize.get() as usize;
  let max_total_size = LuauCodeGenMaxTotalSize.get() as usize;

  create_lua_state_usize_usize_allocation_callback_void(
    l,
    block_size,
    max_total_size,
    null_mut(),
    null_mut(),
  );
}
