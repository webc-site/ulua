use core::ffi::c_char;

use ulua_ast::records::ast_array::AstArray;
use ulua_bytecode::records::string_ref::StringRef;
use ulua_common::macros::luau_assert::LUAU_ASSERT;

pub fn sref_ast_array_c_char(data: AstArray<c_char>) -> StringRef<'static> {
  // AstArray 新 API 暴露 pub data 字段（begin() 已移除）；空数组 data 为 null
  LUAU_ASSERT!(!data.data.is_null());
  // 同 `sref_ast_name`：底层字节由源缓冲/驻留表持有，契约上 'static（cpp 同款）。
  StringRef::new(data.data, data.len())
}
