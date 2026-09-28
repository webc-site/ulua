use crate::{
  records::{ast_stat_error::AstStatError, ast_visitor::AstVisitor},
  visit::{AstNodeRefMut, AstVisitable, ast_expr_visit, ast_stat_visit},
};

impl_visitable!(AstStatError, StatError, |this, visitor| {
  for &expression in this.expressions.iter() {
    // Safety: expressions 元素来自错误恢复路径以 arena 节点填充的槽位，null 由 dispatch 短路；地址稳定、单线程独占遍历。
    unsafe {
      ast_expr_visit(expression, visitor);
    }
  }

  for &statement in this.statements.iter() {
    // Safety: statements 元素同上（错误节点的语句列表由 parser 写入 arena）；ast_stat_visit 对 null 或存活节点的契约满足。
    unsafe {
      ast_stat_visit(statement, visitor);
    }
  }
});
