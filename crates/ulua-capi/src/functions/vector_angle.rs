//! 本文件对应 `ulua_vector_angle` 导出符号（源：ulua-vm/src/functions/vector_angle.rs）。
//! vm 侧已前移为独占引用（`&mut *l`）接收者，无法再由 `functions/shells.rs` 中透传裸指针的
//! 共用宏 `capi_libfn_shell_l_cint!` 直呼（宏体语义不得改，其余同形壳零行为变化），故本壳
//! 从宏模板退役、写显式 `extern "C-unwind"` 一行调用（`lua_status.rs` 先例）：唯一差异
//! 是在本帧把 `l` 重建为独占引用（`&mut *l`）后转调。
use core::ffi::c_int;

use ulua_vm::{functions::vector_angle, records::lua_state::LuaState};

/// # Safety
/// C ABI 导出壳（符号 `ulua_vector_angle`），除把 `l` 在本帧重建为独占引用（`&mut *l`）外，
/// 仅透传至 `ulua_vm::functions::vector_angle::vector_angle`，零业务逻辑。调用方须保证：
/// - `l`：指向由本 VM 创建的合法 `LuaState`，非空、对齐，整个调用期间存活，且与对该状态的
///   其它访问单线程驱动（不得跨 OS 线程并发）——引用重建前提；
/// - 其余安全前置条件与被调函数的 `# Safety` 契约一致。
#[unsafe(export_name = "ulua_vector_angle")]
pub unsafe extern "C-unwind" fn vector_angle(l: *mut LuaState) -> c_int {
  // Safety: 契约声明 `l` 为整个调用期间存活的合法 `LuaState`；本帧引用重建
  // 即时结束借用窗口。
  vector_angle::vector_angle(unsafe { &mut *l })
}
