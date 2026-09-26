use ulua_ast::records::parse_error::ParseError;

use crate::functions::report::{report, report_string};

/// 报告解析错误 (cpp `reportError(name, ParseError)`)；bytecode/compile 两个 CLI 共用。
pub fn report_parse_error(name: &str, error: &ParseError) {
  report(name, error.get_location(), "SyntaxError", error.what());
}

/// [`report_parse_error`] 的无副作用版本，供并行编译按文件缓冲输出。
pub fn report_parse_error_string(name: &str, error: &ParseError) -> String {
  report_string(name, error.get_location(), "SyntaxError", error.what())
}
