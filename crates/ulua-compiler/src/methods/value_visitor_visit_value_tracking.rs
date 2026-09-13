use core::ptr::null_mut;

use ulua_ast::records::ast_stat_local::AstStatLocal;

use crate::records::value_visitor::ValueVisitor;

impl ValueVisitor {
  /// # Safety
  /// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
  pub(crate) fn visit_ast_stat_local(&mut self, node: *mut AstStatLocal) -> bool {
    if node.is_null() {
      return false;
    }

    unsafe {
      let node_ref = &*node;

      let values_len = node_ref.values.len();

      // C++ `variables[vars[i]].init = values[i]`: operator[] creates the entry if
      // absent (default written/constant), but only overwrites `.init` if it exists.
      // zip 取较短一侧，等价于 `min(vars.len, values.len)` 循环。
      for (&var_ptr, &init_ptr) in node_ref.vars.iter().zip(node_ref.values.iter()) {
        self.variables.get_or_insert(var_ptr).init = init_ptr;
      }

      for &var_ptr in node_ref.vars.iter().skip(values_len) {
        self.variables.get_or_insert(var_ptr).init = null_mut();
      }
    }

    true
  }
}
