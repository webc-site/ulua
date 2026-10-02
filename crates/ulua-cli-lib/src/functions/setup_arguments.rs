//! 由 ulua-cli-test 与 ulua-repl-cli 共同上移：将 Rust 参数切片压入 Lua 栈，
//! 对应 C++ `CLI/src/Repl.cpp` 的 `setupArguments`。两侧实现逐行相同。

use ulua_vm::{
  functions::{lua_checkstack::lua_checkstack, lua_pushlstring::lua_pushlstring_bytes},
  records::lua_state::LuaState,
};

/// # Safety
///
/// `l` 必须是有效、活跃的 `LuaState` 指针。
// DELIBERATE DEVIATION（review.md §9.3）：把 Rust 参数切片逐个压入 VM 栈
// （`lua_checkstack`/`lua_pushlstring_bytes`），`*mut LuaState` 解引用为 ulua-vm
// c-API 边界固有形态，故入口保持 `unsafe fn` + `# Safety` 契约（review.md §2）。
// 串以 `&[u8]` 定长交 VM 拷贝（review.md §10：裸指针配点收进 VM bytes 门面，
// 调用点不再散落 `.as_ptr().cast()`）。
pub unsafe fn setup_arguments(l: *mut LuaState, args: &[impl AsRef<str>]) {
  // Safety: `# Safety` 契约保证 `l` 为活跃状态机；按 `args.len()` 预留后再逐个压栈。
  unsafe { lua_checkstack(l, args.len() as i32) };
  for arg in args {
    let s = arg.as_ref();
    // Safety: `l` 同上；`bytes` 门面以切片长度取字节，VM 当调用即拷入新串对象，
    // 借用不跨调用持有；`&mut *l` 一次性重借用即核心期望的接收者形。
    unsafe { lua_pushlstring_bytes(&mut *l, s.as_bytes()) };
  }
}
