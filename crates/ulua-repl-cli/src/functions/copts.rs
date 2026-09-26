use core::ptr::null;

use ulua_cli_lib::{
  functions::copts::copts_with,
  records::global_options::{get_debug_level, get_optimization_level},
};
use ulua_compiler::records::compile_options::CompileOptions as LuauCompileOptions;

use crate::functions::coverage_active::coverage_active;

/// cpp `copts()`: 以共享全局级别构造 `CompileOptions`，复用 cli-lib 的唯一组装点
/// `copts_with`（vector_* 在 repl 侧不设，落 cpp 同款 nullptr），仅叠加 repl 专有的
/// `coverage_level`（覆盖插桩开启时 2，否则 0）。级别读数与 type_info 默认与 cpp 一致。
pub fn copts() -> LuauCompileOptions {
  LuauCompileOptions {
    coverage_level: if coverage_active() { 2 } else { 0 },
    ..copts_with(
      get_optimization_level(),
      get_debug_level(),
      1,
      null(),
      null(),
      null(),
    )
  }
}
