use crate::{functions::lua_l_checkvector::lua_l_checkvector, records::lua_state::LuaState};

/// # Safety
///
/// `l`/缓冲指针必须存活，输出缓冲满足注释所述最小长度（如 LUA_DEBUG_SOURCEINFO、32+ 字节），fmt 与可变参严格匹配。
pub(crate) unsafe fn lua_l_optvector(l: *mut LuaState, narg: i32, def: *const f32) -> *const f32 {
  unsafe {
    if (*l).is_none_or_nil(narg) {
      def
    } else {
      lua_l_checkvector(l, narg)
    }
  }
}
