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
  // r14-p2 逐点定性（w6d 口径保留面）：status 三处消费形点位——`!=0` 裸谓词（入口断言）、
  // `as i32` 原值消费（存值读回透传）、`!=Ok` 否定谓词（cont 恢复点后）——status() 门面
  // 的 non-repr→Ok 兜底把 0x7f（SCHEDULED_REENTRY）等越界值折成 Ok，前两者改写原谓词/
  // 原值真值、第三者把判真折成判假，均不逐位等价（r13-w1b 真值表反证红线），保留 C 形；
  // status/base 落笔与 n_ccalls/base_ccalls 读写（LuaState 无 status 写面、无 ccount 门面）、
  // ci 场域回写与 close/base 帧面读（含落笔行 RHS 同形单点，r12 既有判例）均属 CallInfo/
  // 场域建立面无既有门面，B 红线不翻案，全数原样保留。
  // 收编共三处读数：两处 seterrorobj 顶槽/顶下槽地址入参经 top_slot(0)/top_slot(-1)
  // 边界原语（resume_finish:89 先例，同址同现读位点），cont 恢复点后 poscall 参数槽经
  // top_slot(-(n as isize))——位点保持现读，禁预绑定（见行内注）。
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
      // 收编：顶槽地址读数经 top_slot(0) 边界原语（镜像 cpp `L->top` 读数形，
      // off=0 即保留顶槽；resume_finish 错误收尾同款先例，入参求值位现读不变）
      lua_d_seterrorobj(l, status, (*l).top_slot(0));
    }

    // call user-defined error function
    if (*ci).errfunc != 0 {
      // save ci pointer - it will be invalidated by callerrfunc call
      let old_ci = saveci!(l, ci);

      // if errfunc fails, we fail with "error in error handling" or "not enough memory"
      // SAFETY: `errfunc` 为本帧错误处理实参在栈上的槽偏移（lua_d_pcall 协议登记），
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

      // 收编：顶下槽地址读数经 top_slot(-1) 边界原语（同址现读；errfunc 收口后
      // 顶槽即新错误对象落位，槽距 -1 与原 `offset(-1)` 逐指令等价）
      lua_d_seterrorobj(l, status, (*l).top_slot(-1));

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
    // 收编：poscall 结果窗基址读数经 top_slot(-(n as isize)) 边界原语（同址现读）。
    // 红线：cont()/handler 再入可搬栈，读数必须保持本恢复点现读位点——原语内联
    // 即场域读，禁把读数提出到 cont 调用之前预绑定（r12/r13 恢复点禁预绑定纪律）
    luau_poscall(l, (*l).top_slot(-(n as isize)));

    // run remaining continuations from the stack; typically resumes pcalls
    resume_continue(l);
  }
}
