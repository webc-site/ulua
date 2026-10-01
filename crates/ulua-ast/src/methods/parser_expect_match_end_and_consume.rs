use ulua_common::macros::luau_noinline::LUAU_NOINLINE;

use crate::{
  enums::type_lexer::Type,
  records::{match_lexeme::MatchLexeme, parser::Parser},
};

impl Parser {
  LUAU_NOINLINE! {
      pub(crate) fn expect_match_end_and_consume(
          &mut self,
          type_: Type,
          begin: &MatchLexeme,
      ) -> bool {
          if self.lexer.current().r#type != type_ {
              return self.expect_match_end_and_consume_fail_with_lookahead(type_, begin);
          }

          // If the token matches on a different line and a different column, it suggests misleading indentation
          // This can be used to pinpoint the problem location for a possible future *actual* mismatch
          let current_loc = self.lexer.current().location;
          if current_loc.begin.line != begin.position.line
              && current_loc.begin.column != begin.position.column
          {
              // 无嫌疑记录、或既有嫌疑比本次 `begin` 更早（缩进更不可信）时改登记：
              // `is_none_or` 收拢旧的 Some/None 双分支同写
              if self.end_mismatch_suspect.is_none_or(|s| s.position.line < begin.position.line) {
                  self.end_mismatch_suspect = Some(*begin);
              }
          }

          self.next_lexeme();

          true
      }
  }
}
