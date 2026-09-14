use alloc::vec::Vec;

use ulua_ast::records::ast_expr_global::AstExprGlobal;

use crate::records::lint_global_local::LintGlobalLocal;
impl LintGlobalLocal {
  /// # Safety
  /// 调用方须保证 `node` 等裸指针参数有效，且满足 C++ 原实现的调用契约。
  pub unsafe fn track_global_ref(&mut self, node: *mut AstExprGlobal) {
    let current_function_refs = self
      .function_stack
      .iter()
      .map(|entry| entry.ast)
      .collect::<Vec<_>>();

    self.global_refs.push(node);

    let g = self.globals.get_or_insert(unsafe { (*node).name });

    if g.first_ref.is_null() {
      g.first_ref = node;

      if !g.builtin {
        g.function_ref.clear();
        g.function_ref.reserve(current_function_refs.len());
        g.function_ref.extend(current_function_refs);
      }
    } else if !g.builtin {
      let mut prefix = 0;

      while prefix < g.function_ref.len()
        && prefix < current_function_refs.len()
        && g.function_ref[prefix] == current_function_refs[prefix]
      {
        prefix += 1;
      }

      g.function_ref.truncate(prefix);
    }
  }
}
