use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::{
  enums::type_constant_folding::Type,
  functions::{cstring_builtin_folding::cstring_str, cvar::cvar},
  records::constant::Constant,
};

pub fn ctypeof(c: &Constant) -> Constant {
  LUAU_ASSERT!(c.r#type != Type::Unknown);

  match c.r#type {
    Type::Nil => cstring_str("nil"),
    Type::Boolean => cstring_str("boolean"),
    Type::Number => cstring_str("number"),
    Type::Integer => cstring_str("integer"),
    Type::Vector => cvar(), // vector can have a custom typeof name at runtime
    Type::String => cstring_str("string"),
    _ => {
      LUAU_ASSERT!(false);
      cvar()
    }
  }
}
