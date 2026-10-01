//! 本文件对应 `ulua_lua_l_optlstring` 导出符号（源：ulua-vm/src/functions/lua_l_optlstring.rs）。
//! vm 侧已前移为可变引用（`&mut LuaState`）接收者，无法再由 `functions/shells.rs` 中
//! 透传裸指针的共用宏 `capi_shell!` 直呼（宏体语义不得改，其余同形壳零行为变化），
//! 故本壳从宏模板退役、写显式 `extern "C-unwind"` 一行调用（`lua_lessthan` 先例）：
//! 唯一差异是在本帧把 `l` 重建为可变引用（`&mut *l`）后转调。
use core::ffi::{c_char, c_int};

use ulua_vm::{functions::lua_l_optlstring, records::lua_state::LuaState};

/// # Safety
/// C ABI 导出壳（符号 `ulua_lua_l_optlstring`），除把 `l` 在本帧重建为可变引用（`&mut *l`）外，
/// 仅逐参数透传至 `ulua_vm::functions::lua_l_optlstring::lua_l_optlstring(l, narg, def, len)`，
/// 零业务逻辑。调用方须保证：
/// - `l`：指向由本 VM 创建的合法 `LuaState`，非空、对齐，整个调用期间存活，且与对该
///   状态的其它访问单线程驱动（不得跨 OS 线程并发）——引用重建前提；
/// - `def`：可读 NUL 结尾 C 串或 NULL；`len`：可写、调用期间存活的 `usize` 出参存储或 NULL；
/// - 返回值（`*const c_char`）：narg 槽为 nil/缺失时原样转还 `def`（可为 NULL）；否则指向
///   栈内串数据，下次压栈/GC 回收后即失效，调用方只读；
/// - 其余安全前置条件与被调方文档所列调用序契约一致。
#[unsafe(export_name = "ulua_lua_l_optlstring")]
pub unsafe extern "C-unwind" fn lua_l_optlstring(
  l: *mut LuaState,
  narg: c_int,
  def: *const c_char,
  len: *mut usize,
) -> *const c_char {
  // Safety: 契约声明 `l` 为整个调用期间存活的合法 `LuaState`；本帧引用重建
  // 即时结束借用窗口。
  unsafe { lua_l_optlstring::lua_l_optlstring(&mut *l, narg, def, len) }
}
