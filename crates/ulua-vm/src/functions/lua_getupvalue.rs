use core::{ffi::c_char, ptr::null};

use crate::{
  functions::{
    aux_upvalue::aux_upvalue, ensure_stack::ensure_stack, index_2_addr::index_2_addr,
    lapi_barrier::lua_c_threadbarrier_lapi,
  },
  macros::{api_incr_top::api_incr_top, setobj_2_s::setobj_2_s},
  records::lua_state::LuaState,
};

/// # Safety
/// `l` 须为存活 `LuaState`：`funcindex` 为合法（伪）索引且 `index_2_addr` 所得须为闭包供 `aux_upvalue(..,n)` 取第 n
/// 个 upvalue（越界返回 None→NULL）；命中分支已 `ensure_stack(l,1)` 后 `setobj_2_s` 写 `(*l).top` 再 `api_incr_top`
/// （前须留 ≥1 槽）。返回的 name 指向上值名串（生命周期随该闭包）。跨线程经 threadbarrier 同步；本路径不触发 GC。
/// cpp VM/src/lapi.cpp:1723
pub unsafe fn lua_getupvalue(l: *mut LuaState, funcindex: i32, n: i32) -> *const c_char {
  unsafe {
    lua_c_threadbarrier_lapi(l);
    // cpp `ensure_stack(L, 1)`：写 L->top 只在 name 非空的分支里发生
    ensure_stack(l, 1);
    match aux_upvalue(index_2_addr(l, funcindex), n) {
      Some((name, val)) => {
        setobj_2_s!(l, (*l).top, val);
        api_incr_top!(l);
        name
      }
      None => null(),
    }
  }
}
