use crate::{
  enums::solver_mode::SolverMode, records::clone_public_interface::ClonePublicInterface,
};

impl ClonePublicInterface {
  pub fn is_new_solver(&self) -> bool {
    self.solver_mode == SolverMode::New
  }
}
