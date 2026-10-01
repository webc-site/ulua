//! vm 侧已前移为独占引用（`&mut *l`）接收者（r3 vm 门面族），无法再由 `functions/shells.rs` 中
//! 透传裸指针的共用宏 `capi_shell!` 直呼（宏体语义不得改，其余同形壳零行为变化），故本壳
//! 从宏模板退役、写显式 `extern "C-unwind"` 一行调用（`lua_status.rs` 先例）：唯一差异
//! 是在本帧把 `l` 重建为独占引用（`&mut *l`）后转调。
use ulua_vm::{
  functions::lua_l_buffinit,
  records::{lua_l_strbuf::LuaLStrbuf, lua_state::LuaState},
};

/// # Safety
/// C ABI 导出壳（符号 `ulua_lua_l_buffinit`），除把 `l` 在本帧重建为独占引用（`&mut *l`）外，仅透传至
/// `ulua_vm::functions::lua_l_buffinit::lua_l_buffinit`，零业务逻辑。调用方须保证：
/// - `l`：指向由本 VM 创建的合法 `LuaState`，非空、对齐，整个调用期间存活，且与对该状态的
///   其它访问单线程驱动（不得跨 OS 线程并发）——引用重建前提；
/// - `b`（`*mut LuaLStrbuf`）：指向存活的 `LuaLStrbuf`（C 侧 `luaL_Buffer` 对应物），
///   非空、对齐、可写，调用期间不被并发改写；
/// - 其余安全前置条件与被调函数的 `# Safety` 契约一致。
#[unsafe(export_name = "ulua_lua_l_buffinit")]
pub unsafe extern "C-unwind" fn lua_l_buffinit(l: *mut LuaState, b: *mut LuaLStrbuf) {
  // Safety: 契约声明 `l` 为整个调用期间存活的合法 `LuaState`；本帧引用重建
  // 即时结束借用窗口。
  lua_l_buffinit::lua_l_buffinit(unsafe { &mut *l }, unsafe { &mut *b })
}
