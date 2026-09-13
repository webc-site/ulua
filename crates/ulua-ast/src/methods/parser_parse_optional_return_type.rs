use std::ptr::null_mut;

use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::records::{
  ast_type_pack::AstTypePack, lexeme::Type, parser::Parser, position::Position,
};

impl Parser {
  pub fn parse_optional_return_type(
    &mut self,
    return_specifier_position: Option<&mut Position>,
  ) -> *mut AstTypePack {
    let curr = *self.lexer.current();

    if curr.r#type == Type(':' as i32) || curr.r#type == Type::SKINNY_ARROW {
      if curr.r#type == Type::SKINNY_ARROW {
        self.report(
          curr.location,
          format_args!("Function return type annotations are written after ':' instead of '->'"),
        );
      }

      if let Some(pos) = return_specifier_position {
        *pos = curr.location.begin;
      }

      self.next_lexeme();

      let old_recursion_count = self.recursion_counter;

      let result = self.parse_return_type();
      LUAU_ASSERT!(!result.is_null());

      // At this point, if we find a , character, it indicates that there are multiple return types
      // in this type annotation, but the list wasn't wrapped in parentheses.
      if self.lexer.current().r#type == Type(',' as i32) {
        self.report(
                    self.lexer.current().location,
                    format_args!(
                        "Expected a statement, got ','; did you forget to wrap the list of return types in parentheses?"
                    ),
                );

        self.next_lexeme();
      }

      self.recursion_counter = old_recursion_count;

      return result;
    }

    null_mut()
  }
}
