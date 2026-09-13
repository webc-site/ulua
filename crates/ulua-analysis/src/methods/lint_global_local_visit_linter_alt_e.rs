use ulua_ast::{
  records::{ast_expr_global::AstExprGlobal, ast_stat_function::AstStatFunction},
  rtti::ast_node_is,
};
use ulua_config::enums::code::Code;

use crate::{functions::emit_warning::emit_warning, records::lint_global_local::LintGlobalLocal};
impl LintGlobalLocal {
  /// # Safety
  /// 调用方须保证 `node` 等裸指针参数有效，且满足 C++ 原实现的调用契约。
  pub(crate) fn visit_ast_stat_function(&mut self, node: *mut AstStatFunction) -> bool {
    let name = unsafe { (*node).name };
    if unsafe { ast_node_is::<AstExprGlobal>(&(*name).base) } {
      let gv = name as *mut AstExprGlobal;
      let g = self.globals.get_or_insert(unsafe { (*gv).name });

      if g.builtin {
        emit_warning(
          unsafe { &mut *self.context },
          Code::BuiltinGlobalWrite,
          unsafe { (*gv).base.base.location },
          format_args!(
            "Built-in global '{}' is overwritten here; consider using a local or changing the name",
            unsafe { (*gv).name }
          ),
        );
      } else {
        g.assigned = true;
        g.defined_as_function = true;
        g.defined_in_module_scope = self.function_stack.is_empty();
      }

      unsafe { self.track_global_ref(gv) };
    }

    true
  }
}
