use crate::{
  enums::tms::TMS,
  functions::{c_slice_mut, lua_t_gettmbyobj::lua_t_gettmbyobj},
  macros::{lua_g_typeerror::luaG_typeerror, setobj_2_s::setobj_2_s},
  records::{lua_state::LuaState, slot::Slot},
  type_aliases::stk_id::StkId,
};

/// # Safety
/// `l` 须为存活调用帧且 `func` 指向其栈内实参槽句柄、自该槽到 `(*l).top` 连续可写
/// （cpp lvmutils.cpp:828）：块内经 `lua_t_gettmbyobj` 取 __call 元方法，随后把
/// `[func, top)` 整体后移一格再把 tm 写入 `func`，栈余量不足即越界写。句柄跨调用
/// 存续服从「扩容先行、借用后派生」不变量——本函数不重取栈基址，槽地址在调用期间
/// 不得迁移。
pub unsafe fn lua_v_tryfunc_tm(l: *mut LuaState, func: Slot<'_>) {
  // SAFETY: 契约保证 func 与 [func, top) 区间为当前帧可写栈槽、top 有空余，挪位与覆写均不越栈界
  unsafe {
    // 句柄落回裸槽视图一处：帧挪位窗（`[func, top]` 后移一格）是帧内栈区切片，
    // 起点/终点经 `Slot::as_ptr` 与既有 top 字段现读派生；`as_ptr`/`slot_distance`
    // 均为 `inline(always)` 指针读出，与原裸形参同址同宽度（§9.4）
    let func = func.as_ptr();

    let tm = lua_t_gettmbyobj(l, func, TMS::TmCall);
    if !(*tm).is_function() {
      luaG_typeerror!(l, func, "call");
    }

    // cpp `for (p = L->top; p > func; p--) setobj2s(L, p, p-1)`：把 `[func, top]`
    // （含预留 top 槽，界内契约见 # Safety）两端点整体后移一格，收为一次切片
    // `copy_within`（dst>src 的后向写序与 cpp 递减游标逐位一致）；全程不重分配栈，
    // 窗基址由 func 现读派生，尾槽 `top` 写为移位落点、随后抬顶提交。
    let win = c_slice_mut(func, LuaState::slot_distance(func, (*l).top) as usize + 1);
    win.copy_within(0..win.len() - 1, 1);

    // r12-w9b 收编（抬顶提交形）：尾写经 advance_top(1) 原语落笔——本函数契约保证
    // 全程不重分配栈、移位窗只写已界内槽不触 `top` 场域，故原式「场再读后 wrapping_add」
    // 与原语内 `self.top.add(1)` 同址同值（top 有空余槽见 # Safety），逐位等价且更贴
    // cpp `L->top++` 的现读场语义；断言位点原无，维持不带断言的提交形。
    (*l).advance_top(1);
    setobj_2_s!(l, func, tm);
  }
}

/// # Safety
/// C ABI 导出壳：签名与符号不动。前置条件同 [`lua_v_tryfunc_tm`]：`l` 为存活调用帧，
/// `func` 为指向其栈内实参槽的非空可写 `StkId`，调用期间槽地址不迁移。
pub unsafe extern "C-unwind" fn lua_v_tryfunc_tm_export(l: *mut LuaState, func: StkId) {
  // SAFETY: 导出壳在边界显式重建可写槽句柄后转调同契约 `lua_v_tryfunc_tm`；
  // `func` 为合法栈槽指针，写面前提满足 `from_raw` 纪律，解引用窗口止于本调用
  unsafe {
    lua_v_tryfunc_tm(l, Slot::from_raw(func));
  }
}
