use alloc::vec::Vec;

use crate::type_aliases::{constraint_v::ConstraintV, type_id::TypeId};

#[derive(Debug, Clone)]
pub struct SelectedOverload {
  pub overload: Option<TypeId>,
  pub assumed_constraints: Vec<ConstraintV>,
  pub should_retry: bool,
}
