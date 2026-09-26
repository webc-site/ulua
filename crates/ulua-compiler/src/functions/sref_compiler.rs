use ulua_ast::records::{ast_array::AstArray, ast_name::AstName};
use ulua_bytecode::records::string_ref::StringRef;
use ulua_common::macros::luau_assert::LUAU_ASSERT;
/// cpp `Compiler.cpp:54` `sref(AstName)`：把 AST 名表里的字符串包成 `StringRef` 视图。
/// `'a` 由调用方决定（名表须活得比视图久，即覆盖整个字节码构建过程），因此底层
/// `StringRef::new` 的裸指针读区间在这里由调用方约定兑现。
pub(crate) fn sref_ast_name<'a>(name: AstName) -> StringRef<'a> {
  LUAU_ASSERT!(!name.is_null());
  // Safety: 调用方约定 AST 名表在 `'a` 全程存活（`CompileOptions`/`AstNameTable` 覆盖编译期）；
  // `StringRef::new` 与 `AstName::value` 同为 `*const u8` 字节域，无转换。
  unsafe { StringRef::new(name.value, name.len()) }
}

/// cpp `Compiler.cpp` `sref(sealed_ast::Array<const char>)`：名表/字面量 arena 视图。
/// 批 2 存储面 `AstArray<c_char>→AstArray<u8>`（cpp char 与 u8 同一字节域），
/// `StringRef::new` 同域收 `*const u8`，无指针转换。
/// `'a` 由调用方决定，约束同 `sref_ast_name`：数据缓冲须活得比视图久。
pub(crate) fn sref_ast_array_u8<'a>(data: AstArray<u8>) -> StringRef<'a> {
  // AstArray 新 API 暴露 pub data 字段（begin() 已移除）；空数组 data 为 null
  LUAU_ASSERT!(!data.data.is_null());
  // Safety: 调用方约定 `data` 指向的缓冲在 `'a` 全程可读
  unsafe { StringRef::new(data.data, data.len()) }
}
