//! 由 ulua-cli-test 与 ulua-repl-cli 共同上移：将 Rust 参数切片压入 Lua 栈，
//! 对应 C++ `CLI/src/Repl.cpp` 的 `setupArguments`。两侧实现逐行相同。

use ulua_vm::{
  functions::{lua_checkstack::lua_checkstack, lua_pushlstring::lua_pushlstring},
  records::lua_state::LuaState,
};

/// # Safety
///
/// `l` 必须是有效、活跃的 `LuaState` 指针。
pub unsafe fn setup_arguments(l: *mut LuaState, args: &[impl AsRef<str>]) {
  // Safety: `# Safety` 契约保证 `l` 为活跃状态机；按 `args.len()` 预留后再逐个压栈。
  unsafe { lua_checkstack(l, args.len() as i32) };
  for arg in args {
    let s = arg.as_ref();
    // Safety: `&str` 的 `as_ptr()` 指向 `s.len()` 个可读字节（Lua 侧按长度取，
    // 不要求 NUL 结尾）；栈槽已由上面的 `lua_checkstack` 预留。
    unsafe { lua_pushlstring(l, s.as_ptr().cast(), s.len()) };
  }
}
