use core::{
  ffi::{c_char, c_int},
  ptr::eq,
};

use crate::{
  functions::{
    index_2_addr::index2addr, lua_concat::lua_c_threadbarrier_lapi, lua_v_gettable::lua_v_gettable,
  },
  macros::{
    api_check::api_check, api_incr_top::api_incr_top, lua_o_nilobject::luaO_nilobject,
    lua_s_new::luaS_new, setsvalue::setsvalue, ttype::ttype,
  },
  records::{lua_state::lua_State, lua_t_value::TValue},
  type_aliases::stk_id::StkId,
};

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
pub unsafe fn lua_getfield(l: *mut lua_State, idx: c_int, k: *const c_char) -> c_int {
  unsafe {
    lua_c_threadbarrier_lapi(l);

    let t: StkId = index2addr(l, idx);
    api_check!(l, !eq(t, luaO_nilobject));

    let mut key = TValue::default();
    setsvalue!(l, &mut key, luaS_new(l, k));
    lua_v_gettable(l, t, &mut key, (*l).top);
    api_incr_top!(l);

    ttype!((*l).top.sub(1))
  }
}
