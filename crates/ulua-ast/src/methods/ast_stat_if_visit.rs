use crate::{
  methods::ast_stat_block_visit::ast_stat_block_visit,
  records::{ast_stat_if::AstStatIf, ast_visitor::AstVisitor},
  visit::{AstNodeRefMut, AstVisitable, ast_expr_visit_ref, ast_stat_visit_ref},
};

impl_visitable!(AstStatIf, StatIf, |this, visitor| {
  // condition/thenbody/elsebody 已句柄化（node_handle）：可变借用沿 `&mut self`
  // 逐级传递，elsebody 的 null 短路折叠进 `get_mut` 的 Option，全链路无裸指针。
  ast_expr_visit_ref(this.condition.get_mut(), visitor);
  ast_stat_block_visit(this.thenbody.get_mut(), visitor);
  if let Some(elsebody) = this.elsebody.get_mut() {
    ast_stat_visit_ref(elsebody, visitor);
  }
});
