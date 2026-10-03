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
/// r16-v24 收形：首参转 `&mut LuaState`——`l` 的存活与独占由类型承载。体内
/// `l.ci`/`l.base_ci` 的 ci 链裸读与 `(*ci).flags` 帧标志读点为 r13-w1c
/// 逐点定性保留面（ci 链无门面，r16-v22 lua_r_constructobject 同款保留判例），
/// 位点与时序原样保留；其中 `(*l).ci`/`(*l).base_ci` 接收者形经 `&mut` 隐式 deref
/// 等价改写为 `l.ci`/`l.base_ci`（同址同宽，避 `clippy::explicit_auto_deref`）。
/// 故 `unsafe fn` 屏障与本帧 `unsafe { … }` 块界均不变，未新增也未收窄任何授权窗。
///
/// # Safety
/// `l` 的存活与独占由 `&mut LuaState` 承载；返回的 `ci`/`var` 指针仅在 `l` 未发生栈重分配时有效。
pub(crate) unsafe fn resolve_local(
  l: &mut LuaState,
  level: i32,
  n: i32,
) -> Option<(*mut CallInfo, *const LocVar)> {
  unsafe {
    if (level as u32) >= (l.ci.offset_from(l.base_ci) as u32) {
      return None;
    }

    let ci: *mut CallInfo = l.ci.offset(-(level as isize));

    if ((*ci).flags & LUA_CALLINFO_NATIVE as u32) != 0 {
      return None;
    }

    let var: *const LocVar = lua_f_getlocal(get_lua_proto(ci).as_ref(), n, currentpc(&*ci));

    Some((ci, var))
  }
}

/// r16-v24 收形：首参转 `&mut LuaState`——`l` 的存活与独占由类型承载。体内保留点：
/// `resolve_local`/`lua_a_pushvalue` 转调各以一次 `&mut *l` 就地重建引用（借用窗仅在
/// 当句内，仓内既有判例形制）；`lua_c_threadbarrier_lapi` 仍收裸形，经 `l.as_mut_ptr()`
/// 转手、不跨调用持有重建出的引用；`(*ci).base.offset(…)` 帧窗基址裸读与
/// `getstr((*var).varname)` 局部变量名裸读为 r13-w1c 逐点定性保留面，原形原位保留。
/// 以上真实裸指针操作的前提由调用方给出，故 `unsafe fn` 屏障保留、不降为安全 `fn`
/// （判例同 r16-v21 lua_touserdatatagged_ref）。
///
/// # Safety
/// `l` 的存活与独占由 `&mut LuaState` 承载；`level` 须落在 `[0, ci-base_ci)` 内以选出有效调用帧，`n` 为该帧
/// `Proto` 的局部变量序号；命中时 `(*ci).base + (*var).reg` 须指向仍存活的栈槽（`lua_a_pushvalue` 读值），
/// 且 `(*var).varname` 为存活 `tstring`；NATIVE 帧直接返回 NULL。可分配/屏障，须受保护帧。cpp `ldebug.cpp:76`。
pub unsafe fn lua_getlocal(l: &mut LuaState, level: i32, n: i32) -> *const c_char {
  unsafe {
    let Some((ci, var)) = resolve_local(&mut *l, level, n) else {
      return null();
    };

    if !var.is_null() {
      lua_c_threadbarrier_lapi(l.as_mut_ptr());
      lua_a_pushvalue(&mut *l, &*(*ci).base.offset((*var).reg as isize));

      getstr((*var).varname)
    } else {
      null()
    }
  }
}
