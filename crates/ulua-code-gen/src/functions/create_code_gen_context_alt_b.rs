use core::ffi::c_void;

use ulua_common::FInt::{LuauCodeGenBlockSize, LuauCodeGenMaxTotalSize};

use crate::{
  functions::create_code_gen_context_alt_c::create_lua_state_usize_usize_allocation_callback_void,
  type_aliases::{allocation_callback::AllocationCallback, lua_state::lua_State},
};

pub fn create_lua_state_allocation_callback_void(
  l: *mut lua_State,
  allocation_callback: *mut AllocationCallback,
  allocation_callback_context: *mut c_void,
) {
  create_lua_state_usize_usize_allocation_callback_void(
    l,
    LuauCodeGenBlockSize.get() as usize,
    LuauCodeGenMaxTotalSize.get() as usize,
    allocation_callback,
    allocation_callback_context,
  );
}
