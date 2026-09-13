use crate::records::{constraint_solver::ConstraintSolver, type_error::TypeError};
impl ConstraintSolver {
  pub fn report_error_type_error(&mut self, e: TypeError) {
    {
      self.errors.push(e);
      let last_error = self.errors.last_mut().unwrap();
      if let Some(ref module) = self.module {
        last_error.module_name = module.name.clone();
      }
    }
  }
}
