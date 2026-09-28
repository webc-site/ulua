use core::ffi::c_int;

use ulua_vm::records::lua_state::LuaState;

/// # Safety
/// C ABI 导出壳（符号 `ulua_lua_pushboolean`），调用 `(*l).push_boolean(b != 0)`。调用方须保证：
/// - `l`：指向由本 VM 创建的合法 `LuaState`，非空、对齐，整个调用期间存活，且与对该状态的其它访问单线程驱动（不得跨 OS 线程并发）；
/// - `b` 为 C int 布尔值；
/// - 其余安全前置条件与被调方法的 `# Safety` 契约一致。
#[unsafe(export_name = "ulua_lua_pushboolean")]
pub unsafe extern "C-unwind" fn lua_pushboolean(l: *mut LuaState, b: c_int) {
  // Safety: C ABI 导出壳，l 为有效 LuaState*。
  unsafe { (*l).push_boolean(b != 0) }
}
