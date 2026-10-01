use crate::{
  functions::{cstr, cstr_cow, lua_l_optlstring::lua_l_optlstring},
  macros::{lua_l_error::luaL_error, lua_lib_fn::lua_lib_fn},
  records::lua_state::LuaState,
};

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
pub unsafe fn lua_b_assert(l: *mut LuaState) -> i32 {
  unsafe {
    (*l).check_any(1);
    if !(*l).to_boolean(1) {
      let mut len = 0;
      let msg = lua_l_optlstring(l, 2, cstr(b"assertion failed!\0"), &mut len);
      let msg = cstr_cow(msg);
      luaL_error!(l, "{}", msg);
    }
    (*l).get_top()
  }
}

lua_lib_fn!(pub fn lua_b_assert, lua_b_assert_arm);
