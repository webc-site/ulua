use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::{enums::type_constant_folding::Type, records::constant::Constant};

impl Constant {
  pub fn is_truthful(&self) -> bool {
    LUAU_ASSERT!(self.r#type != Type::Unknown);
    self.r#type != Type::Nil
      && !(self.r#type == Type::Boolean && !unsafe { self.data.value_boolean })
  }
}
