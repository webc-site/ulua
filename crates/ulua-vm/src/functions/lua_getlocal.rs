use core::{ffi::c_char, ptr::null};

use crate::{
  functions::{
    currentpc::currentpc, getluaproto::get_lua_proto, lapi_barrier::lua_c_threadbarrier_lapi,
    lua_a_pushvalue::lua_a_pushvalue, lua_f_getlocal::lua_f_getlocal,
  },
  macros::{getstr::getstr, lua_callinfo_native::LUA_CALLINFO_NATIVE},
  records::{call_info::CallInfo, loc_var::LocVar, lua_state::LuaState},
};

/// getlocal/setlocal 共用序言：`level` 越界或 NATIVE 帧返回 `None`（外部改写
/// 原生帧的表/寄存器会破坏表状态/寄存器 tag 的安全契约）；命中返回
/// `(调用帧, n 号局部变量记录)`，`var` 可为 null，由调用方按各自语义收尾。
///
/// # Safety
/// `l` 须为存活 `LuaState`；返回的 `ci`/`var` 指针仅在 `l` 未发生栈重分配时有效。
pub(crate) unsafe fn resolve_local(
  l: *mut LuaState,
  level: i32,
  n: i32,
) -> Option<(*mut CallInfo, *const LocVar)> {
  unsafe {
    if (level as u32) >= ((*l).ci.offset_from((*l).base_ci) as u32) {
      return None;
    }

    let ci: *mut CallInfo = (*l).ci.offset(-(level as isize));

    if ((*ci).flags & LUA_CALLINFO_NATIVE as u32) != 0 {
      return None;
    }

    let var: *const LocVar = lua_f_getlocal(get_lua_proto(ci).as_ref(), n, currentpc(l, ci));

    Some((ci, var))
  }
}

/// # Safety
/// `l` 须为存活 `LuaState`；`level` 须落在 `[0, ci-base_ci)` 内以选出有效调用帧，`n` 为该帧
/// `Proto` 的局部变量序号；命中时 `(*ci).base + (*var).reg` 须指向仍存活的栈槽（`lua_a_pushvalue` 读值），
/// 且 `(*var).varname` 为存活 `tstring`；NATIVE 帧直接返回 NULL。可分配/屏障，须受保护帧。cpp `ldebug.cpp:76`。
pub unsafe fn lua_getlocal(l: *mut LuaState, level: i32, n: i32) -> *const c_char {
  unsafe {
    let Some((ci, var)) = resolve_local(l, level, n) else {
      return null();
    };

    if !var.is_null() {
      lua_c_threadbarrier_lapi(l);
      lua_a_pushvalue(l, (*ci).base.offset((*var).reg as isize));

      getstr((*var).varname)
    } else {
      null()
    }
  }
}
