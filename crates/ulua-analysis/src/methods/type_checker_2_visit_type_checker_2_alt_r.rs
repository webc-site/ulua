use core::ffi::CStr;

use ulua_ast::records::{ast_node::AstNode, ast_stat_type_alias::AstStatTypeAlias};

use crate::{
  records::{
    recursive_restraint_violation::RecursiveRestraintViolation, type_checker_2::TypeChecker2,
  },
  type_aliases::type_error_data::TypeErrorData,
};
impl TypeChecker2 {
  /// # Safety
  /// 调用方须保证 `stat` 等裸指针参数有效，且满足 C++ 原实现的调用契约。
  pub(crate) fn visit_ast_stat_type_alias(&mut self, stat: *mut AstStatTypeAlias) {
    // We will not visit type aliases that do not have an associated scope,
    // this means that (probably) this was a duplicate type alias or a
    // type alias with an illegal name (like `typeof`).
    if !unsafe {
      (*self.module)
        .ast_scopes
        .contains_key(&(stat as *const AstNode))
    } {
      return;
    }

    let scope = self.find_innermost_scope(unsafe { (*stat).base.base.location });
    if !scope.is_null() {
      let name = unsafe { (*stat).name };
      let name_str = unsafe { CStr::from_ptr(name.value).to_string_lossy().into_owned() };
      if let Some(loc) = unsafe { (*scope).is_invalid_type_alias(&name_str) } {
        self.report_error_type_error_data_location(
          TypeErrorData::RecursiveRestraintViolation(RecursiveRestraintViolation::default()),
          &loc,
        );
      }
    }

    let generics = unsafe { (*stat).generics };
    let generic_packs = unsafe { (*stat).generic_packs };
    self.visit_generics(generics, generic_packs);

    let type_ptr = unsafe { (*stat).type_ptr };
    unsafe { self.visit_ast_type(type_ptr) };
  }
}
