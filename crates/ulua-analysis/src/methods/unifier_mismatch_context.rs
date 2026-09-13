use crate::{
  enums::{context_error::Context, variance::Variance},
  records::unifier::Unifier,
};
impl Unifier {
  pub fn unifier_mismatch_context(&mut self) -> Context {
    match self.variance {
      Variance::Covariant => Context::CovariantContext,
      Variance::Invariant => Context::InvariantContext,
    }
  }
}
