use crate::{
  enums::subtyping_variance::SubtypingVariance,
  records::{path::Path, subtyping_reasoning::SubtypingReasoning},
};

impl SubtypingReasoning {
  pub fn new(sub_path: Path, super_path: Path, variance: SubtypingVariance) -> Self {
    Self {
      sub_path,
      super_path,
      variance,
      is_property_modifier_violation: false,
    }
  }
}
