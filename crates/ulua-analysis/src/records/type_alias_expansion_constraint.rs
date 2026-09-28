use crate::type_aliases::type_id::TypeId;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct TypeAliasExpansionConstraint {
  /// Must be a PendingExpansionType.
  pub(crate) target: TypeId,
}
