use alloc::vec::Vec;

use crate::type_aliases::type_id::TypeId;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
pub struct RefinementPartition {
  /// Types that we want to intersect against the type of the expression.
  pub(crate) discriminant_types: Vec<TypeId>,
  /// Sometimes the type we're discriminating against is implicitly nil.
  pub(crate) should_append_nil_type: bool,
}

impl RefinementPartition {
  pub fn discriminant_types(&self) -> &[TypeId] {
    &self.discriminant_types
  }

  pub fn should_append_nil_type(&self) -> bool {
    self.should_append_nil_type
  }
}
