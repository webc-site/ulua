use ulua_common::LUAU_ASSERT;

use crate::records::lexeme::{Lexeme, Type};

impl Lexeme {
  pub fn get_length(&self) -> u32 {
    LUAU_ASSERT!(
      self.r#type == Type::RAW_STRING
        || self.r#type == Type::QUOTED_STRING
        || self.r#type == Type::INTERP_STRING_BEGIN
        || self.r#type == Type::INTERP_STRING_MID
        || self.r#type == Type::INTERP_STRING_END
        || self.r#type == Type::INTERP_STRING_SIMPLE
        || self.r#type == Type::BROKEN_INTERP_DOUBLE_BRACE
        || self.r#type == Type::NUMBER
        || self.r#type == Type::COMMENT
        || self.r#type == Type::BLOCK_COMMENT
    );

    self.length
  }
}
