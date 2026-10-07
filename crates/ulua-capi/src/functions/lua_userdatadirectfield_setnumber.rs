//! 本文件对应 `ulua_lua_userdatadirectfield_setnumber` 导出符号（源：ulua-vm/src/functions/lua_userdatadirectfield_setnumber.rs）。
//! 导出壳为 `functions/shells.rs` 中模板宏 `capi_shell_udfield_set!` 的一次调用，壳契约见宏模板。
capi_shell_udfield_set!(lua_userdatadirectfield_setnumber, lua_userdatadirectfield_setnumber, n: f64);
