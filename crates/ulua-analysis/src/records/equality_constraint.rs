use crate::type_aliases::type_id::TypeId;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct EqualityConstraint {
  pub(crate) result_type: TypeId,
  pub(crate) assignment_type: TypeId,
}
