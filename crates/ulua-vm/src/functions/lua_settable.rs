use core::{ffi::c_int, ptr::eq};

use crate::{
  functions::{index_2_addr::index2addr, lua_v_settable::lua_v_settable},
  macros::{
    api_check::api_check, api_checknelems::api_checknelems, lua_o_nilobject::luaO_nilobject,
  },
  records::lua_state::lua_State,
  type_aliases::stk_id::StkId,
};

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
pub unsafe fn lua_settable(l: *mut lua_State, idx: c_int) {
  unsafe {
    api_checknelems!(l, 2);

    let t: StkId = index2addr(l, idx);
    api_check!(l, !eq(t, luaO_nilobject));
    lua_v_settable(l, t, (*l).top.sub(2), (*l).top.sub(1));
    (*l).top = (*l).top.sub(2);
  }
}
