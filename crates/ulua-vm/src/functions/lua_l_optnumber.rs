use crate::{
  functions::lua_l_checknumber::lua_l_checknumber, macros::lua_l_opt::luaL_opt,
  records::lua_state::LuaState,
};

/// # Safety
///
/// `l`/缓冲指针必须存活，输出缓冲满足注释所述最小长度（如 LUA_DEBUG_SOURCEINFO、32+ 字节），fmt 与可变参严格匹配。
pub(crate) unsafe fn lua_l_optnumber(l: *mut LuaState, narg: i32, def: f64) -> f64 {
  // Safety: `luaL_opt!` 仅在实参非 nil 时展开为同契约的 check 调用；`l` 存活且 `narg` 槽可读由本函数契约传递
  unsafe { luaL_opt!(l, lua_l_checknumber, narg, def) }
}
