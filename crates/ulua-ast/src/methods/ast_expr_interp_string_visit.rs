use crate::{
  records::{ast_expr_interp_string::AstExprInterpString, ast_visitor::AstVisitor},
  visit::{AstNodeRefMut, AstVisitable, ast_expr_visit},
};

impl_visitable!(AstExprInterpString, ExprInterpString, |this, visitor| {
  for &expr in this.expressions.iter() {
    // Safety: expressions 元素由 parse_string_with_interpolation 逐个以 arena 节点填充，null 或存活均满足 ast_expr_visit 契约；dispatch 在独占遍历中写穿。
    unsafe {
      ast_expr_visit(expr, visitor);
    }
  }
});
