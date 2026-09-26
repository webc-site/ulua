use crate::macros::lua_registryindex::LUA_REGISTRYINDEX;
#[inline(always)]
pub const fn lua_ispseudo(i: i32) -> bool {
  i <= LUA_REGISTRYINDEX
}
