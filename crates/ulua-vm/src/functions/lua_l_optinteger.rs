use crate::records::lua_state::LuaState;

/// # Safety
///
/// `l`/缓冲指针必须存活，输出缓冲满足注释所述最小长度（如 LUA_DEBUG_SOURCEINFO、32+ 字节），fmt 与可变参严格匹配。
pub unsafe fn lua_l_optinteger(l: *mut LuaState, narg: i32, def: i32) -> i32 {
  unsafe {
    if (*l).is_none_or_nil(narg) {
      def
    } else {
      (*l).check_integer(narg)
    }
  }
}
