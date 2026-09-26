use ulua_compiler::records::compile_error::CompileError;

use crate::functions::report::report_string;

/// cpp `reportError(name, CompileError)` 的无副作用形态（无消费者直接打印的
/// 变体已随死代码清理删除）：bytecode/compile 两个 CLI 经并行编译按文件缓冲输出。
pub(crate) fn report_compile_error_string(name: &str, error: &CompileError) -> String {
  report_string(
    name,
    error.get_location(),
    "CompileError",
    &error.to_string(),
  )
}
