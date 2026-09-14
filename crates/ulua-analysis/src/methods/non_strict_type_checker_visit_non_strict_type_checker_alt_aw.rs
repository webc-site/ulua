use ulua_ast::records::ast_type_function::AstTypeFunction;

use crate::records::non_strict_type_checker::NonStrictTypeChecker;

impl NonStrictTypeChecker {
  /// # Safety
  /// 调用方须保证 `function` 等裸指针参数有效，且满足 C++ 原实现的调用契约。
  pub(crate) fn visit_ast_type_function(&mut self, function: *mut AstTypeFunction) {
    unsafe {
      self.visit_ast_type_list(&mut (*function).arg_types);
      self.visit_ast_type_pack((*function).return_types);
    }
  }
}
