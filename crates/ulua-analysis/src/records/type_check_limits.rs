use alloc::sync::Arc;

use crate::records::frontend_cancellation_token::FrontendCancellationToken;

#[derive(Debug, Clone, Default)]
pub struct TypeCheckLimits {
  pub(crate) finish_time: Option<f64>,
  pub(crate) instantiation_child_limit: Option<i32>,
  pub(crate) unifier_iteration_limit: Option<i32>,
  pub(crate) cancellation_token: Option<Arc<FrontendCancellationToken>>,
}

impl TypeCheckLimits {
  pub fn finish_time(&self) -> Option<f64> {
    self.finish_time
  }

  pub fn instantiation_child_limit(&self) -> Option<i32> {
    self.instantiation_child_limit
  }

  pub fn unifier_iteration_limit(&self) -> Option<i32> {
    self.unifier_iteration_limit
  }

  pub fn cancellation_token(&self) -> Option<Arc<FrontendCancellationToken>> {
    self.cancellation_token.clone()
  }
}
