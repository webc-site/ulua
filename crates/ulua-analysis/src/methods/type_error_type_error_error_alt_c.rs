use alloc::string::String;

use ulua_ast::records::location::Location;

use crate::{records::type_error::TypeError, type_aliases::type_error_data::TypeErrorData};
impl TypeError {
  pub fn type_error_location_type_error_data(location: Location, data: TypeErrorData) -> Self {
    Self {
      location,
      module_name: String::new(),
      data,
    }
  }
}
