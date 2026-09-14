use core::ffi::c_void;

use ulua_ast::records::ast_stat_local::AstStatLocal;

use crate::records::cost_visitor::CostVisitor;

impl CostVisitor {
  pub fn visit_ast_stat_local(&mut self, node: *mut c_void) -> bool {
    unsafe {
      let node = &*(node as *mut AstStatLocal);

      for (i, &expr_ptr) in node.values.iter().enumerate() {
        let arg = self.model(expr_ptr);

        // C++ `i < vars.size()` 越界保护：values 可能多于 vars
        if arg.constant != 0 && i < node.vars.len() {
          let var_ptr = node.vars.as_slice()[i];
          self.vars.try_insert(var_ptr, arg.constant);
        }

        self.result.operator_add_assign(&arg);
      }
    }

    false
  }
}
