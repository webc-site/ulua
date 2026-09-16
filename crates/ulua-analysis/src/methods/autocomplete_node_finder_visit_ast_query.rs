use ulua_ast::records::{ast_expr::AstExpr, ast_node::AstNode};

use crate::records::autocomplete_node_finder::AutocompleteNodeFinder;
impl AutocompleteNodeFinder {
  /// # Safety
  /// 调用方须保证 `expr` 等裸指针参数有效，且满足 C++ 原实现的调用契约。
  pub unsafe fn visit_ast_expr(&mut self, expr: *mut AstExpr) -> bool {
    let expr_ref = unsafe { &*expr };
    let location = expr_ref.base.location;
    if location.begin <= self.pos && self.pos <= location.end && location.begin != location.end {
      self.ancestry.push(expr as *mut AstNode);
      true
    } else {
      false
    }
  }
}
