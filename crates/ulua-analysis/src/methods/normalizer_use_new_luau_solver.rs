use crate::{enums::solver_mode::SolverMode, records::normalizer::Normalizer};

impl Normalizer {
  pub fn use_new_luau_solver(&self) -> bool {
    self.solver_mode == SolverMode::New
  }
}
