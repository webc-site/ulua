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

      // 单次遍历实现 zip_longest 语义：任一侧耗尽即视为 Cost::default()
      // （全零加法单位元）；求值顺序 `vars[i]` 先于 `values[i]`，对齐 C++
      let mut vars = node.vars.iter();
      let mut values = node.values.iter();

      loop {
        let ac = match (vars.next(), values.next()) {
          (Some(&v), Some(&val)) => self.model(v).add(&self.model(val)),
          (Some(&v), None) => self.model(v),
          (None, Some(&val)) => self.model(val),
          (None, None) => break,
        };

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
