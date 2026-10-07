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
/// 缩栈前提由调用方给出：`l` 的存活与独占由 `&mut LuaState` 承载，但本函数会就地 realloc
/// 栈与 CallInfo 两数组并改写其全部指针字段，故调用序前提仍在本体之外——`base_ci..=ci` 帧
/// 数组与 `stack..stack_last` 栈窗须为该线程当前有效分配，且入口顶槽读数位点在任何 realloc
/// 先行之前（本体内已按此序摆放）。cpp lgc.cpp:485 `shrinkstack`
///
/// r16-v25 收形：首参转 `&mut LuaState`——`l` 的存活与独占由类型承载，原「`l` 非空」散文
/// 名实随形改调用序/分配前提（判例：commit `9b448ba4`）。体内栈/CallInfo 数组重分配、ci 链
/// 遍历与 `offset_from` 算术系真实裸指针操作且前提由调用方给出，保留 `unsafe fn` 屏障不降
/// 安全 `fn`（同型判例：r16-v21 `lua_touserdatatagged_ref`、r16-v22 `lua_r_setupconstructor`）。
/// 对仍收裸形的 `lua_d_realloc_ci`/`lua_d_reallocstack` 转调各以一次 `l.as_mut_ptr()` 就地
/// 重建（借用窗止于当句，未跨调用持有）；`(*l).field` 裸读点经 `&mut` 隐式 deref 等价改写为
/// `l.field`（同址同宽、时序不变，规避 `clippy::explicit_auto_deref`），体内 r13-w1b 逐点
/// 定性保留面位点原样不动。
pub(crate) unsafe fn shrinkstack(l: &mut LuaState) {
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
    let mut lim: StkId = l.top_slot(0);
    let mut ci: *mut CallInfo = l.base_ci;
    while ci <= l.ci {
      LUAU_ASSERT!((*ci).top <= l.stack_last);
      if lim < (*ci).top {
        lim = (*ci).top;
      }
      ci = ci.add(1);
    }

    // shrink stack and callinfo arrays if we aren't using most of the space
    let ci_used = l.ci.offset_from(l.base_ci) as i32;
    let s_used = lim.offset_from(l.stack) as i32;
    if l.size_ci > LUAI_MAXCALLS {
      // handling overflow?
      return;
    }

    if 3 * (ci_used as usize) < l.size_ci as usize && 2 * BASIC_CI_SIZE < l.size_ci {
      lua_d_realloc_ci(l.as_mut_ptr(), l.size_ci / 2); // still big enough...
    }

    condhardstacktests!(lua_d_realloc_ci(l.as_mut_ptr(), ci_used + 1));

    if 3 * (s_used as usize) < l.stacksize as usize
      && 2 * (BASIC_STACK_SIZE + EXTRA_STACK) < l.stacksize
    {
      lua_d_reallocstack(l.as_mut_ptr(), l.stacksize / 2, 0); // still big enough...
    }

    condhardstacktests!(lua_d_reallocstack(l.as_mut_ptr(), s_used, 0));
  }
}
