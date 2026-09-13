use crate::records::type_ids::TypeIds;

#[derive(Debug, Clone, Default)]
pub struct GenericBounds {
  pub(crate) lower_bound: TypeIds,
  pub(crate) upper_bound: TypeIds,
}
