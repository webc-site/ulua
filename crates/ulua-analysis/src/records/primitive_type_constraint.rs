use crate::type_aliases::type_id::TypeId;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct PrimitiveTypeConstraint {
  pub(crate) free_type: TypeId,
  pub(crate) expected_type: Option<TypeId>,
  pub(crate) primitive_type: TypeId,
}

impl PrimitiveTypeConstraint {
  pub fn free_type(&self) -> TypeId {
    self.free_type
  }

  pub fn expected_type(&self) -> Option<TypeId> {
    self.expected_type
  }

  pub fn primitive_type(&self) -> TypeId {
    self.primitive_type
  }
}
