use core::sync::atomic::Ordering;

use crate::{enums::solver_mode::SolverMode, records::frontend::Frontend};

impl Frontend {
  pub fn set_luau_solver_mode(&mut self, mode: SolverMode) {
    self
      .use_new_luau_solver
      .store(mode as i32, Ordering::Relaxed);
  }
}
