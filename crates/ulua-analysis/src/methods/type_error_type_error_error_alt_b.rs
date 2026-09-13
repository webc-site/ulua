use ulua_ast::records::location::Location;

use crate::{
  records::type_error::TypeError,
  type_aliases::{module_name_type::ModuleName, type_error_data::TypeErrorData},
};

impl TypeError {
  pub fn type_error_location_module_name_type_error_data(
    location: Location,
    module_name: ModuleName,
    data: TypeErrorData,
  ) -> Self {
    Self {
      location,
      module_name,
      data,
    }
  }
}
