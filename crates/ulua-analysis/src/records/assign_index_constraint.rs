use crate::type_aliases::type_id::TypeId;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct AssignIndexConstraint {
  pub(crate) lhs_type: TypeId,
  pub(crate) index_type: TypeId,
  pub(crate) rhs_type: TypeId,
  /// The canonical write type of the property. It is _solely_ used to
  /// populate ast_types during constraint resolution. Nothing should ever
  /// block on it.
  pub(crate) prop_type: TypeId,
}
