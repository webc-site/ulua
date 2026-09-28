use ulua_compiler::records::compile_options::CompileOptions;

use crate::records::global_options::{get_debug_level, get_optimization_level};

/// 镜像 cpp CLI 的 `copts()`：`CompileOptions` 的组装点唯一收敛于此。
/// bytecode/repl 等共用这一默认形态（级别读 cli-lib 的共享全局，即 cpp
/// 同一组 `globalOptions`），compile-cli 则按自身 `GlobalOptions` 提供全量
/// 值，走 [`copts_with`]。
pub fn copts() -> CompileOptions {
  copts_with(
    get_optimization_level(),
    get_debug_level(),
    1,
    None,
    None,
    None,
  )
}

/// 全参数形态：级别三元组 + `vector_*` 三个宿主名（None 表示未设置，即 cpp
/// 的 nullptr）。组装为惯用 Rust 值（`Option<String>`），不再有 C 串指针透传。
pub fn copts_with(
  optimization_level: i32,
  debug_level: i32,
  type_info_level: i32,
  vector_lib: Option<&str>,
  vector_ctor: Option<&str>,
  vector_type: Option<&str>,
) -> CompileOptions {
  CompileOptions {
    optimization_level,
    debug_level,
    type_info_level,
    vector_lib: vector_lib.map(str::to_owned),
    vector_ctor: vector_ctor.map(str::to_owned),
    vector_type: vector_type.map(str::to_owned),
    ..Default::default()
  }
}
