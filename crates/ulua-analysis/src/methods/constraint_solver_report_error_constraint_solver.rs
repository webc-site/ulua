use ulua_ast::records::location::Location;

use crate::{
  records::{constraint_solver::ConstraintSolver, type_error::TypeError},
  type_aliases::type_error_data::TypeErrorData,
};

impl ConstraintSolver {
  pub fn report_error_type_error_data_location(
    &mut self,
    data: TypeErrorData,
    location: &Location,
  ) {
    self
      .errors
      .push(TypeError::type_error_location_type_error_data(
        *location, data,
      ));
    if let Some(ref module) = self.module {
      let name = module.name.clone();
      if let Some(last) = self.errors.last_mut() {
        last.module_name = name;
      }
    }
  }
}
