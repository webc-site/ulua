use crate::{
  enums::type_lexer::Type,
  records::{parser::Parser, position::Position},
};

impl Parser {
  pub fn expect_and_consume_char(&mut self, value: char, context: &str) -> bool {
    let type_ = Type(value as i32);
    self.expect_and_consume_type(type_, context)
  }

  /// cpp `expectAndConsumeChar` + `getPosition` 惯用法的单源：消费成功取
  /// 消费后词元的起始位（`previous_location` 即被消费词元），失败取
  /// [`Position::missing`]。九处 CST 记位样板（':'/'='/']' 等）收口于此。
  pub(crate) fn expect_and_consume_char_position(
    &mut self,
    value: char,
    context: &str,
  ) -> Position {
    if self.expect_and_consume_char(value, context) {
      self.lexer.previous_location().begin
    } else {
      Position::missing()
    }
  }
}

impl Parser {
  pub fn expect_and_consume_type(&mut self, type_: Type, context: &str) -> bool {
    if self.lexer.current().r#type != type_ {
      self.expect_and_consume_fail_with_lookahead(type_, context);

      false
    } else {
      self.next_lexeme();

      true
    }
  }
}
