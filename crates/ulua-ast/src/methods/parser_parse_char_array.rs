use core::{ffi::c_char, slice::from_raw_parts};

use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::records::{ast_array::AstArray, lexeme::Type, lexer::Lexer, parser::Parser};

impl Parser {
  pub(crate) fn parse_char_array(
    &mut self,
    original_string: Option<*mut AstArray<c_char>>,
  ) -> Option<AstArray<c_char>> {
    let current_lexeme = *self.lexer.current();
    let current_type = current_lexeme.r#type;
    LUAU_ASSERT!(
      current_type == Type::QUOTED_STRING
        || current_type == Type::RAW_STRING
        || current_type == Type::INTERP_STRING_SIMPLE
    );

    let data_ptr = unsafe { current_lexeme.data.data };
    let length = current_lexeme.get_length() as usize;
    let bytes = unsafe { from_raw_parts(data_ptr as *const u8, length) };

    if let Some(original) = original_string {
      unsafe {
        *original = self.copy_bytes(bytes);
      }
    }

    let mut data = bytes.to_vec();

    if current_type == Type::QUOTED_STRING || current_type == Type::INTERP_STRING_SIMPLE {
      if !Lexer::fixup_quoted_bytes(&mut data) {
        self.next_lexeme();
        return None;
      }
    } else {
      Lexer::fixup_multiline_bytes(&mut data);
    }

    let value = self.copy_bytes(&data);
    self.next_lexeme();
    Some(value)
  }
}

pub fn parser_parse_char_array(
  this: &mut Parser,
  original_string: Option<*mut AstArray<c_char>>,
) -> Option<AstArray<c_char>> {
  this.parse_char_array(original_string)
}
