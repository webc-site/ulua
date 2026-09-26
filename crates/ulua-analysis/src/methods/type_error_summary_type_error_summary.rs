use ulua_ast::records::location::Location;

use crate::{
  records::type_error_summary::TypeErrorSummary, type_aliases::module_name_type::ModuleName,
};
impl TypeErrorSummary {
  pub fn type_error_summary_type_error_summary(
    location: Location,
    module_name: ModuleName,
    code: i32,
  ) -> Self {
    Self {
      location,
      module_name,
      code,
    }
  }
}
