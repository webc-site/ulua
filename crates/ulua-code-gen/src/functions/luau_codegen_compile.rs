use core::ptr::null_mut;

use ulua_vm::records::lua_state::LuaState;

use crate::{
  functions::compile_internal::compile_internal, records::compilation_options::CompilationOptions,
};

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
pub unsafe fn luau_codegen_compile(l: *mut LuaState, idx: i32) {
  // Safety: compile_internal 依 `# Safety` 契约消费 l（存活 LuaState*）与 idx（界内栈位），
  // null_mut() 为“无 meta 输出”的默认形参（对齐 C++ nullptr），返回值显式丢弃与 C++ 一致。
  unsafe {
    let _ = compile_internal(&None, l, idx, &CompilationOptions::default(), null_mut());
  }
}
