use crate::{
  enums::type_lexer::Type,
  records::{match_lexeme::MatchLexeme, parser::Parser, position::Position},
};

impl Parser {
  pub fn expect_match_and_consume(
    &mut self,
    value: char,
    begin: &MatchLexeme,
    search_for_missing: bool,
  ) -> bool {
    let r#type = Type(value as i32);

    if self.lexer.current().r#type != r#type {
      self.expect_match_and_consume_fail(r#type, begin, None);

      self.expect_match_and_consume_recover(value, begin, search_for_missing)
    } else {
      self.next_lexeme();

      true
    }
  }

  /// 匹配版记位（契约同
  /// [`Parser::expect_and_consume_char_position`](crate::records::parser::Parser::expect_and_consume_char_position)）：
  /// 消费成功取消费后词元起始位，失败取 [`Position::missing`]。
  pub(crate) fn expect_match_and_consume_position(
    &mut self,
    value: char,
    begin: &MatchLexeme,
    search_for_missing: bool,
  ) -> Position {
    if self.expect_match_and_consume(value, begin, search_for_missing) {
      self.lexer.previous_location().begin
    } else {
      Position::missing()
    }
  }
}
