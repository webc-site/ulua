use ulua_common::LUAU_ASSERT;

use crate::records::{function_type::FunctionType, lint_deprecated_api::LintDeprecatedApi};

impl LintDeprecatedApi {
  pub fn pop_scope(&mut self, fty: *const FunctionType) {
    LUAU_ASSERT!(!fty.is_null());
    LUAU_ASSERT!(!self.function_type_scope_stack.is_empty());
    LUAU_ASSERT!(self.function_type_scope_stack.last() == Some(&fty));
    self.function_type_scope_stack.pop();
  }
}
