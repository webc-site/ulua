//! C 调用返回收尾公共样板（cpp lvmexecute.cpp `luaD_precall` C 路径与
//! `luau_poscall`/`LOP_CALL`/`LOP_RETURN` 同款拷回-补 nil-弹帧三连）的单源。
//!
//! 原先在 `luau_precall`、`luau_poscall`、`luau_execute` 各持一份同构实现；
//! 两层接口按 `l->top` 收口协议拆分：C 路径恒收在结果尾，Lua 路径 MULTRET 收
//! 结果尾、否则恢复父帧 top。

use core::slice::from_raw_parts_mut;

use crate::{
  macros::{lua_multret::LUA_MULTRET, setnilvalue::setnilvalue, setobj_2_s::setobj_2_s},
  records::lua_state::LuaState,
  type_aliases::stk_id::StkId,
};

/// 把 `[vali, valend)` 的值按 `nresults` 上限拷回 `(*ci).func`（不足补 nil），
/// 弹出当前帧（`ci`/`base`），返回补齐后的「结果尾」指针。不动 `(*l).top`——
/// 由调用方按各自协议收口（C 路径收结果尾，Lua 路径见 [`pop_frame_copy_results`]）。
///
/// # Safety（由调用侧 unsafe 上下文承担）
///
/// `l` 指向存活 `LuaState` 且其 `ci` 为当前 Lua/C 帧（CALL 布局保证 func..top 可写）；
/// `vali <= valend` 且两指针指向同一存活栈区间；`nresults` 为 -1/0/正数声明的结果数。
#[inline(always)]
pub(crate) unsafe fn copy_results_pop_frame(
  l: *mut LuaState,
  vali: StkId,
  valend: StkId,
  nresults: i32,
) -> StkId {
  // Safety: 契约保证 l/ci 有效、vali..valend 为存活栈区间，拷贝目标 (*ci).func 起同样在栈内可写
  unsafe {
    // ci is our callinfo, cip is our parent
    let ci = (*l).ci;
    let cip = ci.sub(1);

    // copy return values into parent stack (but only up to nresults!),
    // fill the rest with nil
    // note: in MULTRET context nresults starts as -1 so the copy limit is
    // the whole [vali, valend) window (C++ `i != 0` never fires)
    let res = (*ci).func;
    // 源窗口元素数：契约保证 vali <= valend 且同属存活栈，offset_from 一次算清，
    // 取代原「指针逐格比较」的手写游走
    let avail = valend.offset_from(vali).max(0) as usize;
    let ncopy = if nresults < 0 {
      avail
    } else {
      avail.min(nresults as usize)
    };
    // 保留索引遍历：j 同时是 dst 与 src 两个窗口的偏移，且窗口可重叠（RETURN 布局下
    // src = dst + Δ），借成 &mut/[TValue] 切片会构成别名冲突，只能按正序逐格先读后写
    for j in 0..ncopy {
      setobj_2_s!(l, res.add(j), vali.add(j));
    }
    let mut res = res.add(ncopy);
    // 补 nil 数：仅 nresults>0 且源不足时补差额（等价原 `while i > 0` 尾循环）
    let nfill = if nresults > 0 {
      (nresults as usize) - ncopy
    } else {
      0
    };
    // Safety: res 起的 nfill 格是本帧预留的结果槽，落在 (*ci).func..(*l).top 内
    for slot in from_raw_parts_mut(res, nfill) {
      setnilvalue!(slot);
    }
    res = res.add(nfill);

    // pop the stack frame
    (*l).ci = cip;
    (*l).base = (*cip).base;
    res
  }
}

/// Lua 帧返回收口（cpp `luau_poscall` 尾句）：[`copy_results_pop_frame`] 之上把
/// `l->top` 收口——MULTRET 时收在结果尾，否则恢复父帧 top。
///
/// # Safety（同 [`copy_results_pop_frame`] 契约）
#[inline(always)]
pub(crate) unsafe fn pop_frame_copy_results(
  l: *mut LuaState,
  vali: StkId,
  valend: StkId,
  nresults: i32,
) {
  // Safety: 契约保证 l/ci 有效、vali..valend 为存活栈区间，拷贝目标 (*ci).func 起同样在栈内可写
  unsafe {
    let res = copy_results_pop_frame(l, vali, valend, nresults);

    // 弹帧后 (*l).ci 即父帧 cip
    (*l).top = if nresults == LUA_MULTRET {
      res
    } else {
      (*(*l).ci).top
    };
  }
}
