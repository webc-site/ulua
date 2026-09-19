use std::option::Option;

use ulua_ast::records::{
  ast_array::AstArray,
  ast_expr_table::{AstExprTable, Item},
};
use ulua_common::{fflag, fint};

use crate::{
  enums::value_context::ValueContext,
  records::{
    non_strict_context::NonStrictContext, non_strict_type_checker::NonStrictTypeChecker,
    recursion_counter::RecursionCounter,
  },
};
impl NonStrictTypeChecker {
  /// # Safety
  /// 调用方须保证 `table` 等裸指针参数有效，且满足 C++ 原实现的调用契约。
  pub(crate) fn visit_ast_expr_table(&mut self, table: *mut AstExprTable) -> NonStrictContext {
    unsafe {
      let mut _rc = Option::None;
      if fflag::LuauAddRecursionCounterToNonStrictTypeChecker.get() {
        _rc = Option::Some(RecursionCounter::recursion_counter_i32(
          &mut self.non_strict_recursion_count,
        ));
        if fint::LuauNonStrictTypeCheckerRecursionLimit.get() > 0
          && self.non_strict_recursion_count >= fint::LuauNonStrictTypeCheckerRecursionLimit.get()
        {
          return NonStrictContext::new();
        }
      }

      let items: AstArray<Item> = (*table).items.clone();
      for item in items.as_slice() {
        if !item.key.is_null() {
          self.visit_ast_expr_value_context(item.key, ValueContext::RValue);
        }
        self.visit_ast_expr_value_context(item.value, ValueContext::RValue);
      }

      NonStrictContext::new()
    }
  }
}
