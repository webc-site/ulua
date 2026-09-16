use crate::{
  functions::merge_reasonings::k_empty_reasoning, records::subtyping_result::SubtypingResult,
  type_aliases::subtyping_reasonings::SubtypingReasonings,
};

impl SubtypingResult {
  pub fn with_property_modifier_violation(&mut self) -> &mut Self {
    let mut updated = SubtypingReasonings::new(k_empty_reasoning());
    for r in self.reasoning.iter() {
      let mut r = r.clone();
      r.is_property_modifier_violation = true;
      updated.insert(r);
    }
    self.reasoning = updated;
    self
  }
}
