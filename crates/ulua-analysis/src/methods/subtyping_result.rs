//! `subtyping_result` 方法汇总：原先按 cpp 符号逐方法拆分的同前缀小文件合并至此，行为逐字保留。

use core::mem::{swap, take};

use ulua_common::fflag::LuauSubtypingSkipUnreadReasoning;

use crate::{
  enums::{
    subtyping_suppression_policy::SubtypingSuppressionPolicy, subtyping_variance::SubtypingVariance,
  },
  functions::{
    merge_reasonings::{k_empty_reasoning, merge_reasonings},
    prepend_reasoning_component::{ReasoningSide, prepend_component},
  },
  records::{
    path::Path, subtyping_reasoning::SubtypingReasoning, subtyping_result::SubtypingResult,
    type_error::TypeError,
  },
  type_aliases::{
    component::Component, constraint_v::ConstraintV, error_vec::ErrorVec,
    subtyping_reasonings::SubtypingReasonings,
  },
};

impl SubtypingResult {
  pub fn and_also(
    &mut self,
    mut other: SubtypingResult,
    policy: SubtypingSuppressionPolicy,
  ) -> &mut Self {
    // If the other result is not a subtype, we want to join all of its
    // reasonings to this one. If this result already has reasonings of its own,
    // those need to be attributed here whenever this _also_ failed.
    if !other.is_subtype {
      if self.is_subtype {
        swap(&mut self.reasoning, &mut other.reasoning);
      } else {
        // NOTE: This probably doesn't need to be two copies.
        self.reasoning = merge_reasonings(&self.reasoning, &other.reasoning);
      }
    }

    self.is_subtype &= other.is_subtype;

    if policy == SubtypingSuppressionPolicy::All {
      self.is_error_suppressing &= other.is_error_suppressing;
    } else {
      self.is_error_suppressing |= other.is_error_suppressing;
    }

    self.normalization_too_complex |= other.normalization_too_complex;
    self.is_cacheable &= other.is_cacheable;

    self.errors.extend(other.errors);
    self
      .generic_bounds_mismatches
      .extend(other.generic_bounds_mismatches);
    self.assumed_constraints.extend(other.assumed_constraints);

    self
  }
}

impl SubtypingResult {
  pub fn is_error_suppressing(&self) -> bool {
    self.is_error_suppressing
  }
}

impl SubtypingResult {
  pub fn is_subtype(&self) -> bool {
    self.is_subtype
  }
}

impl SubtypingResult {
  pub fn negate(result: &SubtypingResult) -> SubtypingResult {
    SubtypingResult {
      is_subtype: !result.is_subtype,
      normalization_too_complex: result.normalization_too_complex,
      ..result.clone()
    }
  }
}

impl SubtypingResult {
  pub fn or_else(&mut self, mut other: SubtypingResult) -> &mut Self {
    // If this result is a subtype, we do not join the reasoning lists. If this
    // result is not a subtype, but the other is a subtype, we want to _clear_
    // our reasoning list. If both results are not subtypes, we join the
    // reasoning lists.
    if !self.is_subtype {
      if other.is_subtype {
        self.reasoning.clear();
        self.assumed_constraints = take(&mut other.assumed_constraints);
      } else {
        self.reasoning = merge_reasonings(&self.reasoning, &other.reasoning);
        self.is_error_suppressing |= other.is_error_suppressing;
      }
    } else if other.is_subtype {
      // If the other result has assumed constraints, we drop ours (given
      // we represent a failed subtype) and then take the constraints of
      // the other check.
      self.assumed_constraints = take(&mut other.assumed_constraints);
    }

    self.is_subtype |= other.is_subtype;
    self.normalization_too_complex |= other.normalization_too_complex;
    self.is_cacheable &= other.is_cacheable;
    self.errors.extend(other.errors);
    self
      .generic_bounds_mismatches
      .extend(other.generic_bounds_mismatches);

    self
  }
}

impl SubtypingResult {
  pub fn with_assumed_constraint(&mut self, constraint: ConstraintV) -> &mut Self {
    self.assumed_constraints.push(constraint);
    self
  }
}

impl SubtypingResult {
  pub fn with_both_component(&mut self, component: Component) -> &mut Self {
    self
      .with_sub_component(component.clone())
      .with_super_component(component)
  }
}

impl SubtypingResult {
  pub fn with_error(&mut self, err: TypeError) -> &mut Self {
    self.errors.push(err);
    self
  }
}

impl SubtypingResult {
  pub fn with_errors(&mut self, err: &mut ErrorVec) -> &mut Self {
    for e in err.iter() {
      self.with_error(e.clone());
    }
    self
  }
}

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

// cpp `SubtypingResult& with_sub_component(Component)`——同形核心见
// [`crate::functions::prepend_reasoning_component`]（与
// `with_super_component` 仅路径侧不同）。

impl SubtypingResult {
  pub fn with_sub_component(&mut self, component: Component) -> &mut Self {
    if LuauSubtypingSkipUnreadReasoning.get() && self.is_subtype {
      return self;
    }
    prepend_component(&mut self.reasoning, component, ReasoningSide::Sub);
    self
  }
}

impl SubtypingResult {
  pub fn with_sub_path(&mut self, path: Path) -> &mut Self {
    if LuauSubtypingSkipUnreadReasoning.get() && self.is_subtype {
      return self;
    }
    if self.reasoning.empty() {
      self.reasoning.insert(SubtypingReasoning {
        sub_path: path,
        super_path: Path::default(),
        variance: SubtypingVariance::Covariant,
        is_property_modifier_violation: false,
      });
    } else {
      let mut updated = SubtypingReasonings::new(k_empty_reasoning());
      for r in self.reasoning.iter() {
        let mut r = r.clone();
        r.sub_path = path.append(&r.sub_path);
        updated.insert(r);
      }
      self.reasoning = updated;
    }

    self
  }
}

// cpp `SubtypingResult& with_super_component(Component)`——同形核心见
// [`crate::functions::prepend_reasoning_component`]（与
// `with_sub_component` 仅路径侧不同）。

impl SubtypingResult {
  pub fn with_super_component(&mut self, component: Component) -> &mut Self {
    if LuauSubtypingSkipUnreadReasoning.get() && self.is_subtype {
      return self;
    }
    prepend_component(&mut self.reasoning, component, ReasoningSide::Super);
    self
  }
}

impl SubtypingResult {
  pub fn with_super_path(&mut self, path: Path) -> &mut Self {
    if LuauSubtypingSkipUnreadReasoning.get() && self.is_subtype {
      return self;
    }
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
