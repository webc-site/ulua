use ulua_ast::records::location::Location;

use crate::{
  records::{
    code_too_complex::CodeTooComplex, constraint_generator::ConstraintGenerator,
    type_error::TypeError,
  },
  type_aliases::type_error_data::TypeErrorData,
};
impl ConstraintGenerator {
  pub fn report_code_too_complex(&mut self, location: Location) {
    unsafe {
      let err = TypeErrorData::CodeTooComplex(CodeTooComplex { _unused: None });
      self.errors.push(TypeError {
        location,
        module_name: self.module.as_ref().unwrap().name.clone(),
        data: err.clone(),
      });
      if !self.logger.is_null() {
        (*self.logger).capture_generation_error(self.errors.last().unwrap());
      }
      self.recursion_limit_met = true;
    }
  }
}
