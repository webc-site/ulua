use alloc::string::String;

use crate::type_aliases::type_pack_id::TypePackId;
#[derive(Debug, Clone, PartialEq)]
pub struct TypePackMismatch {
  pub(crate) wanted_tp: TypePackId,
  pub(crate) given_tp: TypePackId,
  pub(crate) reason: String,
}

impl TypePackMismatch {
  pub fn wanted_tp(&self) -> TypePackId {
    self.wanted_tp
  }

  pub fn given_tp(&self) -> TypePackId {
    self.given_tp
  }
}
