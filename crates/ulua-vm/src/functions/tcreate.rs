use core::ffi::c_int;

use crate::{
  functions::{
    c_slice_mut, lua_createtable::lua_createtable, lua_l_checkinteger::lua_l_checkinteger,
  },
  luaL_argerror,
  macros::{hvalue::hvalue, lua_isnoneornil::lua_isnoneornil},
  setobj2t,
  type_aliases::{lua_state::lua_State, stk_id::StkId, t_value::TValue},
};

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
#[cfg_attr(feature = "capi", unsafe(export_name = "ulua_tcreate"))]
pub(crate) unsafe extern "C-unwind" fn tcreate(l: *mut lua_State) -> c_int {
  unsafe {
    let size = lua_l_checkinteger(l, 1);
    if size < 0 {
      luaL_argerror!(l, 1, "size out of range");
    }

    if !lua_isnoneornil!(l, 2) {
      lua_createtable(l, size as c_int, 0);
      let t = hvalue!((*l).top.offset(-1));

      let v: StkId = (*l).base.add(1);

      for e in c_slice_mut((*t).array, size as usize) {
        setobj2t!(l, e as *mut TValue, v);
      }
    } else {
      lua_createtable(l, size as c_int, 0);
    }

    1
  }
}
