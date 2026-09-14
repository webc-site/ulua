use core::ptr::null_mut;

use crate::records::{ast_type::AstType, lexeme::Type, parser::Parser};

impl Parser {
  pub fn parse_type_bool(&mut self, in_declaration_context: bool) -> *mut AstType {
    let old_recursion_count = self.recursion_counter;

    let begin = self.lexer.current().location;

    let c = self.lexer.current().r#type;
    let type_ = if c != Type('|' as i32) && c != Type('&' as i32) {
      let result = self.parse_simple_type(false, in_declaration_context);
      self.recursion_counter = old_recursion_count;
      result.r#type
    } else {
      null_mut()
    };

    let type_with_suffix = self.parse_type_suffix(type_, &begin);
    self.recursion_counter = old_recursion_count;

    type_with_suffix
  }
}
