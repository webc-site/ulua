use crate::records::lua_state::LuaState;

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
pub unsafe fn lua_l_optinteger_64(l: *mut LuaState, narg: i32, def: i64) -> i64 {
  unsafe {
    if (*l).is_none_or_nil(narg) {
      def
    } else {
      (*l).check_integer_64(narg)
    }
  }
}
