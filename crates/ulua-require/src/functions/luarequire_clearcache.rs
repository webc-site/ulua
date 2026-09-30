use ulua_vm::{macros::lua_registryindex::LUA_REGISTRYINDEX, records::lua_state::LuaState};

use crate::functions::{
  cache_table_keys::REQUIRED_CACHE_TABLE_KEY, registry_table::set_table_field,
};

/// 对应 cpp `luarequire_clearcache`（Require.cpp 壳 + RequireImpl.cpp 实体两段）：
/// Rust 侧无 TU 分离需求，壳与实体合一（review.md §3「只包一层的壳合并」）。
///
/// # Safety
/// `l` 必须指向存活的 `LuaState`（VM 以 Lua/C API 调本 C 函数时传入）。
pub unsafe extern "C-unwind" fn luarequire_clearcache(l: *mut LuaState) -> i32 {
  // Safety: 契约保证 l 为存活独占 state，重建借用无别名冲突；new_table 经 VM API
  // 操作栈，set_table_field 门面把键按 ptr+len 形态即时压栈并消费，栈净减一。
  let l = unsafe { &mut *l };
  l.new_table();
  set_table_field(l, LUA_REGISTRYINDEX, REQUIRED_CACHE_TABLE_KEY);
  0
}
