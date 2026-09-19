use crate::records::{
  ast_expr::AstExpr, ast_expr_type_assertion::AstExprTypeAssertion,
  cst_expr_type_assertion::CstExprTypeAssertion, lexeme::Type, location::Location, parser::Parser,
};

impl Parser {
  pub fn parse_assertion_expr(&mut self) -> *mut AstExpr {
    let start = self.lexer.current().location;
    let expr = self.parse_simple_expr();

    if self.lexer.current().r#type == Type::DOUBLE_COLON {
      let op_position = self.lexer.current().location.begin;
      self.next_lexeme();
      let annotation = self.parse_type(false);

      let node = unsafe {
        (*self.allocator).alloc(AstExprTypeAssertion::new(
          Location::new(start.begin, (*annotation).base.location.end),
          expr,
          annotation,
        ))
      };

      self.attach_cst(node, |alloc| {
        alloc.alloc(CstExprTypeAssertion::new(op_position))
      });

      node as *mut AstExpr
    } else {
      expr
    }
  }
}
