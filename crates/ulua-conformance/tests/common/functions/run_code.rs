use core::{
  ffi::{c_char, c_int, c_void},
  ptr::null_mut,
};

use ulua_compiler::functions::luau_compile::luau_compile;
use ulua_vm::{
  functions::{lua_pcall::lua_pcall, luau_load::luau_load},
  macros::lua_multret::LUA_MULTRET,
  records::lua_state::lua_State,
};

use crate::common::functions::c_alloc::c_free;

/// # Safety
/// 调用方须保证满足 C++ 原实现的调用契约。
pub unsafe fn run_code(l: *mut lua_State, source: &str) -> c_int {
  unsafe {
    let mut bytecode_size = 0usize;
    let bytecode = luau_compile(
      source.as_ptr() as *const c_char,
      source.len(),
      null_mut(),
      &mut bytecode_size,
    );

    if luau_load(l, c"test".as_ptr(), bytecode, bytecode_size, 0) != 0 {
      c_free(bytecode.cast());
      return -1;
    }

    c_free(bytecode.cast());
    lua_pcall(l, 0, LUA_MULTRET, 0)
  }
}
