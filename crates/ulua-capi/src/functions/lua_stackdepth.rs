//! 本文件对应 `ulua_lua_stackdepth` 导出符号（源：ulua-vm/src/functions/lua_stackdepth.rs）。
//! vm 侧已 B 档前移为 `&LuaState` 接收者，无法再由 `functions/shells.rs` 中透传裸指针的
//! 共用宏 `capi_shell!` 直呼（宏体语义不得改，72+ 同形壳零行为变化），故本壳从宏模板
//! 退役、写显式 `extern "C-unwind"` 一行调用，与 `lua_l_checkudata.rs` 等表示适配显式壳
//! 统形：唯一差异是在本帧把 `l` 重建为只读引用后转调。
use core::ffi::c_int;

use ulua_vm::{functions::lua_stackdepth, records::lua_state::LuaState};

/// # Safety
/// C ABI 导出壳（符号 `ulua_lua_stackdepth`），除把 `l` 在本帧重建为只读引用外，仅透传至
/// `ulua_vm::functions::lua_stackdepth::lua_stackdepth(&*l)`，零业务逻辑。调用方须保证：
/// - `l`：指向由本 VM 创建的合法 `LuaState`，非空、对齐，整个调用期间存活，且与对该状态的
///   其它访问单线程驱动（不得跨 OS 线程并发），其 `ci`/`base_ci` 落在同一 `base_ci` 数组内
///   且 `ci ≥ base_ci`——`&*l` 引用重建与帧差计算前提；
/// - 其余安全前置条件与被调函数的 `# Safety` 契约一致。
#[unsafe(export_name = "ulua_lua_stackdepth")]
pub unsafe extern "C-unwind" fn lua_stackdepth(l: *mut LuaState) -> c_int {
  // Safety: 契约声明 `l` 为整个调用期间存活的合法 `LuaState`；被调方仅读 `ci`/`base_ci`
  // 两个指针字段取差值、不解引用任何帧内容，本帧 `&*l` 重建即时结束借用窗口。
  unsafe { lua_stackdepth::lua_stackdepth(&*l) }
}
