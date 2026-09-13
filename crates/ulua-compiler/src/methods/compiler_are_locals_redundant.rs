use ulua_ast::records::ast_stat_local::AstStatLocal;

use crate::records::{compiler::Compiler, variable::Variable};

impl Compiler {
  /// # Safety
  /// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
  pub unsafe fn are_locals_redundant(&mut self, stat: *mut AstStatLocal) -> bool {
    unsafe {
      if stat.is_null() {
        return false;
      }

      let stat_ref = &*stat;

      // Extra expressions may have side effects
      if stat_ref.values.len() > stat_ref.vars.len() {
        return false;
      }

      for &local_ptr in stat_ref.vars.iter() {
        if local_ptr.is_null() {
          return false;
        }

        let local = &*local_ptr;

        if local.is_exported {
          // exported locals must be written to the export table
          return false;
        }

        let v_opt = self.variables.find(&local_ptr);

        let v_ptr = match v_opt {
          Some(ptr) => ptr,
          None => return false,
        };

        let v: &Variable = v_ptr;
        if !v.constant {
          return false;
        }
      }
    }

    true
  }
}
