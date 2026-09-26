use core::ptr::addr_of;

use crate::{
  functions::{currentpc::currentpc, lua_g_getline::lua_g_getline},
  macros::ci_func::ci_func,
  records::{call_info::CallInfo, closure::LClosure, lua_state::LuaState},
};

/// # Safety
/// `ci` 须为存活 `CallInfo` 且为 Lua 帧（`ci_func!(ci)` 返回 LuaClosure）：其 `inner.l` 内嵌的 `LClosure.p`
/// 须指向存活 `Proto`，`(*proto).lineinfo` 覆盖 `currentpc(_l,ci)` 反查所得 PC；`_l` 供 `currentpc` 读该帧
/// 指令游标，须存活。纯只读，不分配、不抛错。
/// cpp VM/src/ldebug.cpp:28
pub(crate) unsafe fn currentline(_l: *mut LuaState, ci: *mut CallInfo) -> i32 {
  unsafe {
    let cl = ci_func!(ci);
    let lcl = addr_of!((*cl).inner.l).cast::<LClosure>();
    lua_g_getline((*lcl).p, currentpc(_l, ci))
  }
}
