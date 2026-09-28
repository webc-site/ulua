use crate::{
  records::{ast_expr_call::AstExprCall, ast_visitor::AstVisitor},
  visit::{AstNodeRefMut, AstVisitable, ast_expr_visit},
};

impl_visitable!(AstExprCall, ExprCall, |this, visitor| {
  // Safety: func 与 args 元素均为 parser 写入 arena 的存活表达式节点（args 数组同 arena 分配）；arena 地址稳定，ast_expr_visit 对 null 短路，遍历期节点由 dispatch 独占写。
  unsafe {
    ast_expr_visit(this.func, visitor);

    for &arg in this.args.iter() {
      ast_expr_visit(arg, visitor);
    }
  }
});
