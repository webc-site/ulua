//! 本文件对应 `ulua_lua_userdatadirectfield_setinteger64` 导出符号（源：ulua-vm/src/functions/lua_userdatadirectfield_setinteger_64.rs）。
//! 导出壳为 `functions/shells.rs` 中模板宏 `capi_shell_udfield_set!` 的一次调用，壳契约见宏模板。
//!
//! B1c 票面 4：曾有第二枚 `_64` 后缀重影壳（导出符号
//! `ulua_lua_userdatadirectfield_setinteger_64`），源侧转发体零消费（cpp oracle 只声明
//! `lua_userdatadirectfield_setinteger64`，见源文件头核查记录），已连同符号删除；保留的
//! `ulua_lua_userdatadirectfield_setinteger64` 符号名、参数布局与契约文案不变。
capi_shell_udfield_set!(lua_userdatadirectfield_setinteger_64, lua_userdatadirectfield_setinteger64, n: i64);
