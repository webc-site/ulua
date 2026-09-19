use ulua_ast::records::location::Location;
use ulua_config::enums::code::Code;

use crate::{
  functions::emit_warning::emit_warning, records::lint_duplicate_function::LintDuplicateFunction,
};
impl LintDuplicateFunction {
  #[inline]
  pub fn report(&mut self, name: &str, location: Location, other_location: Location) {
    self.report_location_c_char_location(name, location, other_location);
  }

  pub fn report_location_c_char_location(
    &mut self,
    name: &str,
    location: Location,
    other_location: Location,
  ) {
    emit_warning(
      unsafe { &mut *self.context },
      Code::DuplicateFunction,
      location,
      format_args!(
        "Duplicate function definition: '{}' also defined on line {}",
        name,
        other_location.begin.line + 1
      ),
    );
  }
}
