use ulua_ast::records::location::Location;
use ulua_config::enums::code::Code;

use crate::{
  functions::emit_warning::emit_warning, records::lint_deprecated_api::LintDeprecatedApi,
};
impl LintDeprecatedApi {
  pub fn report_member(
    &mut self,
    location: &Location,
    table_name: Option<&str>,
    function_name: &str,
  ) {
    if let Some(table_name) = table_name {
      emit_warning(
        unsafe { &mut *self.context },
        Code::DeprecatedApi,
        *location,
        format_args!("Member '{}.{}' is deprecated", table_name, function_name),
      );
    } else {
      emit_warning(
        unsafe { &mut *self.context },
        Code::DeprecatedApi,
        *location,
        format_args!("Member '{}' is deprecated", function_name),
      );
    }
  }
}
