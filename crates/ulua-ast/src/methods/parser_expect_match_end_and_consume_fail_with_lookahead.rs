use ulua_common::macros::luau_noinline::LUAU_NOINLINE;

use crate::{
  enums::type_lexer::Type,
  records::{lexeme::Lexeme, match_lexeme::MatchLexeme, parser::Parser},
};

impl Parser {
  LUAU_NOINLINE! {
      pub(crate) fn expect_match_end_and_consume_fail_with_lookahead(
          &mut self,
          type_: Type,
          begin: &MatchLexeme,
      ) -> bool {
          // 错配嫌疑存在、非 EOF、且比 `begin` 更靠后时，才附加"是否忘了关闭"提示；
          // 三重嵌套的 if/else 收敛为 filter + map，三种 None 路径语义不变
          let suggestion = self
              .end_mismatch_suspect
              .filter(|suspect| {
                  suspect.type_ != Type::EOF && suspect.position.line > begin.position.line
              })
              .map(|suspect| {
                  format!(
                      "; did you forget to close {} at line {}?",
                      Lexeme::type_display_name(suspect.type_),
                      suspect.position.line + 1
                  )
              });

          self.expect_match_and_consume_fail(type_, begin, suggestion.as_deref());

          // check if this is an extra token and the expected token is next
          if self.lexer.lookahead().r#type == type_ {
              // skip invalid and consume expected
              self.next_lexeme();
              self.next_lexeme();

              return true;
          }

          false
      }
  }
}
