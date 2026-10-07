//! 本文件对应 `ulua_luaV_gettable` 导出符号（源：ulua-vm/src/functions/lua_v_gettable.rs）。
//! 导出壳为 `functions/shells.rs` 中模板宏 `capi_shell_tkeyval!` 的一次调用，壳契约见宏模板。
capi_shell_tkeyval!(lua_v_gettable, lua_v_gettable_export, "ulua_luaV_gettable");
