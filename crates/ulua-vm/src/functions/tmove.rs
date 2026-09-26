use crate::{
  enums::lua_type::LuaType,
  functions::{
    lua_g_readonlyerror::check_writable, lua_h_resizearray::lua_h_resizearray,
    lua_l_checkinteger::lua_l_checkinteger, lua_l_checktype::lua_l_checktype,
    lua_pushvalue::lua_pushvalue, moveelements::moveelements,
  },
  macros::{lua_isnoneornil::lua_isnoneornil, lua_l_argcheck::luaL_argcheck, sizenode::sizenode},
  records::lua_state::LuaState,
};

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
pub(crate) unsafe extern "C-unwind" fn tmove(l: *mut LuaState) -> i32 {
  unsafe {
    lua_l_checktype(l, 1, LuaType::Table as i32);
    let f = lua_l_checkinteger(l, 2);
    let e = lua_l_checkinteger(l, 3);
    let t = lua_l_checkinteger(l, 4);
    let tt = if !lua_isnoneornil!(l, 5) { 5 } else { 1 };

    lua_l_checktype(l, tt, LuaType::Table as i32);

    if e >= f {
      luaL_argcheck!(l, f > 0 || e < i32::MAX + f, 3, "too many elements to move");
      let n = e - f + 1;
      luaL_argcheck!(l, t <= i32::MAX - n + 1, 4, "destination wrap around");

      let src = (*(*l).base).as_table_ptr();
      let dst = (*(*l).base.offset((tt - 1) as isize)).as_table_ptr();

      check_writable(l, dst);

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
