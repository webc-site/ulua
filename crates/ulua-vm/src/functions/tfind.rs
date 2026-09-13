use core::ffi::c_int;

use crate::{
  enums::lua_type::LuaType,
  functions::{
    lua_h_getnum::lua_h_getnum, lua_l_argerror_l::lua_l_argerror_l, lua_l_checkany::lua_l_checkany,
    lua_l_checktype::lua_l_checktype, lua_l_optinteger::lua_l_optinteger,
    lua_pushinteger::lua_pushinteger, lua_pushnil::lua_pushnil,
  },
  macros::{equalobj::equalobj, hvalue::hvalue, ttisnil::ttisnil},
  type_aliases::{lua_state::lua_State, stk_id::StkId, t_value::TValue},
};

#[unsafe(export_name = "ulua_tfind")]
pub(crate) unsafe extern "C-unwind" fn tfind(l: *mut lua_State) -> c_int {
  unsafe {
    lua_l_checktype(l, 1, LuaType::Table as c_int);
    lua_l_checkany(l, 2);
    let init = lua_l_optinteger(l, 3, 1);
    if init < 1 {
      // The dependency card for lua_l_argerror_l shows it takes &str.
      lua_l_argerror_l(l, 3, "index out of range");
    }

    let t = hvalue!((*l).base);

    let mut i = init;
    loop {
      let e: *const TValue = lua_h_getnum(t, i);
      if ttisnil!(e) {
        break;
      }

      let v: StkId = (*l).base.offset(1);

      if equalobj!(l, v, e) {
        lua_pushinteger(l, i);
        return 1;
      }
      // C++ does `i++` unconditionally; if the table has an element at INT_MAX
      // that doesn't match, the increment is signed-overflow UB (upstream
      // ltablib.cpp:533). There is no valid index past INT_MAX, so stop cleanly.
      if i == c_int::MAX {
        break;
      }
      i += 1;
    }

    lua_pushnil(l);
    1
  }
}
