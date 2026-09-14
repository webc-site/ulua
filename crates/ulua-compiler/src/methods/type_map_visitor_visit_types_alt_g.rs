use ulua_ast::records::{ast_expr::AstExpr, ast_expr_local::AstExprLocal};
use ulua_common::enums::luau_bytecode_type::LBC_TYPE_ANY;

use crate::records::type_map_visitor::TypeMapVisitor;

impl<'a> TypeMapVisitor<'a> {
  /// # Safety
  /// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
  pub(crate) fn visit_ast_expr_local(&mut self, node: *mut AstExprLocal) -> bool {
    unsafe {
      if node.is_null() {
        return false;
      }

      let local = (*node).local;
      if local.is_null() {
        return false;
      }

      if !(*local).annotation.is_null() {
        let annotation = (*local).annotation;
        let ty = self.record_resolved_type_ast_expr_ast_type(node as *mut AstExpr, annotation);

        if ty != LBC_TYPE_ANY {
          self.local_types.try_insert(local, ty);
        }
      } else if let Some(type_ptr) = self.resolved_locals.find(&local) {
        let ty = self.record_resolved_type_ast_expr_ast_type(node as *mut AstExpr, *type_ptr);
        self.local_types.try_insert(local, ty);
      }

      false
    }
  }
}
