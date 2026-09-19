use core::ptr::NonNull;

use crate::{
  enums::type_lexer::Type,
  records::{ast_type::AstType, parser::Parser},
};

impl Parser {
  pub fn parse_type(&mut self, in_declaration_context: bool) -> *mut AstType {
    let old_recursion_count = self.recursion_counter;

    let begin = self.lexer.current().location;

    let c = self.lexer.current().r#type;
    // 前导 `|` / `&` 时无已解析类型，`None` 对应 cpp 的 `nullptr`。
    let type_ = if c != Type('|' as i32) && c != Type('&' as i32) {
      let result = self.parse_simple_type(false, in_declaration_context);
      self.recursion_counter = old_recursion_count;
      NonNull::new(result.r#type)
    } else {
      None
    };

    let type_with_suffix = self.parse_type_suffix(type_, &begin);
    self.recursion_counter = old_recursion_count;

    type_with_suffix
  }
}
