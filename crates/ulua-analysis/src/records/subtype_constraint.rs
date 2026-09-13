use crate::type_aliases::type_id::TypeId;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SubtypeConstraint {
  pub(crate) sub_type: TypeId,
  pub(crate) super_type: TypeId,
}
