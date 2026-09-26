use ulua_common::macros::luau_noinline::LUAU_NOINLINE;

use crate::{
  enums::type_lexer::Type,
  records::{lexeme::Lexeme, parser::Parser},
};

impl Parser {
  LUAU_NOINLINE! {
      pub fn expect_and_consume_fail(&mut self, type_: Type, context: &str) {
          let type_string = Lexeme::type_display_name(type_);
          // 一次取回当前词素的副本，location/to_string 共用，避免二次访问
          let current = *self.lexer.current();
          let curr_lexeme_string = current.to_string();

          if !context.is_empty() {
              self.report(
                  current.location,
                  format_args!(
                      "Expected {} when parsing {}, got {}",
                      type_string, context, curr_lexeme_string
                  ),
              );
          } else {
              self.report(
                  current.location,
                  format_args!("Expected {}, got {}", type_string, curr_lexeme_string),
              );
          }
      }
  }
}
