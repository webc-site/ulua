use core::ffi::c_char;

use ulua_ast::records::ast_array::AstArray;
use ulua_common::functions::hash_range::hashRange;

use crate::records::ast_array_predicate::AstArrayPredicate;

impl AstArrayPredicate {
  /// # Safety
  /// 调用方须保证 `value` 等裸指针参数有效，且满足 C++ 原实现的调用契约。
  #[inline]
  pub unsafe fn operator_call_2(&self, value: *const AstArray<c_char>) -> usize {
    unsafe {
      let value_ref = &*value;
      hashRange(value_ref.begin() as *const c_char, value_ref.len())
    }
  }
}
