use ulua_common::LUAU_ASSERT;

use crate::records::lexeme::{
  Lexeme, QuoteStyle,
  QuoteStyle::{Double, Single},
  Type,
};

impl Lexeme {
  pub fn get_quote_style(&self) -> QuoteStyle {
    LUAU_ASSERT!(self.r#type == Type::QUOTED_STRING);

    // If we have a well-formed string, we are guaranteed to see a closing delimiter after the string
    let data_ptr = unsafe { self.data.data };
    LUAU_ASSERT!(!data_ptr.is_null());

    let quote = unsafe { *data_ptr.add(self.length as usize) as u8 as char };
    if quote == '\'' {
      return Single;
    } else if quote == '"' {
      return Double;
    }

    LUAU_ASSERT!(false);
    Double // unreachable, but required due to compiler warning
  }
}
