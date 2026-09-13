use core::ffi::c_char;

use ulua_ast::records::parse_error::ParseError;

use crate::functions::report::report;

pub fn report_error_c_char_luau_parse_error(name: *const c_char, error: &ParseError) {
  report(name, error.get_location(), "SyntaxError", error.what());
}
