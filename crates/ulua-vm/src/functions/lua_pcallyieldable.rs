use core::{
  ffi::c_void,
  ptr::{addr_of, addr_of_mut},
};

use crate::{
  functions::{
    lua_d_callint::lua_d_callint, lua_d_pcall::lua_d_pcall, lua_isyieldable::lua_isyieldable,
  },
  macros::{
    api_check::api_check, c_call_yield::C_CALL_YIELD, expandstacklimit::expandstacklimit,
    iscfunction::iscfunction, isyielded::isyielded, lua_callinfo_handle::LUA_CALLINFO_HANDLE,
    savestack::savestack,
  },
  records::{closure::CClosure, lua_state::LuaState},
  type_aliases::stk_id::StkId,
};

struct CallContext {
  func: StkId,
  nresults: i32,
}

/// # Safety
/// 仅供 [`lua_pcallyieldable`] 经受保护回调机制调用：`ud` 必须指向其栈上存活的 `CallContext`
/// （`func/nresults` 由此读出），`l` 为该受保护帧本身（cpp lapi.cpp:1252）。
unsafe extern "C-unwind" fn pcallyieldable_run(l: *mut LuaState, ud: *mut c_void) {
  // SAFETY: 契约保证 ud 指向 CallContext（调用方栈上存活至 pcall 结束）
  unsafe {
    let ctx = &*(ud as *const CallContext);
    let preparereentry = lua_isyieldable(l) != 0;
    lua_d_callint(l, ctx.func, ctx.nresults, preparereentry);
  }
}

/// # Safety
/// 只能按 cpp lapi.cpp:1252 的 resume 协议调用：当前 `(*l).ci` 须为携带 `cont` 的 C 闭包帧，
/// 栈顶自 `base` 起已实参覆盖 `nargs + 1` 个槽且 `errfunc ∈ [0, top-base]`（api_check 兜底），
/// 否则块内 `top.sub(nargs+1)`/`base.add(errfunc-1)` 越界。
pub unsafe fn lua_pcallyieldable(l: *mut LuaState, nargs: i32, nresults: i32, errfunc: i32) -> i32 {
  // SAFETY: 契约保证 ci 帧为带 cont 的 C 闭包且栈内实参覆盖 nargs/errfunc，pcall 恢复链由 lua_d_pcall 维护
  unsafe {
    api_check!(l, iscfunction!((*(*l).ci).func));
    let cl = (*(*(*l).ci).func).as_closure_ptr();
    let c = addr_of!((*cl).inner.c).cast::<CClosure>();
    api_check!(l, (*c).cont.is_some());
    // r14-p1 收编：两处栈帧槽距读数落既有 get_top 门面（stack.rs 既有本体
    // `slot_distance(base, top)`）——即被替代式 `L->top - L->base` 裸槽距读数的
    // 同址同宽镜像（cpp lapi.cpp:1252 侧 api_check 读数形，位点现读不变、不上提
    // 不复用：两行各自现读，与原两行各自现读字段一致）。isize→i32 收敛无损：
    // 槽距受栈深上限约束、nargs/errfunc 受 LUAI_MAXCCALLS 与 api 契约约束，均在
    // i32 域内，谓词真值逐位恒等。
    api_check!(l, (nargs + 1) <= (*l).get_top());
    api_check!(l, errfunc >= 0 && errfunc <= (*l).get_top());

    (*(*l).ci).errfunc = errfunc;
    (*(*l).ci).flags |= LUA_CALLINFO_HANDLE as u32;

    let mut ctx = CallContext {
      func: (*l).top.sub((nargs + 1) as usize),
      nresults,
    };

    let savedfunc = savestack!(l, ctx.func);
    let savederrfunc = if errfunc != 0 {
      savestack!(l, (*l).base.add((errfunc - 1) as usize))
    } else {
      0
    };

    let status = lua_d_pcall(
      l,
      Some(pcallyieldable_run),
      addr_of_mut!(ctx).cast::<c_void>(),
      savedfunc,
      savederrfunc,
    );

    expandstacklimit!(l, (*l).top);

    if status == 0 && isyielded(&*l) {
      return C_CALL_YIELD;
    }

    (*(*l).ci).flags &= !(LUA_CALLINFO_HANDLE as u32);

    // cont 非空已由入口 api_check! 保证（对应 cpp: api_check(L, ci->cont) 后直接调用）
    let cont = (*c)
      .cont
      .expect("可 yield pcall 闭包的 cont 由 lua_pushcclosurek 协议保证非空");
    cont(l, status)
  }
}
