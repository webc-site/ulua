use crate::records::type_ids::TypeIds;

#[derive(Debug, Clone)]
pub struct NormalizedFunctionType {
  pub(crate) is_top: bool,
  pub(crate) parts: TypeIds,
}
