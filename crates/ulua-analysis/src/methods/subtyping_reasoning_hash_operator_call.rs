use crate::records::{
  path_hash::PathHash, subtyping_reasoning::SubtypingReasoning,
  subtyping_reasoning_hash::SubtypingReasoningHash,
};

impl SubtypingReasoningHash {
  pub fn operator_call(&self, r: &SubtypingReasoning) -> usize {
    let path_hash = PathHash;
    path_hash.operator_call_6(&r.sub_path)
      ^ (path_hash.operator_call_6(&r.super_path) << 1)
      ^ ((r.variance as usize) << 1)
      ^ ((r.is_property_modifier_violation as usize) << 2)
  }
}
