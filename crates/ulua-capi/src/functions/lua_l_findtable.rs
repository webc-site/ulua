//! 本文件对应 `ulua_luaL_findtable` 导出符号（源：ulua-vm/src/functions/lua_l_findtable.rs）。
//! vm 侧已前移为可变引用（`&mut LuaState`）接收者，无法再由 `functions/shells.rs` 中
//! 透传裸指针的共用宏 `capi_shell!` 直呼（宏体语义不得改，其余同形壳零行为变化），
//! 故本壳从宏模板退役、写显式 `extern "C-unwind"` 一行调用（`lua_lessthan` 先例）：
//! 唯一差异是在本帧把 `l` 重建为可变引用（`&mut *l`）后转调。
use core::ffi::{c_char, c_int};

use ulua_vm::{functions::lua_l_findtable, records::lua_state::LuaState};

/// # Safety
/// C ABI 导出壳（符号 `ulua_luaL_findtable`），除把 `l` 在本帧重建为可变引用（`&mut *l`）外，
/// 仅逐参数透传至 `ulua_vm::functions::lua_l_findtable::lua_l_findtable(l, idx, fname, szhint)`，
/// 零业务逻辑。调用方须保证：
/// - `l`：指向由本 VM 创建的合法 `LuaState`，非空、对齐，整个调用期间存活，且与对该
///   状态的其它访问单线程驱动（不得跨 OS 线程并发）——引用重建前提；
/// - `fname`：可读 NUL 结尾 C 串路径；返回值（`*const c_char`）成功为 NULL，失败时指向
///   调用方 `fname` 缓冲中未解析段的起点，存活期随 `fname`，调用方只读；
/// - 其余安全前置条件与被调方文档所列调用序契约一致。
#[unsafe(export_name = "ulua_luaL_findtable")]
pub unsafe extern "C-unwind" fn lua_l_findtable(
  l: *mut LuaState,
  idx: c_int,
  fname: *const c_char,
  szhint: c_int,
) -> *const c_char {
  // Safety: 契约声明 `l` 为整个调用期间存活的合法 `LuaState`；本帧引用重建
  // 即时结束借用窗口。
  unsafe { lua_l_findtable::lua_l_findtable(&mut *l, idx, fname, szhint) }
}
