use ulua_ast::records::{deprecated_info::DeprecatedInfo, location::Location};
use ulua_config::enums::code::Code;

use crate::{
  functions::emit_warning::emit_warning, records::lint_deprecated_api::LintDeprecatedApi,
};
impl LintDeprecatedApi {
  pub fn report_member_info(
    &mut self,
    location: &Location,
    table_name: Option<&str>,
    function_name: &str,
    info: &DeprecatedInfo,
  ) {
    let use_part = info
      .use_suggestion()
      .map(|value| format!(", use '{}' instead", value))
      .unwrap_or_default();
    let reason_part = info
      .reason()
      .map(|value| format!(". {}", value))
      .unwrap_or_default();

    if let Some(table_name) = table_name {
      emit_warning(
        unsafe { &mut *self.context },
        Code::DeprecatedApi,
        *location,
        format_args!(
          "Member '{}.{}' is deprecated{}{}",
          table_name, function_name, use_part, reason_part
        ),
      );
    } else {
      emit_warning(
        unsafe { &mut *self.context },
        Code::DeprecatedApi,
        *location,
        format_args!(
          "Member '{}' is deprecated{}{}",
          function_name, use_part, reason_part
        ),
      );
    }
  }
}
