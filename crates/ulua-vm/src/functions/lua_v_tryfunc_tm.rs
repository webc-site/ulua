use crate::{
  enums::tms::TMS,
  functions::lua_t_gettmbyobj::lua_t_gettmbyobj,
  macros::{lua_g_typeerror::luaG_typeerror, setobj_2_s::setobj_2_s},
  records::lua_state::LuaState,
  type_aliases::stk_id::StkId,
};

/// # Safety
/// `l` 须为存活调用帧且 `func` 指向其栈内实参槽、自 `func` 到 `(*l).top` 连续可写
/// （cpp lvmutils.cpp:828）：块内经 `lua_t_gettmbyobj` 取 __call 元方法，随后把
/// `[func, top)` 整体后移一格再把 tm 写入 `func`，栈余量不足即越界写。
pub unsafe fn lua_v_tryfunc_tm(l: *mut LuaState, func: StkId) {
  // Safety: 契约保证 func 与 [func, top) 区间为当前帧可写栈槽、top 有空余，挪位与覆写均不越栈界
  unsafe {
    let tm = lua_t_gettmbyobj(l, func, TMS::TmCall);
    if !(*tm).is_function() {
      luaG_typeerror!(l, func, "call");
    }

    let mut p = (*l).top;
    while p > func {
      setobj_2_s!(l, p, p.wrapping_sub(1));
      p = p.wrapping_sub(1);
    }

    (*l).top = (*l).top.wrapping_add(1);
    setobj_2_s!(l, func, tm);
  }
}
