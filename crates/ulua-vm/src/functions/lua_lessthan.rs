use core::{ffi::c_int, ptr::eq};

use crate::{
  functions::{index_2_addr::index2addr, lua_v_lessthan::lua_v_lessthan},
  macros::lua_o_nilobject::luaO_nilobject,
  type_aliases::{lua_state::lua_State, stk_id::StkId, t_value::TValue},
};

#[unsafe(export_name = "ulua_lua_lessthan")]
pub(crate) unsafe fn lua_lessthan(l: *mut lua_State, index1: c_int, index2: c_int) -> c_int {
  unsafe {
    let o1: StkId = index2addr(l, index1);
    let o2: StkId = index2addr(l, index2);

    let nil_ptr = luaO_nilobject;

    if eq(o1, nil_ptr) || eq(o2, nil_ptr) {
      0
    } else {
      lua_v_lessthan(l, o1 as *const TValue, o2 as *const TValue)
    }
  }
}
