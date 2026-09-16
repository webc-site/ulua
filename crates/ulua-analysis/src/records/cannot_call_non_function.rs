use crate::type_aliases::type_id::TypeId;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct CannotCallNonFunction {
  pub(crate) ty: TypeId,
}

impl CannotCallNonFunction {
  pub fn ty(&self) -> TypeId {
    self.ty
  }
}
