use ulua_ast::records::ast_expr_global::AstExprGlobal;
use ulua_config::enums::code::Code;

use crate::{functions::emit_warning::emit_warning, records::lint_global_local::LintGlobalLocal};
impl LintGlobalLocal {
  /// # Safety
  /// 调用方须保证 `node` 等裸指针参数有效，且满足 C++ 原实现的调用契约。
  pub(crate) fn visit_ast_expr_global(&mut self, node: *mut AstExprGlobal) -> bool {
    if !self.function_stack.is_empty()
      && !self
        .function_stack
        .last()
        .unwrap()
        .dominated_globals
        .contains(unsafe { &(*node).name })
    {
      let g = self.globals.get_or_insert(unsafe { (*node).name });
      g.read_before_written = true;
    }

    unsafe { self.track_global_ref(node) };

    if unsafe { (*node).name } == unsafe { (*self.context).placeholder } {
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
