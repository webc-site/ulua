//! 本文件由 `crates/ulua-capi/tools/gen_capi.py` 自动生成（源：ulua-vm/src/functions/lua_l_optinteger_64.rs）。
//! 导出壳为 `functions/shells.rs` 中模板宏 `capi_shell_check_opt!` 的一次调用，壳契约见宏模板。
capi_shell_check_opt!(
  lua_l_optinteger_64,
  lua_l_optinteger_64,
  i64,
  narg: c_int,
  def: i64
);
