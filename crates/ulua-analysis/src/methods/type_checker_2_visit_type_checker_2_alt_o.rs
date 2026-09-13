use ulua_ast::records::ast_stat_function::AstStatFunction;
use ulua_common::FFlag;

use crate::{enums::value_context::ValueContext, records::type_checker_2::TypeChecker2};

impl TypeChecker2 {
  /// # Safety
  /// 调用方须保证 `stat` 等裸指针参数有效，且满足 C++ 原实现的调用契约。
  pub(crate) fn visit_ast_stat_function(&mut self, stat: *mut AstStatFunction) {
    unsafe {
      let name = (*stat).name;
      let func = (*stat).func;

      self.visit_ast_expr_value_context(name, ValueContext::LValue);
      // SAFETY: func 指向 AST arena 节点。
      self.visit_ast_expr_function(&*func);

      if FFlag::LuauCheckFunctionStatementTypes.get() {
        // SAFETY: name/func 指向 AST arena 节点。
        let lhs_type = self.lookup_type(&*name);
        let rhs_type = self.lookup_type(&(*func).base);
        let location = (*func).base.base.location;
        self.test_is_subtype_type_id_type_id_location(rhs_type, lhs_type, location);
      }
    }
  }
}
