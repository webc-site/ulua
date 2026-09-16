use core::ffi::c_char;

use ulua_ast::records::ast_array::AstArray;

use crate::records::ast_array_predicate::AstArrayPredicate;

impl AstArrayPredicate {
  #[inline]
  /// # Safety
  /// 调用方须保证满足 C++ 原实现的调用契约。
  pub unsafe fn operator_call(
    &self,
    lhs: *const AstArray<c_char>,
    rhs: *const AstArray<c_char>,
  ) -> bool {
    if !lhs.is_null() && !rhs.is_null() {
      unsafe {
        let lhs_ref = &*lhs;
        let rhs_ref = &*rhs;
        lhs_ref.as_bytes() == rhs_ref.as_bytes()
      }
    } else {
      lhs == rhs
    }
  }
}
