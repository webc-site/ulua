use ulua_ast::records::ast_stat_local::AstStatLocal;

use crate::records::lint_local_hygiene::LintLocalHygiene;

impl LintLocalHygiene {
  /// # Safety
  /// 调用方须保证 `node` 等裸指针参数有效，且满足 C++ 原实现的调用契约。
  pub(crate) fn visit_ast_stat_local(&mut self, node: *mut AstStatLocal) -> bool {
    let vars = unsafe { (*node).vars };
    let values = unsafe { (*node).values };

    if vars.len() == 1 && values.len() == 1 {
      let local = vars.as_slice()[0];
      let value = values.as_slice()[0];
      let is_import = self.is_require_call(value);

      {
        let info = self.locals.get_or_insert(local);
        info.defined = node.cast();
        info.import = is_import;
      }

      if is_import {
        *self.imports.get_or_insert(unsafe { (*local).name }) = local;
      }
    } else {
      for &local in vars.as_slice() {
        let info = self.locals.get_or_insert(local);
        info.defined = node.cast();
      }
    }

    true
  }
}
