use ulua_ast::records::location::Location;

use crate::{
  records::{non_strict_type_checker::NonStrictTypeChecker, type_error::TypeError},
  type_aliases::type_error_data::TypeErrorData,
};
impl NonStrictTypeChecker {
  pub fn report_error(&mut self, data: TypeErrorData, location: &Location) {
    unsafe {
      (*self.module)
        .errors
        .push(TypeError::type_error_location_module_name_type_error_data(
          *location,
          (*self.module).name.clone(),
          data,
        ));
    }
  }
}
