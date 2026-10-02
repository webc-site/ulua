use crate::{
  enums::tms::TMS,
  functions::lua_t_gettmbyobj::lua_t_gettmbyobj,
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
    // 句柄落回裸槽视图一处：帧挪位协议（`p > func` 区间比较与逐格后移）是帧内裸指针
    // 算术，按设计票边界不变量保持 `StkId` 形态；`as_ptr` 为 `inline(always)` 指针
    // 读出，与原裸形参同址同宽度（§9.4）
    let func = func.as_ptr();

    let tm = lua_t_gettmbyobj(l, func, TMS::TmCall);
    if !(*tm).is_function() {
      luaG_typeerror!(l, func, "call");
    }

    // r12-w7a2 收编（同形单点·挪位协议窗）：`(*l).top` 预绑定单次读，供后移循环
    // 与抬顶尾写共用——本函数契约保证全程不重分配栈（见 # Safety），循环仅写
    // 已界内槽、不触场域，故尾写与原「场域再读后 wrapping_add」逐位同值；
    // wrapping 形态保留（与原式同宽同回绕语义）
    let top = (*l).top;
    let mut p = top;
    while p > func {
      setobj_2_s!(l, p, p.wrapping_sub(1));
      p = p.wrapping_sub(1);
    }

    (*l).top = top.wrapping_add(1);
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
