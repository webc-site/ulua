use ulua_ast::records::ast_name::AstName;
use ulua_bytecode::records::string_ref::StringRef;
use ulua_common::macros::luau_assert::LUAU_ASSERT;

pub fn sref_ast_name(name: AstName) -> StringRef {
  LUAU_ASSERT!(!name.value.is_null());
  StringRef::new(name.value, name.len())
}
