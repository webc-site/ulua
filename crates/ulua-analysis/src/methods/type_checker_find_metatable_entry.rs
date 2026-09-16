use alloc::string::String;

use ulua_ast::records::location::Location;

use crate::{
  functions::find_metatable_entry::find_metatable_entry,
  records::type_checker::TypeChecker,
  type_aliases::{error_vec::ErrorVec, type_id::TypeId},
};
impl TypeChecker {
  /// # Safety
  /// 调用方须保证 `参数` 等裸指针参数有效，且满足 C++ 原实现的调用契约。
  pub(crate) fn find_metatable_entry(
    &mut self,
    ty: TypeId,
    entry: String,
    location: &Location,
    add_errors: bool,
  ) -> Option<TypeId> {
    let mut errors: ErrorVec = ErrorVec::new();
    let result = find_metatable_entry(self.builtin_types, &mut errors, ty, &entry, *location);
    if add_errors {
      self.report_errors(&errors);
    }
    result
  }
}
