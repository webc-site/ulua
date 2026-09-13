use core::ptr::null_mut;

use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::records::{ast_type_or_pack::AstTypeOrPack, parser::Parser};

impl Parser {
  pub fn parse_simple_type_or_pack(&mut self) -> AstTypeOrPack {
    let old_recursion_count = self.recursion_counter;

    let begin = self.lexer.current().location;

    let result = self.parse_simple_type(true, false);

    if !result.type_pack.is_null() {
      LUAU_ASSERT!(result.r#type.is_null());
      return result;
    }

    self.recursion_counter = old_recursion_count;

    AstTypeOrPack {
      r#type: self.parse_type_suffix(result.r#type, &begin),
      type_pack: null_mut(),
    }
  }
}
