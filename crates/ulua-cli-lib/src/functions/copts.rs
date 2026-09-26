use core::{ffi::c_char, ptr::null};

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
    null(),
    null(),
    null(),
  )
}

/// 全参数形态：级别三元组 + `vector_*` 三个 C 字符串指针（null 表示未设置）。
/// vector_* 指向调用方持有的「NUL 结尾字节串」（`Vec<u8>` 末尾补 0），
/// 其存活期由调用方的全局静态保证。
///
/// 形参保持 `*const c_char` 而非 `Option<&CStr>`：`CompileOptions::vector_*`
/// 本身就是 `*const c_char`（`ulua-compiler` 的 C 形选项面，与 cpp
/// `CompileOptions` 逐字段对齐），此处若先转成 `CStr` 再 `as_ptr()`，只是把同
/// 一个裸指针换一层包装。该字段的 Rust化属 compiler 侧的独立改造，波及
/// `compile-cli`/`bytecode-cli`/`repl-cli` 全部调用点，不在本 crate 的作用域内。
pub fn copts_with(
  optimization_level: i32,
  debug_level: i32,
  type_info_level: i32,
  vector_lib: *const c_char,
  vector_ctor: *const c_char,
  vector_type: *const c_char,
) -> CompileOptions {
  CompileOptions {
    optimization_level,
    debug_level,
    type_info_level,
    vector_lib,
    vector_ctor,
    vector_type,
    ..Default::default()
  }
}
