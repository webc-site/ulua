use crate::{
  records::{ast_stat_return::AstStatReturn, ast_visitor::AstVisitor},
  visit::{AstNodeRefMut, AstVisitable, ast_expr_visit},
};

impl_visitable!(AstStatReturn, StatReturn, |this, visitor| {
  for &expr in this.list.iter() {
    // Safety: list 元素是 parse_expr_list 写入 arena 的返回值表达式节点（可含 null 槽位，dispatch 内短路）；地址稳定，单线程独占遍历。
    unsafe {
      ast_expr_visit(expr, visitor);
    }
  }
});
