use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::{
  functions::{lua_d_realloc_ci::lua_d_realloc_ci, lua_d_reallocstack::lua_d_reallocstack},
  macros::{
    basic_ci_size::BASIC_CI_SIZE, basic_stack_size::BASIC_STACK_SIZE,
    condhardstacktests::condhardstacktests, extra_stack::EXTRA_STACK, luai_maxcalls::LUAI_MAXCALLS,
  },
  records::{call_info::CallInfo, lua_state::LuaState},
  type_aliases::stk_id::StkId,
};

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
pub(crate) unsafe fn shrinkstack(l: *mut LuaState) {
  // r13-w1b 逐点定性（w6d 口径保留面）：base_ci/ci 帧数组遍历、`ci - base_ci` 与
  // `lim - stack` 的 array→array offset_from 算术、stack_last 界缘判读、size_ci/
  // stacksize 容量读写均无 LuaState 门面（边界原语只覆盖 `top` 槽算术，CallInfo
  // 数组面与容量字段系 records/slot.rs 红线原样保留）；`(*ci).top` 为 CallInfo
  // 裸字段。收编仅入口顶槽读数一处（top_slot(0)，位点在任何 realloc 先行之前，
  // 未新增预绑定窗）。
  unsafe {
    // compute used stack - note that we can't use th->top if we're in the middle of vararg call
    // 收编：入口顶槽读数经 top_slot(0) 边界原语（镜像 cpp `StkId lim = L->top;`
    // 单次绑定形，读数位点不变）
    let mut lim: StkId = (*l).top_slot(0);
    let mut ci: *mut CallInfo = (*l).base_ci;
    while ci <= (*l).ci {
      LUAU_ASSERT!((*ci).top <= (*l).stack_last);
      if lim < (*ci).top {
        lim = (*ci).top;
      }
      ci = ci.add(1);
    }

    // shrink stack and callinfo arrays if we aren't using most of the space
    let ci_used = (*l).ci.offset_from((*l).base_ci) as i32;
    let s_used = lim.offset_from((*l).stack) as i32;
    if (*l).size_ci > LUAI_MAXCALLS {
      // handling overflow?
      return;
    }

    if 3 * (ci_used as usize) < (*l).size_ci as usize && 2 * BASIC_CI_SIZE < (*l).size_ci {
      lua_d_realloc_ci(l, (*l).size_ci / 2); // still big enough...
    }

    condhardstacktests!(lua_d_realloc_ci(l, ci_used + 1));

    if 3 * (s_used as usize) < (*l).stacksize as usize
      && 2 * (BASIC_STACK_SIZE + EXTRA_STACK) < (*l).stacksize
    {
      lua_d_reallocstack(l, (*l).stacksize / 2, 0); // still big enough...
    }

    condhardstacktests!(lua_d_reallocstack(l, s_used, 0));
  }
}
