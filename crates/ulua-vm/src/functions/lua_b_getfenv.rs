use crate::{
  functions::{
    getfunc::getfunc, lua_getfenv::lua_getfenv, lua_iscfunction::lua_iscfunction,
    lua_pushvalue::lua_pushvalue, lua_setsafeenv::lua_setsafeenv,
  },
  macros::{lua_globalsindex::LUA_GLOBALSINDEX, lua_lib_fn::lua_lib_fn},
  records::lua_state::LuaState,
};

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
pub(crate) unsafe fn lua_b_getfenv(l: *mut LuaState) -> i32 {
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

lua_lib_fn!(pub(crate) fn lua_b_getfenv, lua_b_getfenv_arm);
