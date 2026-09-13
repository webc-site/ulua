use crate::records::{
  ast_expr::AstExpr, ast_expr_if_else::AstExprIfElse, ast_node::AstNode,
  cst_expr_if_else::CstExprIfElse, cst_node::CstNode, lexeme::Type, location::Location,
  parser::Parser, position::Position,
};

impl Parser {
  pub(crate) fn parse_if_else_expr(&mut self) -> *mut AstExpr {
    let start = self.lexer.current().location;

    self.next_lexeme();

    let condition = self.parse_expr_i32(0);

    let has_then = self.expect_and_consume_type(Type::RESERVED_THEN, "if then else expression");
    let then_position = if has_then {
      self.lexer.previous_location().begin
    } else {
      Position::missing()
    };

    let true_expr = self.parse_expr_i32(0);

    let else_position = self.lexer.current().location.begin;

    let (has_else, false_expr, is_else_if) = if self.lexer.current().r#type == Type::RESERVED_ELSEIF
    {
      let old_recursion_count = self.recursion_counter;
      self.increment_recursion_counter("expression");
      let false_expr = self.parse_if_else_expr();
      self.recursion_counter = old_recursion_count;
      (true, false_expr, true)
    } else {
      let has_else = self.expect_and_consume_type(Type::RESERVED_ELSE, "if then else expression");
      let false_expr = self.parse_expr_i32(0);
      (has_else, false_expr, false)
    };

    let end = unsafe { (*false_expr).base.location };

    let node = unsafe {
      (*self.allocator).alloc(AstExprIfElse::new(
        Location::new(start.begin, end.end),
        condition,
        has_then,
        true_expr,
        has_else,
        false_expr,
      ))
    };

    if self.options.store_cst_data {
      let cst_node = unsafe {
        (*self.allocator).alloc(CstExprIfElse::new(then_position, else_position, is_else_if))
      };
      self
        .cst_node_map
        .try_insert(node as *mut AstNode, cst_node as *mut CstNode);
    }

    node as *mut AstExpr
  }
}
