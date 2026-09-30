//! 本文件由 `crates/ulua-capi/tools/gen_capi.py` 自动生成（源：ulua-vm/src/functions/lua_h_new.rs）。
//! 导出壳为 `functions/shells.rs` 中模板宏 `capi_shell!` 的一次调用，壳契约见宏模板。
capi_shell!(lua_h_new, "ulua_luaH_new", lua_h_new_export, [
  l state,
  narray val c_int,
  nhash val c_int,
  => *mut c_void,
  @ret "- 返回值（`*mut c_void`）：新表 `LuaTable*` 宽化为 `c_void`，由调用方还原；新表尚未入栈，须尽快置于 GC 可达槽位；OOM 抛错不以 null 返错；",
]);
