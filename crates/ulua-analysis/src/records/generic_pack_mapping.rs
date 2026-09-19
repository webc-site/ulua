//! Source: `Analysis/include/Luau/TypePath.h:115` (hand-ported)
use crate::type_aliases::type_pack_id::TypePackId;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GenericPackMapping {
  pub mapped_type: TypePackId,
}

impl GenericPackMapping {
  pub fn operator_eq(&self, other: &GenericPackMapping) -> bool {
    self.mapped_type == other.mapped_type
  }
}
