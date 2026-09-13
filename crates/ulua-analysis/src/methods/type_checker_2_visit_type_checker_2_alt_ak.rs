use core::ffi::CStr;

use ulua_ast::records::ast_expr_index_name::AstExprIndexName;

use crate::{enums::value_context::ValueContext, records::type_checker_2::TypeChecker2};
impl TypeChecker2 {
  /// # Safety
  /// 调用方须保证满足 C++ 原实现的调用契约。
  pub unsafe fn visit_ast_expr_index_name_value_context(
    &mut self,
    index_name: *mut AstExprIndexName,
    context: ValueContext,
  ) {
    unsafe {
      let expr = (*index_name).expr;
      let location = (*index_name).base.base.location;
      let index = (*index_name).index;
      let prop_name = CStr::from_ptr(index.value).to_string_lossy();
      let ast_index_expr_ty = (*self.builtin_types).string_type;
      self.visit_expr_name(expr, location, &prop_name, context, ast_index_expr_ty);
    }
  }
}
