use crate::{
  functions::{
    getfunc::getfunc, lua_getfenv::lua_getfenv, lua_iscfunction::lua_iscfunction,
    lua_pushvalue::lua_pushvalue, lua_setsafeenv::lua_setsafeenv,
  },
  macros::lua_globalsindex::LUA_GLOBALSINDEX,
  type_aliases::lua_state::lua_State,
};

pub(crate) unsafe extern "C-unwind" fn lua_b_getfenv(l: *mut lua_State) -> i32 {
  unsafe {
    getfunc(l, 1);
    if lua_iscfunction(l, -1) != 0 {
      lua_pushvalue(l, LUA_GLOBALSINDEX);
    } else {
      lua_getfenv(l, -1);
    }
    lua_setsafeenv(l, -1, 0);
    1
  }
}
