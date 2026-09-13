use ulua_ast::records::ast_expr_global::AstExprGlobal;

use crate::{
  enums::global::Global::Written, functions::get_global_state::get_global_state,
  records::compiler::Compiler,
};

impl Compiler {
  /// # Safety
  /// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
  pub unsafe fn can_import(&self, expr: *mut AstExprGlobal) -> bool {
    if self.options.optimization_level < 1 {
      return false;
    }

    unsafe {
      let name = (*expr).name;
      get_global_state(&self.globals, name) != Written
    }
  }
}
