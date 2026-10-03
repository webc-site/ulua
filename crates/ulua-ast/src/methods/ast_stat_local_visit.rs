use crate::{
  functions::visit_vars_annotations::visit_vars_annotations,
  records::{ast_stat_local::AstStatLocal, ast_visitor::AstVisitor, node_handle::OptNode},
  visit::{AstNodeRefMut, AstVisitable, ast_expr_visit_ref},
};

impl_visitable!(AstStatLocal, StatLocal, |this, visitor| {
  // vars 注解下钻与 AstStatForIn 逐字相同，收口于 visit_vars_annotations
  // （cpp Ast.cpp:795-806 的判空跳过形态保持）。
  visit_vars_annotations(this.vars, visitor);

  for &expr_ptr in this.values.iter() {
    if let Some(expr) = OptNode::from_ptr(expr_ptr).get_mut() {
      ast_expr_visit_ref(expr, visitor);
    }
  }
});
