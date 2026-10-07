//! 本文件对应 `ulua_lua_pushboolean` 导出符号（源：ulua-vm `LuaState::push_boolean` 方法；vm 侧
//! 无同名自由函数，故为直调方法的显式壳）：适配点是把 `l` 重建为独占引用、C int 布尔折算
//! 为 Rust bool。
use core::ffi::c_int;

use ulua_vm::records::lua_state::LuaState;

/// # Safety
/// C ABI 导出壳（符号 `ulua_lua_pushboolean`），除把 `l` 重建为独占引用（`&mut *l`）、`b` 折算
/// 为 bool 后调用 `push_boolean(b != 0)` 外零业务逻辑。调用方须保证：
/// - `l`：指向由本 VM 创建的合法 `LuaState`，非空、对齐，整个调用期间存活，且与对该状态的其它访问单线程驱动（不得跨 OS 线程并发）；
/// - `b` 为 C int 布尔值；
/// - 其余安全前置条件与被调方法的 `# Safety` 契约一致。
#[unsafe(export_name = "ulua_lua_pushboolean")]
pub unsafe extern "C-unwind" fn lua_pushboolean(l: *mut LuaState, b: c_int) {
  // Safety: 契约声明 `l` 为整个调用期间存活的合法 `LuaState`，`&mut *l` 重建即时结束借用窗口。
  (unsafe { &mut *l }).push_boolean(b != 0)
}
