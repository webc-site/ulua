//! 本文件对应 `ulua_lua_resumeerror` 导出符号（源：ulua-vm/src/functions/lua_resumeerror.rs）。
//! vm 侧 `l` 已前移 `&mut LuaState` 引用形，无法再由 `functions/shells.rs` 中透传裸指针的
//! 共用宏 `capi_shell!` 直呼（宏体语义不得改，同族其余透传壳零行为变化），故本壳从宏模板
//! 退役、写显式 `extern "C-unwind"` 一行调用，与 `functions/lua_status.rs` 先例统形：
//! 唯一差异是在本帧把 `l` 重建为独占引用后转调，`from` 按其可空契约原样透传裸指针。
use core::ffi::c_int;

use ulua_vm::{functions::lua_resumeerror, records::lua_state::LuaState};

/// # Safety
/// C ABI 导出壳（符号 `ulua_lua_resumeerror`），除把 `l` 在本帧重建为独占引用外，仅
/// 透传至 `ulua_vm::functions::lua_resumeerror::lua_resumeerror(&mut *l, from)`，零业务
/// 逻辑。调用方须保证：
/// - `l`：指向由本 VM 创建的合法 `LuaState`，非空、对齐，整个调用期间存活，且与对该状态的
///   其它访问单线程驱动（不得跨 OS 线程并发）——`&mut *` 引用重建前提；
/// - `from`：按被调方契约可为 null（主状态恢复），非空时须为存活恢复发起方；
/// - 其余安全前置条件与被调函数的文档契约一致。
#[unsafe(export_name = "ulua_lua_resumeerror")]
pub unsafe extern "C-unwind" fn lua_resumeerror(l: *mut LuaState, from: *mut LuaState) -> c_int {
  // Safety: 契约声明 `l` 为整个调用期间存活的合法 `LuaState`，本帧 `&mut *` 重建
  // 即时结束借用窗口、不跨调用持有；`from` 以裸指针原样转调（可空契约见上）。
  lua_resumeerror::lua_resumeerror(unsafe { &mut *l }, from)
}
