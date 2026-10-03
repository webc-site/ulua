use crate::{
  functions::visit_vars_annotations::visit_vars_annotations,
  methods::ast_stat_block_visit::ast_stat_block_visit,
  records::{ast_stat_for_in::AstStatForIn, ast_visitor::AstVisitor, node_handle::OptNode},
  visit::{AstNodeRefMut, AstVisitable, ast_expr_visit_ref},
};

impl_visitable!(AstStatForIn, StatForIn, |this, visitor| {
  // vars 注解下钻与 AstStatLocal 逐字相同，收口于 visit_vars_annotations。
  visit_vars_annotations(this.vars, visitor);

  for &expr in this.values.iter() {
    if let Some(e) = OptNode::from_ptr(expr).get_mut() {
      ast_expr_visit_ref(e, visitor);
    }
  }

  // body 已句柄化为 Node：可变借用沿 &mut self 传递，dispatch 走 safe 引用
  // 形态（cpp Ast.cpp:861 无守卫下钻同序）。
  ast_stat_block_visit(this.body.get_mut(), visitor);
});
