use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::{
  enums::type_constant_folding::Type,
  functions::{cstring_builtin_folding::cstring_str, cvar::cvar},
  records::constant::Constant,
};

pub fn ctype(c: &Constant) -> Constant {
  LUAU_ASSERT!(c.r#type != Type::Unknown);

  match c.r#type {
    Type::Nil => cstring_str("nil"),
    Type::Boolean => cstring_str("boolean"),
    Type::Number => cstring_str("number"),
    Type::Integer => cstring_str("integer"),
    Type::Vector => cstring_str("vector"),
    Type::String => cstring_str("string"),
    _ => {
      LUAU_ASSERT!(false);
      cvar()
    }
  }
}
