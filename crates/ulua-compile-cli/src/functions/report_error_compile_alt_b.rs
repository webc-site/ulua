use ulua_compiler::records::compile_error::CompileError;

use crate::functions::report::report;

/// 报告编译错误 (cpp `reportError(name, CompileError)`)
pub fn report_compile_error(name: &str, error: &CompileError) {
  report(
    name,
    error.get_location(),
    "CompileError",
    &error.to_string(),
  );
}
