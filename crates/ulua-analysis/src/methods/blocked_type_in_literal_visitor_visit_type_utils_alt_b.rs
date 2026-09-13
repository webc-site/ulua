use ulua_ast::{
  records::{ast_expr::AstExpr, ast_expr_group::AstExprGroup},
  rtti::ast_node_is,
};

use crate::{
  functions::{follow_type::follow_type_id, get_type_alt_j::get_type_id, is_literal::is_literal},
  records::{
    blocked_type::BlockedType, blocked_type_in_literal_visitor::BlockedTypeInLiteralVisitor,
  },
};

impl BlockedTypeInLiteralVisitor {
  /// # Safety
  /// 调用方须保证 `e` 等裸指针参数有效，且满足 C++ 原实现的调用契约。
  pub unsafe fn visit_ast_expr(&mut self, e: *mut AstExpr) -> bool {
    unsafe {
      if let Some(&ty) = (*self.ast_types).find(&(e as *const AstExpr)) {
        let followed = follow_type_id(ty);
        if !get_type_id::<BlockedType>(followed).is_none() {
          (*self.to_block).push(ty);
        }
      }
    }
    is_literal(e as *const AstExpr) || unsafe { ast_node_is::<AstExprGroup>(&(*e).base) }
  }
}
