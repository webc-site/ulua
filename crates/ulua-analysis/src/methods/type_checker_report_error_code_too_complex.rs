use ulua_ast::records::location::Location;

use crate::records::{
  code_too_complex::CodeTooComplex, type_checker::TypeChecker, type_error::TypeError,
};

impl TypeChecker {
  pub fn report_error_code_too_complex(&mut self, location: &Location) {
    let error =
      TypeError::type_error_location_type_error_data(*location, CodeTooComplex::default().into());
    self.report_error_type_error(&error);
  }
}
