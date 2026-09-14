use alloc::{string::String, vec::Vec};

use crate::type_aliases::type_id::TypeId;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ExplicitFunctionAnnotationRecommended {
  pub(crate) recommended_args: Vec<(String, TypeId)>,
  pub(crate) recommended_return: TypeId,
}

impl ExplicitFunctionAnnotationRecommended {
  pub fn recommended_args(&self) -> &[(String, TypeId)] {
    &self.recommended_args
  }

  pub fn recommended_return(&self) -> TypeId {
    self.recommended_return
  }
}
