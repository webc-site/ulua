use crate::type_aliases::type_id::TypeId;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SimplifyConstraint {
  pub(crate) ty: TypeId,
}
