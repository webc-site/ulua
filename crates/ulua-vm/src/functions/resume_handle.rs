use core::{ffi::c_void, ptr::addr_of};

use ulua_common::{fflag, macros::luau_assert::LUAU_ASSERT};

use crate::{
  enums::lua_status::LuaStatus,
  functions::{
    callerrfunc::callerrfunc, lua_d_rawrunprotected_ldo::lua_d_rawrunprotected,
    lua_d_seterrorobj::lua_d_seterrorobj, lua_f_close::lua_f_close, luau_poscall::luau_poscall,
    restore_stack_limit::restore_stack_limit, resume_continue::resume_continue,
  },
  macros::{
    ci_func::ci_func, lua_callinfo_handle::LUA_CALLINFO_HANDLE, restoreci::restoreci,
    saveci::saveci,
  },
  records::{call_info::CallInfo, closure::CClosure, lua_state::LuaState},
};

/// 协程恢复出错后驱动 message handler 帧的细粒度恢复回调（cpp `resume_handle`）。
///
/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
/// `l` 须为存活协程状态；`ud` 须为细粒度恢复入参：`resume_findhandler` 取回、
/// 置 `LUA_CALLINFO_HANDLE` 的存活 `CallInfo` handler 帧（非空，调用方
/// `resume_finish`/`lua_resumeerror` 均已判空后透传）。
/// 与粗粒度恢复 `resume` 的差异及理由：本回调仅推进出错 handler 帧及其续体
/// `cont`，`ud` 是 handler 帧指针；`resume` 为首次进入重放整个协程体，`ud` 是
/// 首实参栈槽。恢复粒度不同，两者 `ud` 不可互换。
pub(crate) unsafe extern "C-unwind" fn resume_handle(l: *mut LuaState, ud: *mut c_void) {
  unsafe {
    let mut ci = ud as *mut CallInfo;
    let cl = ci_func!(ci);

    LUAU_ASSERT!(((*ci).flags & LUA_CALLINFO_HANDLE as u32) != 0);
    let c = addr_of!((*cl).inner.c).cast::<CClosure>();
    LUAU_ASSERT!((*cl).is_c != 0 && (*c).cont.is_some());
    LUAU_ASSERT!((*l).status != 0);

    // make sure we don't run the handler the second time
    (*ci).flags &= !(LUA_CALLINFO_HANDLE as u32);

    // restore thread status to LUA_OK since we're handling the error
    let mut status = (*l).status as i32;
    (*l).status = LuaStatus::Ok as u8;

    // push error object to stack top if it's not already there
    if status != LuaStatus::ErrRun as i32 {
      lua_d_seterrorobj(l, status, (*l).top);
    }

    // call user-defined error function
    if (*ci).errfunc != 0 {
      // save ci pointer - it will be invalidated by callerrfunc call
      let old_ci = saveci!(l, ci);

      // if errfunc fails, we fail with "error in error handling" or "not enough memory"
      // Safety: `errfunc` 为本帧错误处理实参在栈上的槽偏移（lua_d_pcall 协议登记），
      // `base + errfunc - 1` 落在当前协程界内活栈槽（cpp 同点位直译）
      let err = lua_d_rawrunprotected(
        l,
        Some(callerrfunc),
        (*ci).base.offset((*ci).errfunc as isize - 1) as *mut c_void,
      );

      // DELIBERATE DEVIATION（同 resume_finish 锚点）：本地 cpp（ldo.cpp:652）在
      // errfunc 块后无条件还原 `nCcalls = baseCcalls`；更新上游按旗收进块内。
      // `errfunc == 0` 时该点位前无 luaD_call（nCcalls==baseCcalls 恒成立），
      // 还原为 no-op——旗关路径与本地 oracle 可观察行为恒等。
      if !fflag::LuauXpcallFixMessageYieldPath.get() {
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

      lua_d_seterrorobj(l, status, (*l).top.offset(-1));

      ci = restoreci!(l, old_ci);
      (*ci).errfunc = 0;
    }

    if fflag::LuauXpcallFixMessageYieldPath.get() {
      // restore nCcalls to base for the continuation
      (*l).n_ccalls = (*l).base_ccalls;
    }

    // restore the stack frame to the frame with continuation
    (*l).ci = ci;

    // close eventual pending closures; this means it's now safe to restore stack
    lua_f_close(l, (*(*l).ci).base);

    // adjust the stack frame for ci to prepare for cont call
    (*l).base = (*ci).base;
    (*ci).top = (*l).top;

    restore_stack_limit(l);

    // cont 非空由入口 LUAU_ASSERT!((*c).cont.is_some()) 与 coroutine resume 协议保证（cpp: 同款直接调用）
    let cont = (*c)
      .cont
      .expect("resume 处理帧的 cont 由协程 resume 协议保证非空");
    let n = cont(l, status);

    if (*l).status != LuaStatus::Ok as u8 {
      return;
    }

    // finish cont call and restore stack to previous ci top
    luau_poscall(l, (*l).top.offset(-(n as isize)));

    // run remaining continuations from the stack; typically resumes pcalls
    resume_continue(l);
  }
}
