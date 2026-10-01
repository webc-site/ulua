use core::ptr::NonNull;

use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::{
  enums::type_lexer::Type,
  records::{ast_type_pack::AstTypePack, parser::Parser, position::Position},
};

impl Parser {
  /// cpp `Parser::parseOptionalReturnType`（`Parser.cpp:2561`）：`:`/`->` 在位才解析返回标注。
  ///
  /// 返回 `None` 即 cpp 的 `nullptr` == 「函数没有返回类型标注」，是正常形态而非错误：
  /// 调用方要么补一个空 `AstTypePackExplicit`（`Parser.cpp:1846`），要么直接写进可空的
  /// `AstExprFunction::returnAnnotation`（`Parser.cpp:2339` 的 `typelist`）。
  pub fn parse_optional_return_type(
    &mut self,
    return_specifier_position: Option<&mut Position>,
  ) -> Option<NonNull<AstTypePack>> {
    let curr = *self.lexer.current();

    if curr.r#type == Type::COLON || curr.r#type == Type::SKINNY_ARROW {
      if curr.r#type == Type::SKINNY_ARROW {
        self.report(
          curr.location,
          format_args!("Function return type annotations are written after ':' instead of '->'"),
        );
      }

      if let Some(pos) = return_specifier_position {
        *pos = curr.location.begin;
      }

      self.next_lexeme();

      let old_recursion_count = self.recursion_counter;

      let result = self.parse_return_type();
      LUAU_ASSERT!(result.is_some());

      // At this point, if we find a , character, it indicates that there are multiple return types
      // in this type annotation, but the list wasn't wrapped in parentheses.
      if self.lexer.current().r#type == Type::COMMA {
        self.report(
                    self.lexer.current().location,
                    format_args!(
                        "Expected a statement, got ','; did you forget to wrap the list of return types in parentheses?"
                    ),
                );

        self.next_lexeme();
      }

      self.recursion_counter = old_recursion_count;

      return result;
    }

    None
  }
}
