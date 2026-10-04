//! 由 ulua-cli-test 与 ulua-repl-cli 共同上移：将 Rust 参数切片压入 Lua 栈，
//! 对应 C++ `CLI/src/Repl.cpp` 的 `setupArguments`。两侧实现逐行相同。

use ulua_vm::{
  functions::{lua_checkstack::lua_checkstack, lua_pushlstring::lua_pushlstring_bytes},
  records::lua_state::LuaState,
};

/// 调用序契约（由调用方成立）：`l` 为活跃状态机，且本函数逐个压入 `args`，
/// `checkstack` 预留的槽位数与压入个数一致（`args.len()`）。
// review.md §2/§3 收形：`l` 由裸 `*mut LuaState` 收编为借用 `&mut LuaState`——
// `lua_checkstack` 已是安全引用形，`lua_pushlstring_bytes` 亦收形为引用接收者，
// `&mut *l` 重借用随之消亡。入口降为安全 `fn`；仍保留一处最小 `unsafe` 块，仅因
// `lua_pushlstring_bytes` 为 ulua-vm 侧 `unsafe fn`（其在活跃状态上拷贝定长字节入栈
// 的 VM 边界契约），非本函数解引用调用方裸指针。
pub fn setup_arguments(l: &mut LuaState, args: &[impl AsRef<str>]) {
  lua_checkstack(l, args.len() as i32);
  for arg in args {
    let s = arg.as_ref();
    // Safety: `lua_pushlstring_bytes` 为 ulua-vm `unsafe fn`；`l` 为活跃状态，
    // `bytes` 门面以切片长度取字节，VM 当调用即拷入新串对象，借用不跨调用持有。
    unsafe { lua_pushlstring_bytes(l, s.as_bytes()) };
  }
}
