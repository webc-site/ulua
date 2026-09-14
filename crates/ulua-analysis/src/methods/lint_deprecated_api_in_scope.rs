use crate::records::{function_type::FunctionType, lint_deprecated_api::LintDeprecatedApi};

impl LintDeprecatedApi {
  pub fn in_scope(&self, fty: *const FunctionType) -> bool {
    self.function_type_scope_stack.contains(&fty)
  }
}
