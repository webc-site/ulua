use core::ptr::null_mut;

use crate::records::{ast_type::AstType, lexeme::Type, parser::Parser};

impl Parser {
  pub fn parse_optional_type(&mut self) -> *mut AstType {
    if self.lexer.current().r#type == Type(':' as i32) {
      self.next_lexeme();
      self.parse_type_bool(false)
    } else {
      null_mut()
    }
  }
}
