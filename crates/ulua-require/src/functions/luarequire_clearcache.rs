use ulua_vm::{
  macros::{lua_newtable::lua_newtable, lua_registryindex::LUA_REGISTRYINDEX},
  records::lua_state::LuaState,
};

use crate::functions::{
  cache_table_keys::REQUIRED_CACHE_TABLE_KEY, registry_table::set_table_field,
};

/// 对应 cpp `luarequire_clearcache`（Require.cpp 壳 + RequireImpl.cpp 实体两段）：
/// Rust 侧无 TU 分离需求，壳与实体合一（review.md §3「只包一层的壳合并」）。
///
/// # Safety
/// `l` 必须指向存活的 `LuaState`。
pub unsafe extern "C-unwind" fn luarequire_clearcache(l: *mut LuaState) -> i32 {
  // Safety: l 为宿主按 Lua/C API 契约提供的存活 LuaState；lua_newtable 经 VM API
  // 操作栈，无裸指针解引用；set_table_field 门面把键按 ptr+len 形态即时压栈并
  // 消费，栈净减一（本帧恰有此一值）。
  unsafe {
    lua_newtable(l);

    set_table_field(l, LUA_REGISTRYINDEX, REQUIRED_CACHE_TABLE_KEY);
  }
  0
}
