use ulua_common::fflag::LuauCyclicRequireShortCircuit;
use ulua_vm::records::lua_state::LuaState;

use crate::functions::{
  cache_table_keys::{CYCLIC_PLACEHOLDER_PROVIDED_KEY, REQUIRED_CACHE_TABLE_KEY},
  cyclic_placeholder::is_placeholder_at,
  push_str::{KeyForm, c_str_prefix_owned},
  registry_table::{cache_hit, set_registry_mark},
};

/// cpp `isCached`：查 `_MODULES` 缓存表。
/// 命中时返回 true 且命中值留在栈顶（缓存表已移出，供调用方直接消费，
/// 免二次查表）。
pub(crate) fn is_cached(l: &mut LuaState, key: &[u8]) -> bool {
  if !cache_hit(l, REQUIRED_CACHE_TABLE_KEY, key, KeyForm::Raw) {
    return false;
  }

  // cpp `LuauCyclicRequireShortCircuit` 分支：缓存值是占位表（携带共享占位元表）
  // 时，标记该 cacheKey 的占位表已交给循环 require 方，供 `lua_requirecont` 决定
  // 填充占位表还是直接缓存结果。键取本地拷贝（cpp std::string 拷贝，避免其
  // 内部压栈动作与栈顶命中值互踩），set_registry_mark 自配平。
  if LuauCyclicRequireShortCircuit.get() && l.is_table(-1) && is_placeholder_at(l, -1) {
    let cache_key = c_str_prefix_owned(key);
    set_registry_mark(l, CYCLIC_PLACEHOLDER_PROVIDED_KEY, &cache_key, true);
  }

  // 命中值已留在栈顶（cache_hit 收尾时移除了缓存表），供调用方直接消费。
  true
}
