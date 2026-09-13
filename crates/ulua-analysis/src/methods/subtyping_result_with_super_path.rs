use crate::{
  enums::subtyping_variance::SubtypingVariance,
  functions::merge_reasonings::k_empty_reasoning,
  records::{
    path::Path, subtyping_reasoning::SubtypingReasoning, subtyping_result::SubtypingResult,
  },
  type_aliases::subtyping_reasonings::SubtypingReasonings,
};

impl SubtypingResult {
  pub fn with_super_path(&mut self, path: Path) -> &mut Self {
    if self.reasoning.empty() {
      self.reasoning.insert(SubtypingReasoning {
        sub_path: Path::default(),
        super_path: path,
        variance: SubtypingVariance::Covariant,
        is_property_modifier_violation: false,
      });
    } else {
      let mut updated = SubtypingReasonings::new(k_empty_reasoning());
      for r in self.reasoning.iter() {
        let mut r = r.clone();
        r.super_path = path.append(&r.super_path);
        updated.insert(r);
      }
      self.reasoning = updated;
    }

    self
  }
}
