use crate::{
  enums::lua_type::LuaType,
  functions::{
    getfunc::getfunc, lua_insert::lua_insert, lua_iscfunction::lua_iscfunction,
    lua_isnumber::lua_isnumber, lua_l_checktype::lua_l_checktype, lua_l_error_l::lua_l_error_l,
    lua_pushthread::lua_pushthread, lua_pushvalue::lua_pushvalue, lua_setfenv::lua_setfenv,
    lua_setsafeenv::lua_setsafeenv,
  },
  macros::lua_tonumber::lua_tonumber,
  type_aliases::lua_state::lua_State,
};

pub(crate) unsafe extern "C-unwind" fn lua_b_setfenv(l: *mut lua_State) -> i32 {
  unsafe {
    lua_l_checktype(l, 2, LuaType::Table as i32);
    getfunc(l, 0);
    lua_pushvalue(l, 2);
    lua_setsafeenv(l, -1, 0);
    if lua_isnumber(l, 1) != 0 && lua_tonumber!(l, 1) == 0.0 {
      lua_pushthread(l);
      lua_insert(l, -2);
      lua_setfenv(l, -2);
      return 0;
    } else if lua_iscfunction(l, -2) != 0 || lua_setfenv(l, -2) == 0 {
      lua_l_error_l(
        l,
        c"'setfenv' cannot change environment of given object".as_ptr(),
        format_args!("'setfenv' cannot change environment of given object"),
      );
    }
    1
  }
}
