//! 本文件对应 `ulua_tpack` 导出符号（源：ulua-vm/src/functions/tpack.rs）。
//! 导出壳为 `functions/shells.rs` 中模板宏 `capi_shell!` 的一次调用，壳契约见宏模板。
capi_shell!(tpack, "ulua_tpack", tpack, [
  l refstate,
  => c_int,
]);
