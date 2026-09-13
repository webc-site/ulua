use alloc::vec::Vec;

use crate::{
  records::boundary_snapshot::BoundarySnapshot, type_aliases::step_snapshot::StepSnapshot,
};
#[derive(Debug, Clone, Default)]
pub struct TypeSolveLog {
  pub(crate) initial_state: BoundarySnapshot,
  pub(crate) step_states: Vec<StepSnapshot>,
  pub(crate) final_state: BoundarySnapshot,
}
