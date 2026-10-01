use crate::{
  records::{ast_stat_assign::AstStatAssign, ast_visitor::AstVisitor},
  visit::{AstNodeRefMut, AstVisitable, ast_expr_visit},
};

impl_visitable!(AstStatAssign, StatAssign, |this, visitor| {
  for &lvalue in this.vars.iter() {
    // Safety: vars 元素是 parser 校验过的左值表达式节点（arena 分配，地址稳定）；null 由 dispatch 短路，遍历期独占写穿。
    unsafe {
      ast_expr_visit(lvalue, visitor);
    }
  }

  for &expr in this.values.iter() {
    // Safety: values 元素为 parse_expr_list 写入 arena 的存活表达式节点；其余前提同上。
    unsafe {
      ast_expr_visit(expr, visitor);
    }
  }
});
