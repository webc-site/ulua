use ulua_ast::records::{ast_visitor::AstVisitor, position::Position};

use crate::records::{
  expr_or_local::ExprOrLocal, find_expr_or_local::FindExprOrLocal, source_module::SourceModule,
};
pub fn find_expr_or_local_at_position(source: &SourceModule, pos: Position) -> ExprOrLocal {
  let mut find_visitor = FindExprOrLocal::new(pos);
  // SAFETY: source.root 为 SourceModule 持有的存活根 AstStatBlock 指针，被指对象
  // 不属于 `source`（其仅存裸指针），故此可变借用不与 `&source` 冲突。
  find_visitor.visit_stat_block(unsafe { &mut *source.root });
  find_visitor.result
}
