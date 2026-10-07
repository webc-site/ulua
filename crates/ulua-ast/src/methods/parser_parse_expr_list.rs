use crate::{
  enums::type_lexer::Type,
  records::{
    ast_expr::AstExpr, node_handle::Node, parser::Parser, position::Position,
    temp_vector::TempVector,
  },
};

impl Parser {
  /// cpp `Parser::parseExprList(TempVector<AstExpr*>& result, ...)`：`result` 是
  /// 调用方 `scratch_expr` 的 TempVector 窗口，元素以 [`Node`] 入栈——`parse_expr`
  /// 的产物出自 arena 分配（失败即中止），恒非空由 `Node::from_raw` 单点判定。
  pub fn parse_expr_list(
    &mut self,
    result: &mut TempVector<'_, Node<AstExpr>>,
    mut comma_positions: Option<&mut TempVector<'_, Position>>,
  ) {
    result.push_back(Node::from_raw(self.parse_expr(0)));

    while self.lexer.current().r#type == Type::COMMA {
      if let Some(ref mut positions) = comma_positions {
        positions.push_back(self.lexer.current().location.begin);
      }

      self.next_lexeme();

      if self.lexer.current().r#type == Type::RPAREN {
        self.report(
          self.lexer.current().location,
          format_args!("Expected expression after ',' but got ')' instead"),
        );
        break;
      }

      result.push_back(Node::from_raw(self.parse_expr(0)));
    }
  }
}
