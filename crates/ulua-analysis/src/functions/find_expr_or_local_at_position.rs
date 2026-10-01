use ulua_ast::records::{ast_visitor::AstVisitor, position::Position};

use crate::records::{
  arena_handle::alias, expr_or_local::ExprOrLocal, find_expr_or_local::FindExprOrLocal,
  source_module::SourceModule,
};
pub fn find_expr_or_local_at_position(source: &SourceModule, pos: Position) -> ExprOrLocal {
  let mut find_visitor = FindExprOrLocal::new(pos);
  find_visitor.visit_stat_block(alias(source.root));
  find_visitor.result
}
