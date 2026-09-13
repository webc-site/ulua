use ulua_ast::records::ast_expr_local::AstExprLocal;
use ulua_config::enums::code::Code;

use crate::{functions::emit_warning::emit_warning, records::lint_global_local::LintGlobalLocal};
impl LintGlobalLocal {
  /// # Safety
  /// 调用方须保证 `node` 等裸指针参数有效，且满足 C++ 原实现的调用契约。
  pub(crate) fn visit_ast_expr_local(&mut self, node: *mut AstExprLocal) -> bool {
    let local = unsafe { (*node).local };
    if !local.is_null() && unsafe { (*local).name } == unsafe { (*self.context).placeholder } {
      emit_warning(
        unsafe { &mut *self.context },
        Code::PlaceholderRead,
        unsafe { (*node).base.base.location },
        format_args!("Placeholder value '_' is read here; consider using a named variable"),
      );
    }

    true
  }
}
