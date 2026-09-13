use alloc::string::String;
use core::mem::take;

use ulua_ast::records::ast_expr_constant_string::AstExprConstantString;

use crate::{
  records::{
    singleton_type::SingletonType, string_singleton::StringSingleton, type_checker_2::TypeChecker2,
    type_mismatch::TypeMismatch,
  },
  type_aliases::singleton_variant::SingletonVariant,
};
impl TypeChecker2 {
  /// # Safety
  /// 调用方须保证 `expr` 等裸指针参数有效，且满足 C++ 原实现的调用契约。
  pub(crate) fn visit_ast_expr_constant_string(&mut self, expr: *mut AstExprConstantString) {
    // strings use specialized inference logic for singleton type_arguments,
    // which can lead to real type errors here.
    unsafe {
      let string_bytes = (*expr).value.as_bytes();
      let string_data = String::from_utf8_lossy(string_bytes).into_owned();

      // C++: module->internalTypes.addType(SingletonType{StringSingleton{...}})
      let best_type =
        (*self.module)
          .internal_types
          .add_type(SingletonType::new(SingletonVariant::V1(
            StringSingleton::new(string_data),
          )));
      let inferred_type = self.lookup_type(&(*expr).base);
      let scope = self.find_innermost_scope((*expr).base.base.location);

      let mut r = (*self.subtyping).is_subtype_type_id_type_id_not_null_scope(
        best_type,
        inferred_type,
        scope,
      );

      if !self.is_error_suppressing_location_type_id((*expr).base.base.location, inferred_type) {
        if !r.is_subtype {
          self.report_error_type_error_data_location(
            TypeMismatch::from_wanted_given(inferred_type, best_type).into(),
            &(*expr).base.base.location,
          );
        }

        for e in &mut r.errors {
          e.location = (*expr).base.base.location;
        }
        self.report_errors(take(&mut r.errors));
      }
    }
  }
}
