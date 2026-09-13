use core::ffi::c_int;

use crate::{
  enums::lua_type::LuaType,
  functions::{
    lua_g_readonlyerror::lua_g_readonlyerror, lua_h_resizearray::lua_h_resizearray,
    lua_l_checkinteger::lua_l_checkinteger, lua_l_checktype::lua_l_checktype,
    lua_pushvalue::lua_pushvalue, moveelements::moveelements,
  },
  macros::{
    hvalue::hvalue, lua_isnoneornil::lua_isnoneornil, lua_l_argcheck::luaL_argcheck,
    sizenode::sizenode,
  },
  type_aliases::lua_state::lua_State,
};

pub(crate) unsafe extern "C-unwind" fn tmove(l: *mut lua_State) -> c_int {
  unsafe {
    lua_l_checktype(l, 1, LuaType::Table as c_int);
    let f = lua_l_checkinteger(l, 2);
    let e = lua_l_checkinteger(l, 3);
    let t = lua_l_checkinteger(l, 4);
    let tt = if !lua_isnoneornil!(l, 5) { 5 } else { 1 };

    lua_l_checktype(l, tt, LuaType::Table as c_int);

    if e >= f {
      luaL_argcheck!(
        l,
        f > 0 || e < c_int::MAX + f,
        3,
        "too many elements to move"
      );
      let n = e - f + 1;
      luaL_argcheck!(l, t <= c_int::MAX - n + 1, 4, "destination wrap around");

      let src = hvalue!((*l).base);
      let dst = hvalue!((*l).base.offset((tt - 1) as isize));

      if (*dst).readonly != 0 {
        lua_g_readonlyerror(l);
      }

      let srcelems = (*src).sizearray + sizenode!(src);
      let dstelems = (*dst).sizearray + sizenode!(dst);
      let maxelems = srcelems.max(dstelems);
      let minsparsemoveelems = 32;
      let sparsemove = n > minsparsemoveelems && n / 2 > maxelems;

      if t > 0 && (t - 1) <= (*dst).sizearray && (t - 1 + n) > (*dst).sizearray {
        lua_h_resizearray(l, dst, t - 1 + n);
      }

      moveelements(l, 1, tt, f, e, t, sparsemove);
    }

    lua_pushvalue(l, tt);
    1
  }
}
