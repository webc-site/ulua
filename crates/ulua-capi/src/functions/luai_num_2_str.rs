//! 本文件由 `crates/ulua-capi/tools/gen_capi.py` 自动生成（源：ulua-vm/src/functions/luai_num_2_str.rs）。
//! 导出壳为 `functions/shells.rs` 中模板宏 `capi_shell!` 的一次调用，壳契约见宏模板。
capi_shell!(luai_num_2_str, "ulua_luai_num2str", luai_num2str, [
  buf ptr [*mut c_char] "（`*mut c_char`）：指向 NUL 结尾的可写缓冲，对齐，容量满足被调写入需求且调用期间存活；",
  n val f64,
  => *mut c_char,
  @ret "- 返回值（`*mut c_char`）：指向 `buf` 内写出内容末尾（`buf + 写出长度`），由调用方按此定长使用；指向调用方自有缓冲内，不新分配内存；",
]);
