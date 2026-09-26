//! 本文件由 `crates/ulua-capi/tools/gen_capi.py` 自动生成（源：ulua-vm/src/functions/lua_h_setnum.rs）。
//! 导出壳为 `functions/shells.rs` 中模板宏 `capi_shell!` 的一次调用，壳契约见宏模板。
capi_shell!(lua_h_setnum, "ulua_luaH_setnum", lua_h_setnum_export, [
  l state,
  t voidptr,
  key val c_int,
  => *mut TValue,
  @ret "- 返回值（`*mut TValue`）：指向表内可写值槽（数组段或 node），在该表下次 rehash/扩容前有效，跨表结构修改操作持有即悬挂；",
]);
