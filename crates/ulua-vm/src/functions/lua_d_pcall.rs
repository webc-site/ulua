//! Source: `VM/src/ldo.cpp` (ldo.cpp:729-795, hand-ported)

use core::ffi::c_void;

use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::{
  enums::lua_status::LuaStatus,
  functions::{
    callerrfunc::callerrfunc, lua_d_rawrunprotected_ldo::lua_d_rawrunprotected_mut,
    lua_d_seterrorobj::lua_d_seterrorobj, lua_f_close::lua_f_close,
    restore_stack_limit::restore_stack_limit,
  },
  macros::{restoreci::restoreci, restorestack::restorestack, saveci::saveci},
  records::lua_state::LuaState,
  type_aliases::{pfunc::Pfunc, stk_id::StkId},
};

/// # Safety
/// `l` 须为可受保护调用的存活 `LuaState`（cpp ldo.cpp:874）：`old_top/ef` 必须是本状态上
/// `luaD_savestack` 产出的偏移（错误路径据此 `restorestack/restoreci` 回退帧与栈）；`func`
/// 为合法 `unsafe extern "C-unwind"` 受保护回调，`u` 是其按约定配套的透传数据指针。
pub unsafe fn lua_d_pcall(
  l: *mut LuaState,
  func: Pfunc,
  u: *mut c_void,
  old_top: isize,
  ef: isize,
) -> i32 {
  // Safety: 契约保证 `l` 为存活调用帧、func/ud 满足受保护回调约定；块内 n_ccalls/ci/isactive 的保存与恢复成对进行，错误路径经 restoreci 回退
  unsafe {
    let old_n_ccalls: u16 = (*l).n_ccalls;
    let old_base_ccalls: u16 = (*l).base_ccalls;
    let old_ci: isize = saveci!(l, (*l).ci);
    let oldactive: bool = (*l).isactive;
    let mut status: i32 = lua_d_rawrunprotected_mut(l, func, u);
    if status != 0 {
      let mut errstatus: i32 = status;

      // call user-defined error function (used in xpcall)
      if ef != 0 {
        // push error object to stack top if it's not already there
        if status != LuaStatus::ErrRun as i32 {
          lua_d_seterrorobj(l, status, (*l).top);
        }

        // if errfunc fails, we fail with "error in error handling" or "not enough memory"
        let err =
          lua_d_rawrunprotected_mut(l, Some(callerrfunc), restorestack!(l, ef) as *mut c_void);

        // in general we preserve the status, except for cases when the error handler fails
        // out of memory is treated specially because it's common for it to be cascading, in which case we preserve the code
        if err == 0 {
          errstatus = LuaStatus::ErrRun as i32;
        } else if status == LuaStatus::ErrMem as i32 && err == LuaStatus::ErrMem as i32 {
          errstatus = LuaStatus::ErrMem as i32;
        } else {
          errstatus = LuaStatus::ErrErr as i32;
          status = LuaStatus::ErrErr as i32;
          LUAU_ASSERT!(errstatus != 0);
        }
      }

      // since the call failed with an error, we might have to reset the 'active' thread state
      if !oldactive {
        (*l).isactive = false;
      }

      // Inlined logic from 'lua_isyieldable' to avoid potential for an out of line call.
      let yieldable: bool = (*l).n_ccalls <= (*l).base_ccalls;

      // restore n_ccalls and base_ccalls before calling the debugprotectederror callback which may rely on the proper value to have been restored.
      (*l).n_ccalls = old_n_ccalls;
      (*l).base_ccalls = old_base_ccalls;

      // an error occurred, check if we have a protected error callback
      if yieldable && let Some(debugprotectederror) = (*(*l).global).cb.debugprotectederror {
        debugprotectederror(l);

        // debug hook is only allowed to break
        if (*l).status as i32 == LuaStatus::Break as i32 {
          return 0;
        }
      }

      let oldtop: StkId = restorestack!(l, old_top);
      lua_f_close(l, oldtop); // close eventual pending closures
      lua_d_seterrorobj(l, errstatus, oldtop);
      (*l).ci = restoreci!(l, old_ci);
      (*l).base = (*(*l).ci).base;
      restore_stack_limit(l);
    }
    status
  }
}
