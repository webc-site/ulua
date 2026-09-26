//! Source: `VM/src/lvmexecute.cpp:3843-3872` (hand-ported)

use core::slice::from_raw_parts_mut;

use crate::{
  macros::{lua_multret::LUA_MULTRET, setnilvalue::setnilvalue, setobj_2_s::setobj_2_s},
  records::lua_state::LuaState,
  type_aliases::{stk_id::StkId, t_value::TValue},
};

/// # Safety
/// 调用方须保证：`(*l).ci` 是正在返回的 Lua 帧且父帧 cip 存在（CallInfo 尚未 pop）；
/// `first..(*l).top` 为同属存活栈的连续窗口且 first<=top；目标 `(*ci).func` 起的写入窗口
/// 已由调用方（OP_CALL/performcall）预留，nresults 与预留结果槽数一致（负值为 LUA_MULTRET）。
/// cpp lvmexecute.cpp:3956 `luau_poscall`
/// C++ `void luau_poscall(LuaState* l, StkId first)`.
pub(crate) unsafe fn luau_poscall(l: *mut LuaState, first: StkId) {
  // Safety: 契约保证 `l->ci` 为正在返回的存活帧，父帧 cip 有效；块内将结果搬回 func..func+nresults 不越过父帧栈区
  unsafe {
    // finish interrupted execution of `OP_CALL'
    // ci is our callinfo, cip is our parent
    let ci = (*l).ci;
    let cip = ci.sub(1);

    // copy return values into parent stack (but only up to nresults!), fill
    // the rest with nil
    // TODO: it might be worthwhile to handle the case when nresults==b explicitly?
    let res = (*ci).func;
    let valend = (*l).top;
    let nresults = (*ci).nresults;

    // 源窗口元素数：契约保证 first <= top 且同属存活栈，一次 offset_from 算清，
    // 取代原「指针逐格比较」的手写游走
    let avail = valend.offset_from(first).max(0) as usize;
    // nresults<0（MULTRET）时全量拷贝（C++ `i != 0` 恒真），否则按 nresults 截断
    let ncopy = if nresults < 0 {
      avail
    } else {
      avail.min(nresults as usize)
    };
    // 保留索引遍历：j 同时是 dst 与 src 两个窗口的偏移，且窗口可重叠（Lua 帧结果从
    // func+1 起，src = dst + Δ），借成 &mut/[TValue] 切片会构成别名冲突，只能按正序
    // 逐格「先读后写」走指针（cpp lvmexecute.cpp:3970）
    for j in 0..ncopy {
      setobj_2_s!(l, res.add(j), first.add(j) as *const TValue);
    }
    let mut res = res.add(ncopy);
    // 补 nil 数：仅 nresults>0 且源不足时补差额（等价原 `while i > 0` 尾循环）
    let nfill = if nresults > 0 {
      (nresults as usize) - ncopy
    } else {
      0
    };
    // Safety: res 起的 nfill 格属父帧预留的结果槽，落在 (*ci).func..父帧 top 内
    for slot in from_raw_parts_mut(res, nfill) {
      setnilvalue!(slot);
    }
    res = res.add(nfill);

    // pop the stack frame
    (*l).ci = cip;
    (*l).base = (*cip).base;
    (*l).top = if nresults == LUA_MULTRET {
      res
    } else {
      (*cip).top
    };
  }
}
