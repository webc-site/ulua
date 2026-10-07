use ulua_cli_lib::functions::copts::copts as shared_copts;
use ulua_compiler::records::compile_options::CompileOptions as LuauCompileOptions;

use crate::functions::coverage_active::coverage_active;

/// cpp `copts()`: 复用 cli-lib 的唯一组装点 `copts()`（级别读数与 type_info 默认
/// 与 cpp 一致，vector_* 在 repl 侧不设），仅叠加 repl 专有的 `coverage_level`
/// （覆盖插桩开启时 2，否则 0）。
pub fn copts() -> LuauCompileOptions {
  LuauCompileOptions {
    coverage_level: if coverage_active() { 2 } else { 0 },
    ..shared_copts()
  }
}
