use crate::{
  macros::{api_check::api_check, lua_memory_categories::LUA_MEMORY_CATEGORIES},
  records::lua_state::LuaState,
};

/// 设置当前记账内存类别（`lua_setmemcat`）。`l` 以引用传入（存活由类型保证）；
/// `category` 须落在 MemCat 枚举界内（`api_check` debug 校验），越界值只写截断字节、
/// 后续分配记账读该字段一致。
pub fn lua_setmemcat(l: &mut LuaState, category: i32) {
  api_check!(l, (category as u32) < LUA_MEMORY_CATEGORIES as u32);
  l.activememcat = category as u8;
}
