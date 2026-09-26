use ulua_vm::enums::lua_type::{LUA_T_COUNT, LuaType};

use crate::macros::codegen_assert::CODEGEN_ASSERT;

pub fn is_gco(tag: u8) -> bool {
  CODEGEN_ASSERT!(tag < LUA_T_COUNT as u8);

  // 对应 VM lobject.h 的 iscollectable(o)
  tag >= LuaType::String as u8
}
