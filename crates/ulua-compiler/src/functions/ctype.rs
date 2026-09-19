use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::{
  functions::{cstring_builtin_folding::cstring_str, cvar::cvar},
  records::constant::Constant,
};

pub fn ctype(c: &Constant) -> Constant {
  LUAU_ASSERT!(!c.is_unknown());

  match c {
    Constant::Nil => cstring_str("nil"),
    Constant::Boolean(_) => cstring_str("boolean"),
    Constant::Number(_) => cstring_str("number"),
    Constant::Integer(_) => cstring_str("integer"),
    Constant::Vector(_) => cstring_str("vector"),
    Constant::Str(_) => cstring_str("string"),
    // 仅 Unknown / Table 会到达
    _ => {
      LUAU_ASSERT!(false);
      cvar()
    }
  }
}
