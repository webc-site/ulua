//! 本文件对应 `ulua_lua_pushnumber` 导出符号（源：ulua-vm `LuaState::push_number` 方法；vm 侧
//! 无同名自由函数，故为直调方法的显式壳）：唯一适配是把 `l` 重建为独占引用后调用。
use ulua_vm::records::lua_state::LuaState;

/// # Safety
/// C ABI 导出壳（符号 `ulua_lua_pushnumber`），除把 `l` 重建为独占引用（`&mut *l`）后调用
/// `push_number(n)` 外零业务逻辑。调用方须保证：
/// - `l`：指向由本 VM 创建的合法 `LuaState`，非空、对齐，整个调用期间存活，且与对该状态的其它访问单线程驱动（不得跨 OS 线程并发）；
/// - `n` 为 f64 浮点值；
/// - 其余安全前置条件与被调方法的 `# Safety` 契约一致。
#[unsafe(export_name = "ulua_lua_pushnumber")]
pub unsafe extern "C-unwind" fn lua_pushnumber(l: *mut LuaState, n: f64) {
  // Safety: 契约声明 `l` 为整个调用期间存活的合法 `LuaState`，`&mut *l` 重建即时结束借用窗口。
  (unsafe { &mut *l }).push_number(n)
}
