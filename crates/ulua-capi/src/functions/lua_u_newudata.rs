//! 本文件由 `crates/ulua-capi/tools/gen_capi.py` 自动生成（源：ulua-vm/src/functions/lua_u_newudata.rs）。
//! 导出壳为 `functions/shells.rs` 中模板宏 `capi_shell!` 的一次调用，壳契约见宏模板。
capi_shell!(lua_u_newudata, "ulua_luaU_newudata", lua_u_newudata_export, [
  l state,
  s val usize,
  tag val c_int,
  => *mut Udata,
  @ret "- 返回值（`*mut Udata`）：指向新分配且已 `luaC_init` 的 userdata，尚未入栈，调用方须尽快置于可达槽位；`s` 超限抛 `lua_m_toobig`（unwind），不以 null 返错；",
]);
