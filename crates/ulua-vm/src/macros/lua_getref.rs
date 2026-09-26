use crate::{
  functions::lua_rawgeti::lua_rawgeti, macros::lua_registryindex::LUA_REGISTRYINDEX,
  records::lua_state::LuaState,
};

#[inline(always)]
/// # Safety
///
/// `l` 必须指向存活的 `LuaState` 且栈顶已预留 1 槽；`ref_` 须为注册表中有效的整数引用键。
pub unsafe fn lua_getref(l: *mut LuaState, ref_: i32) {
  // Safety: 契约保证 `l` 存活，lua_rawgeti 仅按注册表整数键取值压栈
  unsafe {
    lua_rawgeti(l, LUA_REGISTRYINDEX, ref_);
  }
}
