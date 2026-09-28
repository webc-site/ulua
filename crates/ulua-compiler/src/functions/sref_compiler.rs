use core::slice::from_raw_parts;

use ulua_ast::records::{ast_array::AstArray, ast_name::AstName};
use ulua_bytecode::records::string_ref::StringRef;
use ulua_common::macros::luau_assert::LUAU_ASSERT;
/// cpp `Compiler.cpp:54` `sref(AstName)`：把 AST 名表里的字符串包成 `StringRef` 视图。
/// `'a` 由调用方决定（名表须活得比视图久，即覆盖整个字节码构建过程），arena 裸指针
/// 到切片的转换只在 `ulua-ast` 名表这一处边界发生，`StringRef` 本体是 safe API。
pub(crate) fn sref_ast_name<'a>(name: AstName) -> StringRef<'a> {
  LUAU_ASSERT!(!name.is_null());
  // Safety: 调用方约定 AST 名表在 `'a` 全程存活（`CompileOptions`/`AstNameTable` 覆盖编译期），
  // `name.value..name.value.add(name.len())` 因此全程可读。
  let bytes = unsafe { from_raw_parts(name.value, name.len()) };
  StringRef::from_slice(bytes)
}

/// cpp `Compiler.cpp` `sref(sealed_ast::Array<const char>)`：名表/字面量 arena 视图。
/// 批 2 存储面 `AstArray<c_char>→AstArray<u8>`（cpp char 与 u8 同一字节域）。
/// `'a` 由调用方决定，约束同 `sref_ast_name`：数据缓冲须活得比视图久。
pub(crate) fn sref_ast_array_u8<'a>(data: AstArray<u8>) -> StringRef<'a> {
  // AstArray 新 API 暴露 pub data 字段（begin() 已移除）；空数组 data 为 null
  LUAU_ASSERT!(!data.data.is_null());
  // Safety: 调用方约定 `data.data..data.data.add(data.len())` 在 `'a` 全程可读。
  let bytes = unsafe { from_raw_parts(data.data, data.len()) };
  StringRef::from_slice(bytes)
}
