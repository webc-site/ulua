use alloc::vec::Vec;

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
    let mut result = SubtypingResult::ok();

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
      self.ice_reporter.as_ptr(),
    ) {
      lower_bound = subst_lower_bound;
    }

    if let Some(subst_upper_bound) = env.apply_mapped_generics(
      self.builtin_types,
      self.arena,
      upper_bound,
      self.ice_reporter.as_ptr(),
    ) {
      upper_bound = subst_upper_bound;
    }

    // C++ 中 normalize 失败返回空指针：视为可驻留（True）并标记归一化过于复杂
    // Safety: self.normalizer_ptr() 为 Subtyping 构造时接线的非空 *mut Normalizer（构造函数
    // 名带 `not_null_normalizer` 契约），比 self 长寿；try_normalize 经 &mut 借用归一化器，
    // 单线程串行无并发别名。
    let normalized = self.normalizer_mut().try_normalize(upper_bound);
    let (normalization_failed, res) = match normalized {
      Some(nt) => (false, {
        // Safety: 同上 normalizer 非空存活；上方 try_normalize 的 &mut 借用已随其返回结束，
        // 此处对同一指针的顺序再借用互不重叠。
        self.normalizer_mut().is_inhabited_normalized_type(&nt)
      }),
      None => (true, NormalizationResult::True),
    };

    if normalization_failed || res == NormalizationResult::HitLimits {
      result.normalization_too_complex = true;
    } else if res == NormalizationResult::False {
      result.is_subtype = false;
    }

    let mut bounds_env = SubtypingEnvironment {
      parent: env as *mut SubtypingEnvironment,
      mapped_generics: DenseHashMap::default(),
      mapped_generic_packs: MappedGenericEnvironment {
        frames: Vec::new(),
        current_scope_index: None,
      },
      substitutions: DenseHashMap::default(),
      seen_set_cache: DenseHashMap::default(),
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
      let lower_error_suppression = {
        // Safety: self.normalizer_ptr() 是非空存活的接线字段（构造时注入，整个 check 期间存活），
        // should_suppress_errors 依契约仅对其进行只读归一化判定，单线程无并发/别名写。
        unsafe { should_suppress_errors(self.normalizer_ptr(), lower_bound) }
      };
      let upper_error_suppression = {
        // Safety: 同上，normalizer 非空存活；lower/upper 两次调用顺序进行，借用互不重叠。
        unsafe { should_suppress_errors(self.normalizer_ptr(), upper_bound) }
      };
      match lower_error_suppression
        .or_else(&upper_error_suppression)
        .value
      {
        Value::Suppress => {}
        // C++：无法证明是错误压制时（含归一化失败），原样落入报 mismatch
        Value::NormalizationFailed | Value::DoNotSuppress => {
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
