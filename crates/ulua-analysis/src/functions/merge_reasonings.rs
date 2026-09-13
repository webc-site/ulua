use ulua_common::{
  FInt,
  records::dense_hash_table::{DenseDefault, DenseHasher},
};

use crate::{
  enums::subtyping_variance::SubtypingVariance,
  records::{
    path::Path, path_hash::PathHash, subtyping_reasoning::SubtypingReasoning,
    subtyping_reasoning_hash::SubtypingReasoningHash, subtyping_result::SubtypingResult,
  },
  type_aliases::subtyping_reasonings::SubtypingReasonings,
};
impl PartialEq for SubtypingReasoning {
  fn eq(&self, other: &Self) -> bool {
    self.operator_eq(other)
  }
}

impl Eq for SubtypingReasoning {}

impl DenseHasher<SubtypingReasoning> for SubtypingReasoningHash {
  fn hash(&self, r: &SubtypingReasoning) -> usize {
    PathHash.operator_call_6(&r.sub_path)
      ^ (PathHash.operator_call_6(&r.super_path) << 1)
      ^ ((r.variance as usize) << 1)
      ^ ((r.is_property_modifier_violation as usize) << 2)
  }
}

pub(crate) fn k_empty_reasoning() -> SubtypingReasoning {
  SubtypingReasoning {
    sub_path: Path::default(),
    super_path: Path::default(),
    variance: SubtypingVariance::Invalid,
    is_property_modifier_violation: false,
  }
}

impl Default for SubtypingResult {
  fn default() -> Self {
    SubtypingResult {
      is_subtype: false,
      normalization_too_complex: false,
      is_cacheable: true,
      is_error_suppressing: false,
      errors: Default::default(),
      reasoning: SubtypingReasonings::new(k_empty_reasoning()),
      assumed_constraints: Default::default(),
      generic_bounds_mismatches: Default::default(),
    }
  }
}

impl DenseDefault for SubtypingResult {
  fn dense_default() -> Self {
    SubtypingResult::default()
  }
}

pub fn merge_reasonings(a: &SubtypingReasonings, b: &SubtypingReasonings) -> SubtypingReasonings {
  let mut result = SubtypingReasonings::new(k_empty_reasoning());

  for r in a.iter() {
    if r.variance == SubtypingVariance::Invariant {
      result.insert(r.clone());
    } else if r.variance == SubtypingVariance::Covariant
      || r.variance == SubtypingVariance::Contravariant
    {
      let inverse_reasoning = SubtypingReasoning {
        sub_path: r.sub_path.clone(),
        super_path: r.super_path.clone(),
        variance: if r.variance == SubtypingVariance::Covariant {
          SubtypingVariance::Contravariant
        } else {
          SubtypingVariance::Covariant
        },
        is_property_modifier_violation: false,
      };

      if b.contains(&inverse_reasoning) {
        result.insert(SubtypingReasoning {
          sub_path: r.sub_path.clone(),
          super_path: r.super_path.clone(),
          variance: SubtypingVariance::Invariant,
          is_property_modifier_violation: false,
        });
      } else {
        result.insert(r.clone());
      }
    }

    if result.size() >= FInt::LuauSubtypingReasoningLimit.get() as usize {
      return result;
    }
  }

  for r in b.iter() {
    if r.variance == SubtypingVariance::Invariant {
      result.insert(r.clone());
    } else if r.variance == SubtypingVariance::Covariant
      || r.variance == SubtypingVariance::Contravariant
    {
      let inverse_reasoning = SubtypingReasoning {
        sub_path: r.sub_path.clone(),
        super_path: r.super_path.clone(),
        variance: if r.variance == SubtypingVariance::Covariant {
          SubtypingVariance::Contravariant
        } else {
          SubtypingVariance::Covariant
        },
        is_property_modifier_violation: false,
      };

      if a.contains(&inverse_reasoning) {
        result.insert(SubtypingReasoning {
          sub_path: r.sub_path.clone(),
          super_path: r.super_path.clone(),
          variance: SubtypingVariance::Invariant,
          is_property_modifier_violation: false,
        });
      } else {
        result.insert(r.clone());
      }
    }

    if result.size() >= FInt::LuauSubtypingReasoningLimit.get() as usize {
      return result;
    }
  }

  result
}
