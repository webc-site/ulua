use ulua_ast::records::ast_name::AstName;
use ulua_bytecode::records::string_ref::StringRef;
use ulua_common::macros::luau_assert::LUAU_ASSERT;

pub fn sref_ast_name(name: AstName) -> StringRef<'static> {
  LUAU_ASSERT!(!name.value.is_null());
  // `StringRef::new` 无寿命检查：AstName 指向字符串驻留表，契约上随进程存续（cpp 同款）。
  StringRef::new(name.value, name.len())
}
