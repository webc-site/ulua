use core::ptr::{eq, null};

pub use crate::macros::lua_o_nilobject::LUA_O_NILOBJECT;
use crate::{
  functions::index_2_addr::index_2_addr, records::lua_state::LuaState,
  type_aliases::t_value::TValue,
};

/// `luaA_toobject` 核心（cpp `lapi.cpp`）。`l` 以引用传入（存活由类型保证）；
/// `idx` 为任意（伪）索引，越界经硬化的 `index_2_addr` 收敛为哨兵槽——此处与
/// cpp 一致把哨兵折叠为 NULL 返回。仅做地址换算，不解引用。
pub fn lua_a_toobject(l: &LuaState, idx: i32) -> *const TValue {
  let p = index_2_addr(l, idx);

  if eq(p, LUA_O_NILOBJECT) { null() } else { p }
}
