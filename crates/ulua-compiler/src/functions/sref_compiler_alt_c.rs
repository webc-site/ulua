use core::ffi::c_char;

use ulua_ast::records::ast_array::AstArray;
use ulua_bytecode::records::string_ref::StringRef;
use ulua_common::macros::luau_assert::LUAU_ASSERT;

pub fn sref_ast_array_c_char(data: AstArray<c_char>) -> StringRef {
  LUAU_ASSERT!(!data.begin().is_null());
  StringRef::new(data.begin(), data.len())
}
