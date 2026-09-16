use ulua_ast::records::parse_error::ParseError;

use crate::functions::report::report;

/// 报告解析错误
pub fn report_error_c_char_luau_parse_error(name: &str, error: &ParseError) {
  report(name, error.get_location(), "SyntaxError", error.what());
}
