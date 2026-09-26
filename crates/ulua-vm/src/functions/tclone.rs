use crate::{
  enums::lua_type::LuaType,
  functions::{
    lua_a_pushvalue::lua_a_pushvalue, lua_h_clone::lua_h_clone, lua_l_checktype::lua_l_checktype,
    lua_l_getmetafield::lua_l_getmetafield,
  },
  macros::{lua_l_argcheck::luaL_argcheck, sethvalue::sethvalue, tm_metatable::TM_METATABLE},
  records::lua_state::LuaState,
  type_aliases::t_value::TValue,
};

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
pub unsafe extern "C-unwind" fn tclone(l: *mut LuaState) -> i32 {
  unsafe {
    lua_l_checktype(l, 1, LuaType::Table as i32);

    luaL_argcheck!(
      l,
      lua_l_getmetafield(l, 1, TM_METATABLE.as_ptr().cast()) == 0,
      1,
      "table has a protected metatable"
    );

    let tt = lua_h_clone(l, (*(*l).base).as_table_ptr());

    let mut v = TValue::default();
    sethvalue!(l, &mut v, tt);
    lua_a_pushvalue(l, &v);

    1
  }
}
