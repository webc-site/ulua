use crate::type_aliases::type_id::TypeId;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct PrimitiveTypeConstraint {
  pub(crate) free_type: TypeId,
  pub(crate) expected_type: Option<TypeId>,
  pub(crate) primitive_type: TypeId,
}
