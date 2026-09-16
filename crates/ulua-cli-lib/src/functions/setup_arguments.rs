//! 由 ulua-cli-test 与 ulua-repl-cli 共同上移：将 Rust 参数切片压入 Lua 栈，
//! 对应 C++ `CLI/src/Repl.cpp` 的 `setupArguments`。两侧实现逐行相同，仅
//! `lua_State` 导入路径不同（`records` 与 `type_aliases` 指向同一类型）。

use core::ffi::c_char;

use ulua_vm::{
  functions::{lua_checkstack::lua_checkstack, lua_pushlstring::lua_pushlstring},
  records::lua_state::lua_State,
};

/// # Safety
///
/// `l` 必须是有效、活跃的 `lua_State` 指针。
pub unsafe fn setup_arguments(l: *mut lua_State, args: &[impl AsRef<str>]) {
  unsafe {
    lua_checkstack(l, args.len() as i32);
    for arg in args {
      let s = arg.as_ref();
      lua_pushlstring(l, s.as_ptr() as *const c_char, s.len());
    }
  }
}
