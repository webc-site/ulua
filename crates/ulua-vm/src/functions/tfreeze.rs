use crate::{
  enums::lua_type::LuaType,
  functions::{lua_getreadonly::lua_getreadonly, lua_l_getmetafield::lua_l_getmetafield},
  macros::{lua_lib_fn::lua_lib_fn, tm_metatable::TM_METATABLE},
  records::lua_state::LuaState,
};

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
pub unsafe fn tfreeze(l: *mut LuaState) -> i32 {
  unsafe {
    (*l).check_type(1, LuaType::Table);

    (*l).arg_check(lua_getreadonly(l, 1) == 0, 1, "table is already frozen");

    (*l).arg_check(
      lua_l_getmetafield(l, 1, TM_METATABLE.as_ptr().cast()) == 0,
      1,
      "table has a protected metatable",
    );

    (*l).set_readonly(1, true);

    (*l).push_value(1);
    1
  }
}

lua_lib_fn!(pub fn tfreeze, tfreeze_arm);
