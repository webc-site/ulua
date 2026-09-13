use ulua_ast::{
  records::{
    ast_expr_global::AstExprGlobal, ast_expr_local::AstExprLocal, ast_stat_assign::AstStatAssign,
  },
  rtti::ast_node_is,
  visit::ast_expr_visit,
};
use ulua_config::enums::code::Code;

use crate::{functions::emit_warning::emit_warning, records::lint_global_local::LintGlobalLocal};
impl LintGlobalLocal {
  /// # Safety
  /// 调用方须保证 `node` 等裸指针参数有效，且满足 C++ 原实现的调用契约。
  pub(crate) fn visit_ast_stat_assign(&mut self, node: *mut AstStatAssign) -> bool {
    let vars = unsafe { (*node).vars };
    for &var in vars.as_slice() {
      if unsafe { ast_node_is::<AstExprGlobal>(&(*var).base) } {
        let gv = var as *mut AstExprGlobal;
        let g = self.globals.get_or_insert(unsafe { (*gv).name });

        if self.function_stack.is_empty() {
          g.defined_in_module_scope = true;
        } else if !self.function_stack.last().unwrap().conditional_execution {
          self
            .function_stack
            .last_mut()
            .unwrap()
            .dominated_globals
            .insert(unsafe { (*gv).name });
        }

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
        }

        unsafe { self.track_global_ref(gv) };
      } else if unsafe { ast_node_is::<AstExprLocal>(&(*var).base) } {
        // Local writes are not local reads.
      } else {
        unsafe {
          ast_expr_visit(var, self);
        }
      }
    }

    let values = unsafe { (*node).values };
    for &val in values.as_slice() {
      unsafe {
        ast_expr_visit(val, self);
      }
    }

    false
  }
}
