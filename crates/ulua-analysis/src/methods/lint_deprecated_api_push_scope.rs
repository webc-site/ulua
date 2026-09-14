use ulua_common::LUAU_ASSERT;

use crate::records::{function_type::FunctionType, lint_deprecated_api::LintDeprecatedApi};

impl LintDeprecatedApi {
  pub fn push_scope(&mut self, fty: *const FunctionType) {
    LUAU_ASSERT!(!fty.is_null());
    self.function_type_scope_stack.push(fty);
  }
}
