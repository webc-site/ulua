use crate::{
  macros::{api_check::api_check, lua_memory_categories::LUA_MEMORY_CATEGORIES},
  records::lua_state::LuaState,
};

/// # Safety
///
/// `l` 必须指向存活 `LuaState`，索引/长度/标签等参数满足各 API 注释约定，需压栈时栈顶预留由调用方保证。
pub unsafe fn lua_setmemcat(l: *mut LuaState, category: i32) {
  api_check!(l, (category as u32) < LUA_MEMORY_CATEGORIES as u32);
  // Safety: 契约保证 `l` 的 global 存活且 category 落在 MemCat 枚举界内，后续分配记账读该字段一致
  unsafe {
    (*l).activememcat = category as u8;
  }
}
