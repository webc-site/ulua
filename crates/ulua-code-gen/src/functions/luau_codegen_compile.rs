use core::ptr::null_mut;

use ulua_vm::records::lua_state::lua_State;

use crate::{
  functions::compile_internal::compile_internal, records::compilation_options::CompilationOptions,
};

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
pub unsafe fn luau_codegen_compile(l: *mut lua_State, idx: i32) {
  unsafe {
    let _ = compile_internal(&None, l, idx, &CompilationOptions::default(), null_mut());
  }
}
