use crate::type_aliases::type_id::TypeId;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct TypesAreUnrelated {
  pub(crate) left: TypeId,
  pub(crate) right: TypeId,
}
