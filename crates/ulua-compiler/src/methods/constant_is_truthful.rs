use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::records::constant::Constant;

impl Constant {
  pub fn is_truthful(&self) -> bool {
    LUAU_ASSERT!(!self.is_unknown());
    match self {
      Self::Nil => false,
      Self::Boolean(b) => *b,
      // nil、false 之外皆真；Unknown 由前置断言排除
      _ => true,
    }
  }
}
