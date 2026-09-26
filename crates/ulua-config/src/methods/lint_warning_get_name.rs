use crate::{enums::code::Code, records::lint_warning::LintWarning};

impl LintWarning {
  #[inline]
  pub fn get_name(code: Code) -> &'static str {
    code.into()
  }
}
