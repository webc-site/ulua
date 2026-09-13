use core::ffi::c_void;

use ulua_ast::records::ast_stat_assign::AstStatAssign;

use crate::records::{cost::Cost, cost_visitor::CostVisitor};

impl CostVisitor {
  pub fn visit_ast_stat_assign(&mut self, node: *mut c_void) -> bool {
    unsafe {
      let node = &*(node as *mut AstStatAssign);

      for &var_ptr in node.vars.iter() {
        self.assign(var_ptr);
      }

      for i in 0..node.vars.len().max(node.values.len()) {
        let mut ac = Cost::default();
        if let Some(&var_ptr) = node.vars.as_slice().get(i) {
          ac = ac.add(&self.model(var_ptr));
        }
        if let Some(&val_ptr) = node.values.as_slice().get(i) {
          ac = ac.add(&self.model(val_ptr));
        }

        // local->local or constant->local assignment is not free
        if ac.model == 0 {
          self.result.operator_add_assign(&Cost::new(1, 0));
        } else {
          self.result.operator_add_assign(&ac);
        }
      }
    }

    false
  }
}
