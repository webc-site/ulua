use crate::{
  functions::lua_l_checkboolean::lua_l_checkboolean, macros::lua_l_opt::luaL_opt,
  records::lua_state::LuaState,
};

/// # Safety
///
/// `l` 必须指向存活 `LuaState`，索引/长度/标签等参数满足各 API 注释约定，需压栈时栈顶预留由调用方保证。
pub unsafe fn lua_l_optboolean(l: *mut LuaState, narg: i32, def: bool) -> bool {
  // Safety: 契约保证 `l` 存活且 narg 槽可读；nil/false 由标签区分，其余情况返回 def
  unsafe {
    let def_cint = if def { 1 } else { 0 };
    luaL_opt!(l, lua_l_checkboolean, narg, def_cint) != 0
  }
}
