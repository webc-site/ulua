use core::{ffi::c_int, ptr::null_mut};

use ulua_compiler::functions::luau_compile::luau_compile;
use ulua_vm::{
  functions::{
    lua_insert::lua_insert, lua_l_checklstring::lua_l_checklstring, lua_pushnil::lua_pushnil,
    lua_setsafeenv::lua_setsafeenv, luau_load::luau_load,
  },
  luaL_optstring,
  macros::lua_environindex::LUA_ENVIRONINDEX,
  records::lua_state::lua_State,
};

use crate::common::functions::c_alloc::c_free;

/// # Safety
///
/// Pointer arguments must be valid, aligned, and properly initialized.
pub unsafe extern "C-unwind" fn lua_loadstring(l: *mut lua_State) -> c_int {
  unsafe {
    let mut len = 0usize;
    let source = lua_l_checklstring(l, 1, &mut len);
    let chunkname = luaL_optstring!(l, 2, source);

    lua_setsafeenv(l, LUA_ENVIRONINDEX, 0);

    let mut bytecode_size = 0usize;
    let bytecode = luau_compile(source, len, null_mut(), &mut bytecode_size);
    let result = luau_load(l, chunkname, bytecode, bytecode_size, 0);
    c_free(bytecode.cast());

    if result == 0 {
      return 1;
    }

    lua_pushnil(l);
    lua_insert(l, -2);
    2
  }
}
