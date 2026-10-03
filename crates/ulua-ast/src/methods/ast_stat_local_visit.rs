use crate::{
  records::{
    ast_stat_local::AstStatLocal, ast_visitor::AstVisitor, node_handle::OptNode,
  },
  visit::{AstNodeRefMut, AstVisitable, ast_expr_visit_ref, ast_type_visit_ref},
};

impl_visitable!(AstStatLocal, StatLocal, |this, visitor| {
  // vars/values 元素槽与 annotation 均为 parser 写入 arena 的裸指针（records
  // 波次）：null 折叠与解引用统一经 `OptNode` 句柄边界，调用点无 `unsafe`
  // （cpp Ast.cpp:795-806 的判空跳过形态逐字保持）。
  for &var_ptr in this.vars.iter() {
    if let Some(var) = OptNode::from_ptr(var_ptr).get() {
      if let Some(ty) = OptNode::from_ptr(var.annotation).get_mut() {
        ast_type_visit_ref(ty, visitor);
      }
    }
  }

  for &expr_ptr in this.values.iter() {
    if let Some(expr) = OptNode::from_ptr(expr_ptr).get_mut() {
      ast_expr_visit_ref(expr, visitor);
    }
  }
});
