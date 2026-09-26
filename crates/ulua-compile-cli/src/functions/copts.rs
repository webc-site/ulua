use alloc::vec::Vec;
use core::{ffi::c_char, ptr::null};

use ulua_cli_lib::functions::copts::copts_with;
use ulua_compiler::records::compile_options::CompileOptions;

use crate::records::global_options::with;

/// 「本结构持有的 NUL 结尾字节串」→ `*const c_char`（None 即 cpp 的 nullptr）；
/// 三个 vector_* 字段同构，单点收口。
fn vec_c_ptr(value: &Option<Vec<u8>>) -> *const c_char {
  value.as_ref().map_or_else(null, |s| s.as_ptr().cast())
}

/// cpp `copts()`: 以本 CLI 的线程局部全局选项构造 `CompileOptions`。
/// 结构组装已上收到 `ulua-cli-lib::functions::copts::copts_with`（与
/// bytecode 等 CLI 共用同一份字段映射），此处只做「全局读数 → 参数」适配：
/// vector_* 指向 GLOBAL_OPTIONS 持有的 NUL 结尾字节串, 随线程局部静态存活至进程退出;
/// 参数解析全部完成后才调用本函数。并行编译下每个任务先经
/// `global_options::restore` 在本线程重放快照再调用：所得指针指向本任务
/// 恢复出的堆缓冲，`CompileOptions` 于任务结束前析构，同线程下一次
/// restore 置换缓冲时已无存活指针，契约不失配。
pub fn copts() -> CompileOptions {
  with(|opts| {
    copts_with(
      opts.optimization_level,
      opts.debug_level,
      opts.type_info_level,
      vec_c_ptr(&opts.vector_lib),
      vec_c_ptr(&opts.vector_ctor),
      vec_c_ptr(&opts.vector_type),
    )
  })
}
