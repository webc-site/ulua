use ulua_ast::{
  records::{
    ast_expr_global::AstExprGlobal, ast_expr_index_name::AstExprIndexName, ast_node::AstNode,
  },
  rtti::ast_node_as,
};

use crate::records::lint_deprecated_api::LintDeprecatedApi;

impl LintDeprecatedApi {
  /// # Safety
  /// 调用方须保证 `node` 等裸指针参数有效，且满足 C++ 原实现的调用契约。
  pub(crate) fn visit_ast_expr_index_name(&mut self, node: *mut AstExprIndexName) -> bool {
    unsafe {
      if let Some(ty) = (*self.context).get_type((*node).expr) {
        self.check_ast_expr_index_name_type_id(&*node, ty);
      } else {
        let global = ast_node_as::<AstExprGlobal>((*node).expr as *mut AstNode);
        if let Some(global) = global.as_ref() {
          self.check_location_ast_name_ast_name(
            &(*node).base.base.location,
            global.name,
            (*node).index,
          );
        }
      }
    }

    true
  }
}
