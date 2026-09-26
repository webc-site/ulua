use core::str::FromStr;

use crate::{enums::code::Code, records::lint_warning::LintWarning};

impl LintWarning {
  #[inline]
  pub fn parse_name(name: &str) -> Code {
    Code::from_str(name).unwrap_or(Code::Unknown)
  }
}
