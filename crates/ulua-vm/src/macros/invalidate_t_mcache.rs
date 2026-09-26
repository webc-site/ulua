use crate::records::lua_table::LuaTable;

/// 清表元方法缺席缓存（cpp `invalidateTMcache(t)`）。
///
/// B 档契约前移（参照 `abs_index`/`isyielded` 先例）：`tmcache` 为普通 `u8` 字段，
/// 清零写不需 unsafe；原 `t` 存活/对齐契约改由 `&mut` 接收者在调用点承载。
#[inline(always)]
pub(crate) fn invalidate_tmcache(t: &mut LuaTable) {
  t.tmcache = 0;
}
