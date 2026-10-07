use crate::records::{
  ast_expr::AstExpr, ast_expr_index_expr::AstExprIndexExpr, cst_expr_index_expr::CstExprIndexExpr,
  location::Location, match_lexeme::MatchLexeme, node_handle::Node, parser::Parser,
  position::Position,
};

impl Parser {
  pub(crate) fn parse_index_expr(&mut self, start: Position, expr: *mut AstExpr) -> *mut AstExpr {
    let match_bracket = MatchLexeme::new(self.lexer.current());
    self.next_lexeme();

    let index = self.parse_expr(0);

    let close_bracket_position = self.lexer.current().location.begin;
    let end = self.lexer.current().location.end;

    let closing_bracket_found = self.expect_match_and_consume(']', &match_bracket, true);

    // expr/index 出自 parse_expr 线的 arena 分配（cpp 语义错误路径亦返回节点，恒非空）。
    let expr = self.alloc_expr(AstExprIndexExpr::new(
      Location::new(start, end),
      Node::from_raw(expr),
      Node::from_raw(index),
    ));

    self.attach_cst(expr, |alloc| {
      alloc.alloc(CstExprIndexExpr::new(
        match_bracket.position,
        if closing_bracket_found {
          close_bracket_position
        } else {
          Position::missing()
        },
      ))
    });

    expr
  }
}
