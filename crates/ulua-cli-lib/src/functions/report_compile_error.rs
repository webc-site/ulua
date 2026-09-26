use ulua_compiler::records::compile_error::CompileError;

use crate::functions::report::report_string;

/// 报告编译错误 (cpp `reportError(name, CompileError)`)；bytecode/compile 两个 CLI 共用。
pub fn report_compile_error(name: &str, error: &CompileError) {
  eprint!("{}", report_compile_error_string(name, error));
}

/// [`report_compile_error`] 的无副作用版本，供并行编译按文件缓冲输出。
pub fn report_compile_error_string(name: &str, error: &CompileError) -> String {
  report_string(
    name,
    error.get_location(),
    "CompileError",
    &error.to_string(),
  )
}
