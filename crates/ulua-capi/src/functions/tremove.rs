//! 本文件对应 `ulua_tremove` 导出符号（源：ulua-vm/src/functions/tremove.rs）。
//! 导出壳为 `functions/shells.rs` 中模板宏 `capi_libfn_shell_l_cint!` 的一次调用，壳契约见宏模板。
capi_libfn_shell_l_cint!(tremove, tremove @ref);
