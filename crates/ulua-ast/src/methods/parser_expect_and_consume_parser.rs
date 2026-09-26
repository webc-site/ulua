use crate::{enums::type_lexer::Type, records::parser::Parser};

impl Parser {
  pub fn expect_and_consume_char(&mut self, value: char, context: &str) -> bool {
    let type_ = Type(value as i32);
    self.expect_and_consume_type(type_, context)
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
