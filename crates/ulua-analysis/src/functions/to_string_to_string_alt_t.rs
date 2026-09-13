//! Node: `cxx:Function:Luau.Analysis:Analysis/src/ToString.cpp:2135:to_string`
//! Source: `Analysis/src/ToString.cpp:2135-2139` (hand-ported)

use alloc::{format, string::String};

use ulua_ast::records::location::Location;

/// C++ `std::string to_string(const Location& location, int offset = 0, bool useBegin = true)`.
/// NOTE: C++ ignores `useBegin` in the body (it prints both ends); preserved.
pub fn to_string_location_i32_bool(location: &Location, offset: i32, _use_begin: bool) -> String {
  format!(
    "({}, {}) - ({}, {})",
    location.begin.line as i64 + offset as i64,
    location.begin.column as i64 + offset as i64,
    location.end.line as i64 + offset as i64,
    location.end.column as i64 + offset as i64,
  )
}
