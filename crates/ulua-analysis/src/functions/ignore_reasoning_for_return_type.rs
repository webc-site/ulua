use std::mem::swap;

use crate::{
  functions::{
    merge_reasonings::k_empty_reasoning, reasoning_is_return_types::reasoning_is_return_types,
  },
  records::subtyping_result::SubtypingResult,
  type_aliases::subtyping_reasonings::SubtypingReasonings,
};
pub fn ignore_reasoning_for_return_type(sr: &mut SubtypingResult) {
  let mut result = SubtypingReasonings::new(k_empty_reasoning());

  for reasoning in sr.reasoning.iter() {
    if reasoning_is_return_types(&reasoning.sub_path)
      && reasoning_is_return_types(&reasoning.super_path)
    {
      continue;
    }
    result.insert(reasoning.clone());
  }

  swap(&mut sr.reasoning, &mut result);

  if sr.reasoning.empty() && sr.generic_bounds_mismatches.is_empty() && sr.errors.is_empty() {
    sr.is_subtype = true;
  }
}
