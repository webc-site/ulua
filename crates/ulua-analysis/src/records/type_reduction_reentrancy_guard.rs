use crate::records::unifier_shared_state::UnifierSharedState;

#[derive(Debug)]
pub struct TypeReductionReentrancyGuard {
  pub shared_state: *mut UnifierSharedState,
}
