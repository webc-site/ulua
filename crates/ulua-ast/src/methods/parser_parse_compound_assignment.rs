use crate::{
  functions::is_expr_l_value::is_expr_l_value,
  records::{
    ast_expr::AstExpr, ast_expr_binary::AstExprBinaryOp, ast_node::AstNode, ast_stat::AstStat,
    ast_stat_compound_assign::AstStatCompoundAssign, cst_node::CstNode,
    cst_stat_compound_assign::CstStatCompoundAssign, location::Location, parser::Parser,
  },
};

impl Parser {
  pub(crate) fn parse_compound_assignment(
    &mut self,
    mut initial: *mut AstExpr,
    op: AstExprBinaryOp,
  ) -> *mut AstStat {
    // C++ REASSIGNS `initial` to an error node and FALLS THROUGH to
    // nextLexeme()+parseExpr() — it does NOT return here. The port returned
    // early, so when the LHS is not an lvalue the operator (`+=` etc.) was
    // never consumed; parseBlockNoScope's statement loop then re-parsed the
    // same token forever, allocating until the process OOMs (machine crash).
    if !is_expr_l_value(initial) {
      initial = self.report_l_value_error(initial) as *mut AstExpr;
    }

    let op_position = self.lexer.current().location.begin;
    self.next_lexeme();

    let value = self.parse_expr_i32(0);

    let node = unsafe {
      (*self.allocator).alloc(AstStatCompoundAssign::new(
        Location::new((*initial).base.location.begin, (*value).base.location.end),
        op,
        initial,
        value,
      ))
    };

    if self.options.store_cst_data {
      let cst_node = unsafe { (*self.allocator).alloc(CstStatCompoundAssign::new(op_position)) };
      self
        .cst_node_map
        .try_insert(node as *mut AstNode, cst_node as *mut CstNode);
    }

    node as *mut AstStat
  }
}
