use ulua_ast::records::location::Location;
use ulua_config::enums::code::Code;

use crate::{
  functions::emit_warning::emit_warning, records::lint_deprecated_api::LintDeprecatedApi,
};
impl LintDeprecatedApi {
  pub fn report_function(&mut self, location: &Location, function_name: &str) {
    emit_warning(
      unsafe { &mut *self.context },
      Code::DeprecatedApi,
      *location,
      format_args!("Function '{}' is deprecated", function_name),
    );
  }
}
