//! 本文件对应 `ulua_lua_setthreaddata` 导出符号（源：ulua-vm/src/functions/lua_setthreaddata.rs）。
//! vm 侧已 B 档前移为 `&mut LuaState` 接收者，无法再由 `functions/shells.rs` 中透传裸指针的
//! 共用宏 `capi_shell!` 直呼（宏体语义不得改，72+ 同形壳零行为变化），故本壳从宏模板退役、
//! 写显式 `extern "C-unwind"` 一行调用，与 `lua_l_checkudata.rs` 等表示适配显式壳统形：
//! 唯一差异是在本帧把 `l` 重建为独占可变引用后转调。
use core::ffi::c_void;

use ulua_vm::{functions::lua_setthreaddata, records::lua_state::LuaState};

/// # Safety
/// C ABI 导出壳（符号 `ulua_lua_setthreaddata`），除把 `l` 在本帧重建为独占可变引用外，
/// 仅按声明顺序逐参数透传至
/// `ulua_vm::functions::lua_setthreaddata::lua_setthreaddata(&mut *l, data)`，零业务逻辑。
/// 调用方须保证：
/// - `l`：指向由本 VM 创建的合法 `LuaState`，非空、对齐，整个调用期间存活，且该期间对此
///   状态无任何其它存活引用或指针并发访问（本帧将整实例写权限经 `&mut` 交予被调方做
///   单字段写入）——`&mut *l` 引用重建前提；
/// - `data`（`*mut c_void`）：C 侧不透明数据指针（userdata/缓冲/ud），可为 null；非 null 时
///   对齐且调用期间存活；被调方只原样存入线程数据槽、不解引用；
/// - 其余安全前置条件与被调函数的 `# Safety` 契约一致。
#[unsafe(export_name = "ulua_lua_setthreaddata")]
pub unsafe extern "C-unwind" fn lua_setthreaddata(l: *mut LuaState, data: *mut c_void) {
  // Safety: 契约声明 `l` 为本次调用期内独占驱动的合法 `LuaState`，`&mut *l` 重建满足排他
  // 借用前提；`data` 只转手存储不解引用；被调方为 safe fn，仅写 `userdata` 一个字段。
  unsafe { lua_setthreaddata::lua_setthreaddata(&mut *l, data) }
}
