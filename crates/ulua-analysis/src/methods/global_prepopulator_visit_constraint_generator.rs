use ulua_ast::records::{ast_expr::AstExpr, ast_expr_global::AstExprGlobal};

use crate::records::{global_prepopulator::GlobalPrepopulator, symbol::Symbol};

impl GlobalPrepopulator {
  /// # Safety
  /// 调用方须保证 `global` 等裸指针参数有效，且满足 C++ 原实现的调用契约。
  pub(crate) fn visit_ast_expr_global(&mut self, global: *mut AstExprGlobal) -> bool {
    unsafe {
      let global_name = (*global).name;
      let scope = self.global_scope.as_mut();

      if let Some(ty) = scope.lookup_symbol(Symbol::from_global(global_name)) {
        let def = self.dfg.as_ref().get_def(global as *const AstExpr);
        *scope.lvalue_types.get_or_insert(def) = ty;
      }
    }

    true
  }
}
