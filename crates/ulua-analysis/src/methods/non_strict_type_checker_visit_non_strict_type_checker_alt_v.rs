use ulua_ast::records::ast_stat_class::AstStatClass;
use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::records::{
  non_strict_context::NonStrictContext, non_strict_type_checker::NonStrictTypeChecker,
};

impl NonStrictTypeChecker {
  /// # Safety
  /// 调用方须保证 `decl_class` 等裸指针参数有效，且满足 C++ 原实现的调用契约。
  pub unsafe fn visit_ast_stat_class(&mut self, decl_class: *mut AstStatClass) -> NonStrictContext {
    unsafe {
      let members = &(*decl_class).members;
      for prop in members.as_slice() {
        if let Some(property) = prop.get_if_0() {
          self.visit_ast_type(property.ty);
        } else if let Some(method) = prop.get_if_1() {
          self.visit_ast_expr_function(method.function);
        } else {
          LUAU_ASSERT!(false);
        }
      }

      NonStrictContext::new()
    }
  }
}
