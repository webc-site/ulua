use ulua_ast::records::{ast_visitor::AstVisitor, position::Position};

use crate::records::{
  expr_or_local::ExprOrLocal, find_expr_or_local::FindExprOrLocal, source_module::SourceModule,
};
pub fn find_expr_or_local_at_position(source: &SourceModule, pos: Position) -> ExprOrLocal {
  let mut find_visitor = FindExprOrLocal::new(pos);
  // cpp `findVisitor.visit(source.root)`：根块在场为调用契约，句柄物化
  // 独占借用（visitor 需要非 const `this` 遍历）。
  let root = source
    .root
    .expect("findExprOrLocalAtPosition: 根块应在场（cpp 直取 source.root）");
  find_visitor.visit_stat_block(root.get_mut());
  find_visitor.result
}
