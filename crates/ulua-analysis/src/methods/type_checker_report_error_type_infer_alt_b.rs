use ulua_ast::records::location::Location;

use crate::{
  records::{type_checker::TypeChecker, type_error::TypeError},
  type_aliases::type_error_data::TypeErrorData,
};
impl TypeChecker {
  pub fn report_error_location_type_error_data(
    &mut self,
    _location: &Location,
    _error_data: TypeErrorData,
  ) {
    let error = TypeError::type_error_location_type_error_data(*_location, _error_data);
    self.report_error_type_error(&error);
  }
}
