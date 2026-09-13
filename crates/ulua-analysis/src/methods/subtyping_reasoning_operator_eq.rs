use crate::records::subtyping_reasoning::SubtypingReasoning;

impl SubtypingReasoning {
  pub fn operator_eq(&self, other: &SubtypingReasoning) -> bool {
    self.sub_path == other.sub_path
      && self.super_path == other.super_path
      && self.variance == other.variance
      && self.is_property_modifier_violation == other.is_property_modifier_violation
  }
}
