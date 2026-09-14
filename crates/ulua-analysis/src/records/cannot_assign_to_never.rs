use alloc::vec::Vec;

use crate::{enums::reason::Reason, type_aliases::type_id::TypeId};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct CannotAssignToNever {
  /// type of the rvalue being assigned
  pub(crate) rhs_type: TypeId,
  /// Originating type.
  pub(crate) cause: Vec<TypeId>,
  pub(crate) reason: Reason,
}

impl CannotAssignToNever {
  pub const fn new(rhs_type: TypeId, cause: Vec<TypeId>, reason: Reason) -> Self {
    Self {
      rhs_type,
      cause,
      reason,
    }
  }
}

impl CannotAssignToNever {
  pub fn rhs_type(&self) -> TypeId {
    self.rhs_type
  }

  pub fn cause(&self) -> &[TypeId] {
    &self.cause
  }

  pub fn reason(&self) -> Reason {
    self.reason
  }
}
