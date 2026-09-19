use crate::type_aliases::type_id::TypeId;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct NegationType {
  pub(crate) ty: TypeId,
}

impl NegationType {
  pub fn new(ty: TypeId) -> Self {
    Self { ty }
  }
}
