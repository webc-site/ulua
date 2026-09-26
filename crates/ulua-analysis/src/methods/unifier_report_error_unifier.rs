use ulua_ast::records::location::Location;

use crate::{
  records::{type_error::TypeError, unifier::Unifier},
  type_aliases::type_error_data::TypeErrorData,
};

impl Unifier {
  pub fn report_error_location_type_error_data(&mut self, location: Location, data: TypeErrorData) {
    let err = TypeError::type_error_location_type_error_data(location, data);
    self.errors.push(err);
    self.failure = true;
  }

  pub fn report_error_type_error(&mut self, err: TypeError) {
    self.errors.push(err);
    self.failure = true;
  }
}
