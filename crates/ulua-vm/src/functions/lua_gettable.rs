use core::{ffi::c_int, ptr::eq};

use crate::{
  functions::{
    index_2_addr::index2addr, lua_concat::lua_c_threadbarrier_lapi, lua_v_gettable::lua_v_gettable,
  },
  macros::{
    api_check::api_check, api_checknelems::api_checknelems, lua_o_nilobject::luaO_nilobject,
    ttype::ttype,
  },
  records::lua_state::lua_State,
  type_aliases::stk_id::StkId,
};

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
pub unsafe fn lua_gettable(l: *mut lua_State, idx: c_int) -> c_int {
  unsafe {
    api_checknelems!(l, 1);
    lua_c_threadbarrier_lapi(l);

    let t: StkId = index2addr(l, idx);
    api_check!(l, !eq(t, luaO_nilobject));
    lua_v_gettable(l, t, (*l).top.sub(1), (*l).top.sub(1));

    ttype!((*l).top.sub(1))
  }
}
