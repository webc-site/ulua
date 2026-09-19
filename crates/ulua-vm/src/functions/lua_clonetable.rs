use core::ffi::c_int;

use crate::{
  functions::{
    ensure_stack::ensure_stack, index_2_addr::index_2_addr, lua_concat::lua_c_threadbarrier_lapi,
    lua_h_clone::lua_h_clone,
  },
  macros::{
    api_check::api_check, api_incr_top::api_incr_top, hvalue::hvalue, lua_c_check_gc::luaC_checkGC,
    sethvalue::sethvalue, ttistable::ttistable,
  },
  type_aliases::{lua_state::lua_State, lua_table::LuaTable, stk_id::StkId},
};

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
pub unsafe fn lua_clonetable(l: *mut lua_State, idx: c_int) {
  unsafe {
    luaC_checkGC!(l);
    lua_c_threadbarrier_lapi(l);
    // cpp `ensure_stack(L, 1)`：随后 sethvalue 直接写 L->top
    ensure_stack(l, 1);

    let t: StkId = index_2_addr(l, idx);

    api_check!(l, ttistable!(t));

    let tt: *mut LuaTable = lua_h_clone(l, hvalue!(t));

    sethvalue!(l, (*l).top, tt);
    api_incr_top!(l);
  }
}
