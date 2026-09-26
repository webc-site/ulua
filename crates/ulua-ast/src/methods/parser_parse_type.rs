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
    let type_ = if c != Type::PIPE && c != Type::AMPERSAND {
      let result = self.parse_simple_type(false, in_declaration_context);
      self.recursion_counter = old_recursion_count;
      // allowPack=false 时只可能是 `Type`；`None` 与 cpp 读到 null 哨兵同形，
      // 交由 parse_type_suffix 按「无前导类型」处理。
      result.as_type().map(NonNull::from)
    } else {
      None
    };

    let type_with_suffix = self.parse_type_suffix(type_, &begin);
    self.recursion_counter = old_recursion_count;

    type_with_suffix
  }
}
