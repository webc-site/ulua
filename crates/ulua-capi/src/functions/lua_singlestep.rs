//! 本文件对应 `ulua_lua_singlestep` 导出符号（源：ulua-vm/src/functions/lua_singlestep.rs）。
//! vm 侧已 B 档前移为 `&mut LuaState` 接收者，无法再由 `functions/shells.rs` 中透传裸指针的
//! 共用宏 `capi_shell_l_int!` 直呼（宏体语义不得改，72+ 同形壳零行为变化），故本壳从宏模板
//! 退役、写显式 `extern "C-unwind"` 一行调用，与 `lua_l_checkudata.rs` 等表示适配显式壳
//! 统形：唯一差异是在本帧把 `l` 重建为独占可变引用后转调。
use core::ffi::c_int;

use ulua_vm::{functions::lua_singlestep, records::lua_state::LuaState};

/// # Safety
/// C ABI 导出壳（符号 `ulua_lua_singlestep`），除把 `l` 在本帧重建为独占可变引用外，仅透传
/// 至 `ulua_vm::functions::lua_singlestep::lua_singlestep(&mut *l, enabled)`，零业务逻辑。
/// 调用方须保证：
/// - `l`：指向由本 VM 创建的合法 `LuaState`，非空、对齐，整个调用期间存活，且该期间对此
///   状态无任何其它存活引用或指针并发访问（本帧将整实例写权限经 `&mut` 交予被调方做
///   单字段写入）——`&mut *l` 引用重建前提；
/// - `enabled`（`c_int`）：值型开关参数，非零即开，无额外内存前提；
/// - 其余安全前置条件与被调函数的 `# Safety` 契约一致。
#[unsafe(export_name = "ulua_lua_singlestep")]
pub unsafe extern "C-unwind" fn lua_singlestep(l: *mut LuaState, enabled: c_int) {
  // Safety: 契约声明 `l` 为本次调用期内独占驱动的合法 `LuaState`，`&mut *l` 重建满足排他
  // 借用前提；被调方为 safe fn，仅写 `singlestep` 一个布尔字段。
  unsafe { lua_singlestep::lua_singlestep(&mut *l, enabled) }
}
