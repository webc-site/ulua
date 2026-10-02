//! 本文件对应 `ulua_lua_g_pusherror` 导出符号（源：ulua-vm/src/functions/lua_g_pusherror.rs）。
//! vm 侧已前移 `&mut LuaState` 引用形，无法再由 `functions/shells.rs` 中透传裸形的共用
//! 宏 `capi_shell!` 直呼（宏体语义不得改，同族其余透传壳零行为变化），故本壳从宏模板
//! 退役、按本树 `functions/lua_xmove.rs`/`lua_status.rs`/`lua_setlightuserdataname.rs`
//! 显式壳先例写 `extern "C-unwind"` 一行调用：唯一差异是在本帧把 `l` 重建为独占引用后转调。
use core::ffi::c_char;

use ulua_vm::{functions::lua_g_pusherror, records::lua_state::LuaState};

/// # Safety
/// C ABI 导出壳（符号 `ulua_lua_g_pusherror`），除把 `l` 在本帧重建为独占引用外，仅
/// 透传至 `ulua_vm::functions::lua_g_pusherror::lua_g_pusherror(&mut *l, error)`，
/// 零业务逻辑。调用方须保证：
/// - `l`：指向由本 VM 创建的合法 `LuaState`，非空、对齐，整个调用期间存活，且与对该
///   状态的其它访问单线程独占驱动（不得跨 OS 线程并发）——`&mut *l` 引用重建前提（宏
///   壳旧契约同款首条「非空」，本壳不新增运行期判空位、行为逐位中性；单 state 实参无
///   同态重叠面，判空/守卫先于引用构造律在本壳平凡成立）；
/// - `error`（`*const c_char`）：指向 NUL 结尾的只读串缓冲，对齐且在调用期间存活——
///   裸形透传不切片化，本壳不解引用（被调方经 `cstr_bytes_ref` 门面消费，null 与否以
///   被调方文档契约为准）；
/// - 其余安全前置条件与被调函数的文档契约一致。
#[unsafe(export_name = "ulua_lua_g_pusherror")]
pub unsafe extern "C-unwind" fn lua_g_pusherror(l: *mut LuaState, error: *const c_char) {
  // Safety: 契约声明 `l` 为整个调用期间存活的合法 `LuaState`，本帧 `&mut *l` 重建即时
  // 结束借用窗口、不跨调用持有；被调方只检查栈后压错串（r16-v7：callee 已 safe 化，
  // 外层 unsafe 包裹随 callee 消亡，仅存引用重建内联窗——`lua_status`/`lua_xmove`
  // 显式壳同款终形）。
  lua_g_pusherror::lua_g_pusherror(unsafe { &mut *l }, error)
}
