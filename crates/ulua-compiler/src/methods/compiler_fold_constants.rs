use core::ptr::null_mut;

use ulua_ast::records::ast_node::AstNode;

use crate::{
  functions::fold_constants::{FoldConstantsArgs, fold_constants},
  records::compiler::Compiler,
};

impl Compiler {
  /// # Safety
  /// 传入的指针必须有效且指向存活对象。
  pub unsafe fn fold_constants(&mut self, root: *mut AstNode, record_changes: bool) {
    let expr_change_log = if record_changes {
      &mut self.expr_changes as *mut _
    } else {
      null_mut()
    };
    let local_change_log = if record_changes {
      &mut self.local_changes as *mut _
    } else {
      null_mut()
    };

    unsafe {
      fold_constants(
        root,
        FoldConstantsArgs {
          constants: &mut self.constants,
          variables: &mut self.variables,
          locals: &mut self.locstants,
          builtins: self.builtins_fold,
          fold_library_k: self.builtins_fold_library_k,
          library_member_constant_cb: self.options.library_member_constant_cb,
          string_table: &mut *self.names,
          table_constants: &self.table_constants,
          expr_change_log,
          local_change_log,
        },
      );
    }
  }
}
