use core::ffi::c_char;

use ulua_compiler::records::compile_error::CompileError;

use crate::functions::report::report;

pub fn report_error_c_char_luau_compile_error(name: *const c_char, error: &CompileError) {
  report(
    name,
    error.get_location(),
    "CompileError",
    &error.to_string(),
  );
}
