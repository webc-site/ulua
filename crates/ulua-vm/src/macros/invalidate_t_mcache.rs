use crate::records::lua_table::LuaTable;

/// 清表元方法缺席缓存（cpp `invalidateTMcache(t)`）。
///
/// `tmcache` 为 `Cell<u8>`（内部可变），清零不需排他借用 ⇒ 收 `&`：调用点仅握有
/// 共享句柄（如 `fasttm` 链、`*const LuaTable`）也能失效缓存。
#[inline(always)]
pub(crate) fn invalidate_tmcache(t: &LuaTable) {
  t.tmcache.set(0);
}
