use ulua_vm::enums::lua_type::{LUA_T_COUNT, LuaType};

use crate::macros::codegen_assert::CODEGEN_ASSERT;

pub fn is_gco(tag: u8) -> bool {
  CODEGEN_ASSERT!(tag < LUA_T_COUNT as u8);

  // mirrors iscollectable(o) from VM/lobject.h
  tag >= LuaType::String as u8
}
