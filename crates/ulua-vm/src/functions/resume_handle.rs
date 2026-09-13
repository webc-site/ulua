use core::ffi::c_void;

use ulua_common::{FFlag, macros::luau_assert::LUAU_ASSERT};

use crate::{
  enums::lua_status::LuaStatus,
  functions::{
    callerrfunc::callerrfunc, lua_d_rawrunprotected_ldo::luaD_rawrunprotected,
    lua_d_seterrorobj::luaD_seterrorobj, lua_f_close::luaF_close, luau_poscall::luau_poscall,
    restore_stack_limit::restore_stack_limit, resume_continue::resume_continue,
  },
  macros::{
    ci_func::ci_func, lua_callinfo_handle::LUA_CALLINFO_HANDLE, restoreci::restoreci,
    saveci::saveci,
  },
  records::{call_info::CallInfo, closure::CClosure},
  type_aliases::lua_state::lua_State,
};

pub(crate) unsafe extern "C-unwind" fn resume_handle(l: *mut lua_State, ud: *mut c_void) {
  unsafe {
    let mut ci = ud as *mut CallInfo;
    let cl = ci_func!(ci);

    LUAU_ASSERT!(((*ci).flags & LUA_CALLINFO_HANDLE as u32) != 0);
    let c = core::ptr::addr_of!((*cl).inner.c).cast::<CClosure>();
    LUAU_ASSERT!((*cl).is_c != 0 && (*c).cont.is_some());
    LUAU_ASSERT!((*l).status != 0);

    // make sure we don't run the handler the second time
    (*ci).flags &= !(LUA_CALLINFO_HANDLE as u32);

    // restore thread status to LUA_OK since we're handling the error
    let mut status = (*l).status as i32;
    (*l).status = LuaStatus::Ok as u8;

    // push error object to stack top if it's not already there
    if status != LuaStatus::ErrRun as i32 {
      luaD_seterrorobj(l, status, (*l).top);
    }

    // call user-defined error function
    if (*ci).errfunc != 0 {
      // save ci pointer - it will be invalidated by callerrfunc call
      let old_ci = saveci!(l, ci);

      // if errfunc fails, we fail with "error in error handling" or "not enough memory"
      let err = luaD_rawrunprotected(
        l,
        Some(callerrfunc),
        (*ci).base.offset((*ci).errfunc as isize - 1) as *mut c_void,
      );

      if !FFlag::LuauXpcallFixMessageYieldPath.get() {
        // restore nCcalls to base if errfunc itself errored
        (*l).n_ccalls = (*l).base_ccalls;
      }

      // in general we preserve the status, except for cases when the error handler fails
      // out of memory is treated specially because it's common for it to be cascading, in which case we preserve the code
      if err == 0 {
        status = LuaStatus::ErrRun as i32;
      } else if status == LuaStatus::ErrMem as i32 && err == LuaStatus::ErrMem as i32 {
        status = LuaStatus::ErrMem as i32;
      } else {
        status = LuaStatus::ErrErr as i32;
      }

      luaD_seterrorobj(l, status, (*l).top.offset(-1));

      ci = restoreci!(l, old_ci);
      (*ci).errfunc = 0;
    }

    if FFlag::LuauXpcallFixMessageYieldPath.get() {
      // restore nCcalls to base for the continuation
      (*l).n_ccalls = (*l).base_ccalls;
    }

    // restore the stack frame to the frame with continuation
    (*l).ci = ci;

    // close eventual pending closures; this means it's now safe to restore stack
    luaF_close(l, (*(*l).ci).base);

    // adjust the stack frame for ci to prepare for cont call
    (*l).base = (*ci).base;
    (*ci).top = (*l).top;

    restore_stack_limit(l);

    let n = (*c).cont.unwrap()(l, status);

    if (*l).status != LuaStatus::Ok as u8 {
      return;
    }

    // finish cont call and restore stack to previous ci top
    luau_poscall(l, (*l).top.offset(-(n as isize)));

    // run remaining continuations from the stack; typically resumes pcalls
    resume_continue(l);
  }
}
