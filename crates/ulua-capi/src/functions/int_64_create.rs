//! 本文件对应 `ulua_int64_create` 导出符号（源：ulua-vm/src/functions/int_64_create.rs）。
//! 导出壳为 `functions/shells.rs` 中模板宏 `capi_shell_l_cint!` 的一次调用，壳契约见宏模板。
capi_shell_l_cint!(int_64_create, int64_create);
