use core::sync::atomic::Ordering;

use crate::{enums::solver_mode::SolverMode, records::frontend::Frontend};

impl Frontend {
  pub fn get_luau_solver_mode(&self) -> SolverMode {
    match self.use_new_luau_solver.load(Ordering::Relaxed) {
      x if x == SolverMode::Old as i32 => SolverMode::Old,
      _ => SolverMode::New,
    }
  }
}
