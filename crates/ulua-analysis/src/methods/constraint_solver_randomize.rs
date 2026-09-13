use crate::records::constraint_solver::ConstraintSolver;

impl ConstraintSolver {
  pub fn randomize(&mut self, seed: u32) {
    if self.unsolved_constraints.is_empty() {
      return;
    }

    let mut rng = seed;

    for i in (1..self.unsolved_constraints.len()).rev() {
      let j = (rng as usize) % (i + 1);

      self.unsolved_constraints.swap(i, j);

      rng = rng.wrapping_mul(1664525).wrapping_add(1013904223);
    }
  }
}
