use crate::{
  enums::{
    subtyping_suppression_policy::SubtypingSuppressionPolicy, subtyping_variance::SubtypingVariance,
  },
  functions::{
    assert_reasoning_valid_subtyping::assert_reasoning_valid, merge_reasonings::k_empty_reasoning,
  },
  methods::subtyping_is_contravariant_with_subtyping::IntoCovOperand,
  records::{
    path::Path, scope::Scope, subtyping::Subtyping, subtyping_environment::SubtypingEnvironment,
    subtyping_reasoning::SubtypingReasoning, subtyping_result::SubtypingResult,
  },
  type_aliases::subtyping_reasonings::SubtypingReasonings,
};

impl Subtyping {
  pub(crate) fn is_invariant_with_subtyping_environment_sub_ty_super_ty_not_null_scope<
    SubTy,
    SuperTy,
  >(
    &mut self,
    env: &mut SubtypingEnvironment,
    sub_ty: SubTy,
    super_ty: SuperTy,
    scope: *mut Scope,
  ) -> SubtypingResult
  where
    SubTy: IntoCovOperand,
    SuperTy: IntoCovOperand,
  {
    // C++: isCovariantWith(env, sub_ty, super_ty, scope).
    let mut result = self.covariant_dispatch(
      env,
      sub_ty.into_cov_operand(),
      super_ty.into_cov_operand(),
      scope,
    );
    let contra = self.is_contravariant_with_subtyping_environment_sub_ty_super_ty_not_null_scope(
      env, sub_ty, super_ty, scope,
    );
    result.and_also(contra, SubtypingSuppressionPolicy::Any);

    if result.reasoning.empty() {
      result.reasoning.insert(SubtypingReasoning {
        sub_path: Path::default(),
        super_path: Path::default(),
        variance: SubtypingVariance::Invariant,
        is_property_modifier_violation: false,
      });
    } else {
      let mut updated = SubtypingReasonings::new(k_empty_reasoning());
      for r in result.reasoning.iter() {
        let mut r = r.clone();
        r.variance = SubtypingVariance::Invariant;
        updated.insert(r);
      }
      result.reasoning = updated;
    }

    // `assertReasoningValid` is a debug-only no-op; pass `sub_ty` for both args to
    // satisfy the single `TID` parameter (see the contravariant port for details).
    assert_reasoning_valid(sub_ty, sub_ty, &result, self.builtin_types, self.arena);

    result
  }
}
