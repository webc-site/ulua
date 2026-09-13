use core::ptr::null_mut;

use ulua_ast::records::{
  ast_expr_call::AstExprCall, ast_expr_group::AstExprGroup, ast_expr_index_expr::AstExprIndexExpr,
  ast_expr_index_name::AstExprIndexName, ast_expr_local::AstExprLocal,
  ast_expr_type_assertion::AstExprTypeAssertion, ast_node::AstNode, ast_type_group::AstTypeGroup,
  ast_type_typeof::AstTypeTypeof,
};

use crate::records::require_tracer::RequireTracer;
impl RequireTracer {
  /// # Safety
  /// 调用方须保证 `node` 等裸指针参数有效，且满足 C++ 原实现的调用契约。
  pub unsafe fn get_dependent(&self, node: *mut AstNode) -> *mut AstNode {
    unsafe {
      if (*node).is::<AstExprLocal>() {
        let expr = node as *mut AstExprLocal;
        let local = (*expr).local;
        match self.locals.find(&local) {
          Some(&val) => return val as *mut AstNode,
          None => return null_mut(),
        }
      } else if (*node).is::<AstExprIndexName>() {
        let expr = node as *mut AstExprIndexName;
        return (*expr).expr as *mut AstNode;
      } else if (*node).is::<AstExprIndexExpr>() {
        let expr = node as *mut AstExprIndexExpr;
        return (*expr).expr as *mut AstNode;
      } else if (*node).is::<AstExprCall>() {
        let expr = node as *mut AstExprCall;
        if (*expr).self_ {
          let func = (*expr).func as *mut AstExprIndexName;
          return (*func).expr as *mut AstNode;
        }
      } else if (*node).is::<AstExprGroup>() {
        let expr = node as *mut AstExprGroup;
        return (*expr).expr as *mut AstNode;
      } else if (*node).is::<AstExprTypeAssertion>() {
        let expr = node as *mut AstExprTypeAssertion;
        return (*expr).annotation as *mut AstNode;
      } else if (*node).is::<AstTypeGroup>() {
        let expr = node as *mut AstTypeGroup;
        return (*expr).type_ as *mut AstNode;
      } else if (*node).is::<AstTypeTypeof>() {
        let expr = node as *mut AstTypeTypeof;
        return (*expr).expr as *mut AstNode;
      }
    }
    null_mut()
  }
}
