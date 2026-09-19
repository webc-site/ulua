use ulua_common::records::dense_hash_table::DenseDefault;

use crate::{enums::subtyping_variance::SubtypingVariance, records::path::Path};
#[derive(Debug, Clone)]
pub struct SubtypingReasoning {
  pub(crate) sub_path: Path,
  pub(crate) super_path: Path,
  pub(crate) variance: SubtypingVariance,
  pub(crate) is_property_modifier_violation: bool,
}

// Empty-key sentinel for DenseHashSet<SubtypingReasoning, ...> members.
impl DenseDefault for SubtypingReasoning {
  fn dense_default() -> Self {
    SubtypingReasoning {
      sub_path: Default::default(),
      super_path: Default::default(),
      variance: Default::default(),
      is_property_modifier_violation: false,
    }
  }
}
