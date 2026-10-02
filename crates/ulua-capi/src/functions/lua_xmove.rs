//! 本文件对应 `ulua_lua_xmove` 导出符号（源：ulua-vm/src/functions/lua_xmove.rs）。
//! vm 侧已前移 `(&mut LuaState, &mut LuaState)` 引用形，无法再由 `functions/shells.rs`
//! 中透传裸指针的共用宏 `capi_shell!` 直呼（宏体语义不得改，同族其余透传壳零行为变化），
//! 故本壳从宏模板退役、写显式 `extern "C-unwind"` 一行调用，与 `functions/lua_status.rs`
//! 先例统形：唯一差异是在本帧把两侧裸指针重建为独占引用后转调。
use core::ffi::c_int;

use ulua_vm::{functions::lua_xmove, records::lua_state::LuaState};

/// # Safety
/// C ABI 导出壳（符号 `ulua_lua_xmove`），除把 `from`/`to` 在本帧重建为独占引用外，
/// 仅透传至 `ulua_vm::functions::lua_xmove::lua_xmove(&mut *from, &mut *to, n)`，零业务
/// 逻辑。调用方须保证：
/// - `from`、`to`：同 `global_State` 下两棵存活 `LuaState`，非空、对齐，整个调用期间
///   存活，且与各自状态的其它访问单线程驱动（不得跨 OS 线程并发）——`&mut *` 引用
///   重建前提；`from == to`（同线程退化调用）由本壳首行指针相等短路，引用构造窗口
///   根本不成立（noalias 于实参构造即生效，不靠被调方体内兜底；cpp `lua_xmove` 体内
///   短路无前置引用构造，与本壳前移同判：退化调用净行为=不动作）；
/// - 其余安全前置条件与被调函数的文档契约一致。
#[unsafe(export_name = "ulua_lua_xmove")]
pub unsafe extern "C-unwind" fn lua_xmove(from: *mut LuaState, to: *mut LuaState, n: c_int) {
  // 同态守卫前移至引用构造之前：两枚重叠 `&mut` 若在实参求值期成立，进入被调方前
  // 即已违 noalias，体内 `from == to` 短路救不回调用入口 UB。
  if from == to {
    return;
  }
  // Safety: 契约声明两侧为同 VM 存活 `LuaState`，本帧 `&mut *` 重建即时结束借用窗口、
  // 不跨调用持有；搬运只界内读写（`lua_xmove` 体内 `api_check`/`api_checknelems` 兜底）。
  lua_xmove::lua_xmove(unsafe { &mut *from }, unsafe { &mut *to }, n)
}
