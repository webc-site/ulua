//! vm 侧已前移为独占引用（`&mut *l`）接收者（r3 vm 门面族），无法再由 `functions/shells.rs` 中
//! 透传裸指针的共用宏 `capi_shell!` 直呼（宏体语义不得改，其余同形壳零行为变化），故本壳
//! 从宏模板退役、写显式 `extern "C-unwind"` 一行调用（`lua_status.rs` 先例）：唯一差异
//! 是在本帧把 `l` 重建为独占引用（`&mut *l`）后转调。
use ulua_vm::{
  functions::lua_a_pushvalue, records::lua_state::LuaState, type_aliases::t_value::TValue,
};

/// # Safety
/// C ABI 导出壳（符号 `ulua_luaA_pushvalue`），除把 `l` 在本帧重建为独占引用（`&mut *l`）外，仅透传至
/// `ulua_vm::functions::lua_a_pushvalue::lua_a_pushvalue`，零业务逻辑。调用方须保证：
/// - `l`：指向由本 VM 创建的合法 `LuaState`，非空、对齐，整个调用期间存活，且与对该状态的
///   其它访问单线程驱动（不得跨 OS 线程并发）——引用重建前提；
/// - `o`（`*const TValue`）：指向合法、地址稳定（栈槽或被 GC 持有）的 `TValue`，
///   调用期间只读存活；
/// - 其余安全前置条件与被调函数的 `# Safety` 契约一致。
#[unsafe(export_name = "ulua_luaA_pushvalue")]
pub unsafe extern "C-unwind" fn lua_a_pushvalue(l: *mut LuaState, o: *const TValue) {
  // Safety: 契约声明 `l` 为整个调用期间存活的合法 `LuaState`；本帧引用重建
  // 即时结束借用窗口。
  unsafe { lua_a_pushvalue::lua_a_pushvalue(&mut *l, &*o) }
}
