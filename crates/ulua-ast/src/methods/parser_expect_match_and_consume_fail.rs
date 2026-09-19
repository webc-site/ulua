use ulua_common::macros::luau_noinline::LUAU_NOINLINE;

use crate::records::{
  lexeme::{Lexeme, Type},
  match_lexeme::MatchLexeme,
  parser::Parser,
};

impl Parser {
  LUAU_NOINLINE! {
      pub fn expect_match_and_consume_fail(
          &mut self,
          type_: Type,
          begin: &MatchLexeme,
          extra: Option<&str>,
      ) {
          let type_string = Lexeme::type_display_name(type_);
          let match_string = Lexeme::type_display_name(begin.type_);

          let current_lexeme = *self.lexer.current();
          let extra_str = extra.unwrap_or("");

          if current_lexeme.location.begin.line == begin.position.line {
              self.report(
                  current_lexeme.location,
                  format_args!(
                      "Expected {} (to close {} at column {}), got {}{}",
                      type_string,
                      match_string,
                      begin.position.column + 1,
                      current_lexeme,
                      extra_str
                  ),
              );
          } else {
              self.report(
                  current_lexeme.location,
                  format_args!(
                      "Expected {} (to close {} at line {}), got {}{}",
                      type_string,
                      match_string,
                      begin.position.line + 1,
                      current_lexeme,
                      extra_str
                  ),
              );
          }
      }
  }
}
