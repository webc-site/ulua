use crate::{records::refinement_key::RefinementKey, type_aliases::type_id::TypeId};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Proposition {
  pub(crate) key: *const RefinementKey,
  pub(crate) discriminant_ty: TypeId,
  pub(crate) implicit_from_call: bool,
}

impl Proposition {
  pub fn key(&self) -> *const RefinementKey {
    self.key
  }

  pub fn discriminant_ty(&self) -> TypeId {
    self.discriminant_ty
  }

  pub fn implicit_from_call(&self) -> bool {
    self.implicit_from_call
  }
}
