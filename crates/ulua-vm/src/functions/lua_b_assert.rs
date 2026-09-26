use crate::{
  functions::{
    cstr_cow, lua_gettop::lua_gettop, lua_l_checkany::lua_l_checkany,
    lua_l_optlstring::lua_l_optlstring, lua_toboolean::lua_toboolean,
  },
  macros::lua_l_error::luaL_error,
  records::lua_state::LuaState,
};

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
pub unsafe extern "C-unwind" fn lua_b_assert(l: *mut LuaState) -> i32 {
  unsafe {
    lua_l_checkany(l, 1);
    if lua_toboolean(l, 1) == 0 {
      let mut len = 0;
      let msg = lua_l_optlstring(l, 2, c"assertion failed!".as_ptr(), &mut len);
      let msg = cstr_cow(msg);
      luaL_error!(l, "{}", msg);
    }
    lua_gettop(l)
  }
}
