use crate::{enums::code::Code, records::lint_warning::LintWarning};

impl LintWarning {
  pub fn parse_name(name: &str) -> Code {
    for code in Code::ALL {
      if name == Self::get_name(code) {
        return code;
      }
    }

    Code::Unknown
  }
}
