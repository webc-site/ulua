//! 本文件对应 `ulua_byteoffset` 导出符号（源：ulua-vm/src/functions/byteoffset.rs）。
//! 导出壳为 `functions/shells.rs` 中模板宏 `capi_shell_l_cint!` 的一次调用，壳契约见宏模板。
capi_shell_l_cint!(byteoffset, byteoffset);
