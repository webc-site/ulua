use alloc::vec::Vec;
use core::ptr::null;

use ulua_common::records::dense_hash_map::DenseHashMap;

use crate::{
  enums::{
    normalization_result::NormalizationResult,
    subtyping_suppression_policy::SubtypingSuppressionPolicy, value::Value,
  },
  functions::should_suppress_errors_type_utils::should_suppress_errors,
  records::{
    generic_bounds::GenericBounds, generic_bounds_mismatch::GenericBoundsMismatch,
    intersection_builder::IntersectionBuilder,
    mapped_generic_environment::MappedGenericEnvironment, scope::Scope, subtyping::Subtyping,
    subtyping_environment::SubtypingEnvironment, subtyping_result::SubtypingResult,
    union_builder::UnionBuilder,
  },
};
impl Subtyping {
  pub fn subtyping_check_generic_bounds(
    &mut self,
    bounds: &GenericBounds,
    env: &mut SubtypingEnvironment,
    scope: *mut Scope,
    generic_name: &str,
  ) -> SubtypingResult {
    let mut result = SubtypingResult {
      is_subtype: true,
      normalization_too_complex: false,
      is_cacheable: true,
      is_error_suppressing: false,
      errors: Default::default(),
      reasoning: Default::default(),
      assumed_constraints: Default::default(),
      generic_bounds_mismatches: Default::default(),
    };

    let mut aggregate_lower_bound = UnionBuilder::new(self.arena, self.builtin_types);
    aggregate_lower_bound.reserve(bounds.lower_bound.size());
    for &t in &bounds.lower_bound.order {
      if let Some(mapped_bounds) = env.mapped_generics.find(&t)
        && mapped_bounds.is_empty()
      {
        continue;
      }
      aggregate_lower_bound.add(t);
    }
    let mut lower_bound = aggregate_lower_bound.build();

    let mut aggregate_upper_bound = IntersectionBuilder::new(self.arena, self.builtin_types);
    aggregate_upper_bound.reserve(bounds.upper_bound.size());
    for &t in &bounds.upper_bound.order {
      if let Some(mapped_bounds) = env.mapped_generics.find(&t)
        && mapped_bounds.is_empty()
      {
        continue;
      }
      aggregate_upper_bound.add(t);
    }
    let mut upper_bound = aggregate_upper_bound.build();

    if let Some(subst_lower_bound) = env.apply_mapped_generics(
      self.builtin_types,
      self.arena,
      lower_bound,
      self.ice_reporter,
    ) {
      lower_bound = subst_lower_bound;
    }

    if let Some(subst_upper_bound) = env.apply_mapped_generics(
      self.builtin_types,
      self.arena,
      upper_bound,
      self.ice_reporter,
    ) {
      upper_bound = subst_upper_bound;
    }

    // `Normalizer::normalize` here returns a non-nullable `Arc<NormalizedType>`,
    // so the C++ `!nt` (normalization-failed → nullptr) branch is not
    // representable through this signature; the value is always present, and
    // `res` is therefore always the inhabitedness result.
    let nt = unsafe { (*self.normalizer).normalize(upper_bound) };
    let res = unsafe { (*self.normalizer).is_inhabited_normalized_type(&nt) };

    if res == NormalizationResult::HitLimits {
      result.normalization_too_complex = true;
    } else if res == NormalizationResult::False {
      result.is_subtype = false;
    }

    let mut bounds_env = SubtypingEnvironment {
      parent: env as *mut SubtypingEnvironment,
      mapped_generics: DenseHashMap::new(null()),
      mapped_generic_packs: MappedGenericEnvironment {
        frames: Vec::new(),
        current_scope_index: None,
      },
      substitutions: DenseHashMap::new(null()),
      seen_set_cache: DenseHashMap::new((null(), null())),
      iteration_count: 0,
    };
    let mut bounds_result = self
      .is_covariant_with_subtyping_environment_type_id_type_id_not_null_scope(
        &mut bounds_env,
        lower_bound,
        upper_bound,
        scope,
      );
    bounds_result.reasoning.clear();

    if res == NormalizationResult::False {
      result
        .generic_bounds_mismatches
        .push(GenericBoundsMismatch::new(
          generic_name,
          bounds.lower_bound.clone(),
          bounds.upper_bound.clone(),
        ));
    } else if !bounds_result.is_subtype {
      // Check if the bounds are error suppressing before reporting a mismatch
      let lower_error_suppression =
        { unsafe { should_suppress_errors(self.normalizer, lower_bound) } };
      let upper_error_suppression =
        { unsafe { should_suppress_errors(self.normalizer, upper_bound) } };
      match lower_error_suppression
        .or_else(&upper_error_suppression)
        .value
      {
        Value::Suppress => {}
        Value::NormalizationFailed => {
          result
            .generic_bounds_mismatches
            .push(GenericBoundsMismatch::new(
              generic_name,
              bounds.lower_bound.clone(),
              bounds.upper_bound.clone(),
            ));
        }
        Value::DoNotSuppress => {
          result
            .generic_bounds_mismatches
            .push(GenericBoundsMismatch::new(
              generic_name,
              bounds.lower_bound.clone(),
              bounds.upper_bound.clone(),
            ));
        }
      }
    }

    result.and_also(bounds_result, SubtypingSuppressionPolicy::Any);

    result
  }
}
