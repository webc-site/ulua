use crate::type_aliases::type_id::TypeId;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct HasIndexerConstraint {
  pub(crate) result_type: TypeId,
  pub(crate) subject_type: TypeId,
  pub(crate) index_type: TypeId,
}
