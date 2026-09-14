//! Node: `cxx:Function:Luau.VM:VM/src/ldo.cpp:729:lua_d_pcall`
//! Source: `VM/src/ldo.cpp` (ldo.cpp:729-795, hand-ported)

use core::ffi::c_void;

use ulua_common::{FFlag, macros::luau_assert::LUAU_ASSERT};

use crate::{
  enums::lua_status::LuaStatus,
  functions::{
    callerrfunc::callerrfunc, lua_d_rawrunprotected_ldo_alt_b::lua_d_rawrunprotected_mut,
    lua_d_seterrorobj::luaD_seterrorobj, lua_f_close::lua_f_close as luaF_close,
    restore_stack_limit::restore_stack_limit,
  },
  macros::{clvalue::clvalue, restoreci::restoreci, restorestack::restorestack, saveci::saveci},
  records::{call_info::CallInfo, closure::Closure},
  type_aliases::{lua_state::lua_State, pfunc::Pfunc, stk_id::StkId},
};

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
pub unsafe fn lua_d_pcall(
  l: *mut lua_State,
  func: Pfunc,
  u: *mut c_void,
  old_top: isize,
  ef: isize,
) -> i32 {
  unsafe {
    let old_n_ccalls: u16 = (*l).n_ccalls;
    let old_base_ccalls: u16 = (*l).base_ccalls;
    let old_ci: isize = saveci!(l, (*l).ci);
    let oldactive: bool = (*l).isactive;
    let mut status: i32 = lua_d_rawrunprotected_mut(l, func, u);
    if status != 0 {
      let mut errstatus: i32 = status;

      if FFlag::LuauClosureUsageCounter.get() {
        let mut lastci: *mut CallInfo = (*l).ci;
        let savedci: *mut CallInfo = restoreci!(l, old_ci);
        while lastci != savedci {
          let cl = clvalue!((*lastci).func) as *const _ as *mut Closure;
          LUAU_ASSERT!((*cl).usage > 0);
          (*cl).usage -= 1;
          lastci = lastci.offset(-1);
        }
      }

      // call user-defined error function (used in xpcall)
      if ef != 0 {
        // push error object to stack top if it's not already there
        if status != LuaStatus::ErrRun as i32 {
          luaD_seterrorobj(l, status, (*l).top);
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
      luaF_close(l, oldtop); // close eventual pending closures
      luaD_seterrorobj(l, errstatus, oldtop);
      (*l).ci = restoreci!(l, old_ci);
      (*l).base = (*(*l).ci).base;
      restore_stack_limit(l);
    }
    status
  }
}

pub use lua_d_pcall as luaD_pcall;
