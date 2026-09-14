use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::{
  functions::{lua_d_realloc_ci::lua_d_realloc_ci, lua_d_reallocstack::lua_d_reallocstack},
  macros::{
    basic_ci_size::BASIC_CI_SIZE, basic_stack_size::BASIC_STACK_SIZE, cast_int::cast_int,
    condhardstacktests::condhardstacktests, extra_stack::EXTRA_STACK, luai_maxcalls::LUAI_MAXCALLS,
  },
  records::call_info::CallInfo,
  type_aliases::{lua_state::lua_State, stk_id::StkId},
};

pub(crate) unsafe fn shrinkstack(l: *mut lua_State) {
  unsafe {
    // compute used stack - note that we can't use th->top if we're in the middle of vararg call
    let mut lim: StkId = (*l).top;
    let mut ci: *mut CallInfo = (*l).base_ci;
    while ci <= (*l).ci {
      LUAU_ASSERT!((*ci).top <= (*l).stack_last);
      if lim < (*ci).top {
        lim = (*ci).top;
      }
      ci = ci.add(1);
    }

    // shrink stack and callinfo arrays if we aren't using most of the space
    let ci_used = cast_int!((*l).ci.offset_from((*l).base_ci));
    let s_used = cast_int!(lim.offset_from((*l).stack));
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
