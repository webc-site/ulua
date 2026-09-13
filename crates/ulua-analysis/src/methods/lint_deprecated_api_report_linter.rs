use ulua_ast::records::location::Location;
use ulua_config::enums::code::Code;

use crate::{
  functions::emit_warning::emit_warning,
  records::{lint_deprecated_api::LintDeprecatedApi, property_type::Property},
};
impl LintDeprecatedApi {
  pub fn report_property(
    &mut self,
    location: &Location,
    prop: &Property,
    container: Option<&str>,
    field: &str,
  ) {
    let suggestion = if prop.deprecated_suggestion.is_empty() {
      ""
    } else {
      prop.deprecated_suggestion.as_str()
    };

    if let Some(container) = container {
      if suggestion.is_empty() {
        emit_warning(
          unsafe { &mut *self.context },
          Code::DeprecatedApi,
          *location,
          format_args!("Member '{}.{}' is deprecated", container, field),
        );
      } else {
        emit_warning(
          unsafe { &mut *self.context },
          Code::DeprecatedApi,
          *location,
          format_args!(
            "Member '{}.{}' is deprecated, use '{}' instead",
            container, field, suggestion
          ),
        );
      }
    } else if suggestion.is_empty() {
      emit_warning(
        unsafe { &mut *self.context },
        Code::DeprecatedApi,
        *location,
        format_args!("Member '{}' is deprecated", field),
      );
    } else {
      emit_warning(
        unsafe { &mut *self.context },
        Code::DeprecatedApi,
        *location,
        format_args!(
          "Member '{}' is deprecated, use '{}' instead",
          field, suggestion
        ),
      );
    }
  }
}
