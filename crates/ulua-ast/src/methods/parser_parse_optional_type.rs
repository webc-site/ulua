use core::ptr::NonNull;

use crate::{
  enums::type_lexer::Type,
  functions::optional_node::node_opt,
  records::{ast_type::AstType, parser::Parser},
};

impl Parser {
  /// cpp `Parser::parseOptionalType`（`Parser.cpp:2493`）：有 `:` 才解析类型。
  ///
  /// 返回 `None` 即 cpp 的 `nullptr`——「此处没有类型标注」，不是错误（`parseBinding`
  /// 用它填 `Binding::annotation`，`Parser.cpp:2424`；`Binding(Name, nullptr)` 是合法的
  /// 未标注形参）。
  pub fn parse_optional_type(&mut self) -> Option<NonNull<AstType>> {
    if self.lexer.current().r#type == Type::COLON {
      self.next_lexeme();
      node_opt(self.parse_type(false))
    } else {
      None
    }
  }
}
