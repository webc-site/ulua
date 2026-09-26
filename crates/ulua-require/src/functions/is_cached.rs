use ulua_common::fflag::LuauCyclicRequireShortCircuit;
use ulua_vm::{macros::lua_istable::lua_istable, records::lua_state::LuaState};

use crate::functions::{
  c_str_prefix::push_c_str,
  cache_table_keys::REQUIRED_CACHE_TABLE_KEY,
  cyclic_placeholder::{is_placeholder_at, mark_placeholder_provided},
  registry_table::cache_hit,
};

/// # Safety
/// `l` 必须指向存活的 `LuaState`；函数在 Lua 栈上压入/弹出临时值。
/// 命中时返回 true 且命中值留在栈顶（缓存表已移除，供调用方直接消费，
/// 免二次查表——与 check_registered_modules 同形态）。
pub(crate) unsafe fn is_cached(l: *mut LuaState, key: &[u8]) -> bool {
  // Safety: l 为调用方（resolve_require 链路）传入且本帧存活的 LuaState；查表
  // 整段收口 registry_table::cache_hit（push_c_str 按首个 NUL 截断以 ptr+len 推键，
  // 与 cpp `lua_getfield(L, -1, key.c_str())` 的 strlen 语义一致；收尾栈纪律由
  // take_cache_hit 承担）。占位标记只在命中、且栈顶确认为表时调用
  // is_placeholder_at/mark_placeholder_provided（二者栈配平，收尾后栈顶仍是命中值）。
  // cpp `LuauCyclicRequireShortCircuit` 分支：缓存值是占位表（携带共享占位元表）
  // 时，标记该 cacheKey 的占位表已交给循环 require 方，供 `lua_requirecont` 决定
  // 填充占位表还是直接缓存结果；占位表恒非 nil，判定挪到命中分支后与拆流前的
  // 「取键后、收尾前」次序净栈效果一致（两标记门面自身配平）。
  unsafe {
    if !cache_hit(l, REQUIRED_CACHE_TABLE_KEY, key, push_c_str) {
      return false;
    }

    if LuauCyclicRequireShortCircuit.get() && lua_istable!(l, -1) && is_placeholder_at(l, -1) {
      mark_placeholder_provided(l, key);
    }

    true
  }
}
