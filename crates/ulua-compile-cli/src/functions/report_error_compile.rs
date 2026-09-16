use ulua_ast::records::parse_error::ParseError;

use crate::functions::report::report;

/// 报告解析错误 (cpp `reportError(name, ParseError)`)
pub fn report_parse_error(name: &str, error: &ParseError) {
  report(name, error.get_location(), "SyntaxError", error.what());
}
