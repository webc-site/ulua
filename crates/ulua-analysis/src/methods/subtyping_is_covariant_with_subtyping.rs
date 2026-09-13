use core::mem::zeroed;

use ulua_common::{DFInt, FFlag, FInt};

use crate::{
  enums::{
    subtyping_suppression_policy::SubtypingSuppressionPolicy, table_state::TableState,
    type_field::TypeField,
  },
  functions::{
    assert_reasoning_valid_subtyping::assert_reasoning_valid, follow_type::follow_type_id,
    get_2::get2, get_type_alt_j::get_type_id, subsumes_scope::subsumes,
  },
  methods::subtyping_bind_generic::dense_hash_map_find_no_default,
  records::{
    any_type::AnyType,
    blocked_type::BlockedType,
    error_type::ErrorType,
    extern_type::ExternType,
    free_type::FreeType,
    function_type::FunctionType,
    generic_type::GenericType,
    intersection_type::IntersectionType,
    metatable_type::MetatableType,
    negation_type::NegationType,
    never_type::NeverType,
    non_exceptional_recursion_limiter::NonExceptionalRecursionLimiter,
    primitive_type::{PrimitiveType, Type as PrimType},
    scope::Scope,
    singleton_type::SingletonType,
    subtype_constraint::SubtypeConstraint,
    subtyping::Subtyping,
    subtyping_environment::SubtypingEnvironment,
    subtyping_result::SubtypingResult,
    table_type::TableType,
    type_function_instance_type::TypeFunctionInstanceType,
    union_type::UnionType,
    unknown_type::UnknownType,
  },
  type_aliases::{component::Component, constraint_v::ConstraintV, type_id::TypeId},
};
impl Subtyping {
  /// # Safety
  /// 调用方须保证 `scope` 等裸指针参数有效，且满足 C++ 原实现的调用契约。
  pub(crate) fn is_covariant_with_subtyping_environment_type_id_type_id_not_null_scope(
    &mut self,
    env: &mut SubtypingEnvironment,
    sub_ty: TypeId,
    super_ty: TypeId,
    scope: *mut Scope,
  ) -> SubtypingResult {
    // NonExceptionalRecursionLimiter nerl(&normalizer->sharedState->counters.recursionCount);
    let recursion_count_ptr =
      unsafe { &mut (*(*self.normalizer).shared_state).counters.recursion_count as *mut i32 };
    let mut nerl = NonExceptionalRecursionLimiter {
      base: unsafe { zeroed() },
      native_stack_guard: unsafe { zeroed() },
    };
    nerl.non_exceptional_recursion_limiter_non_exceptional_recursion_limiter(recursion_count_ptr);
    if !nerl.is_ok(DFInt::LuauSubtypingRecursionLimit.get()) {
      return SubtypingResult {
        is_subtype: false,
        normalization_too_complex: true,
        ..Default::default()
      };
    }
    let _nerl = nerl;

    env.iteration_count += 1;
    let iteration_limit = FInt::LuauSubtypingIterationLimit.get();
    if iteration_limit > 0 && env.iteration_count >= iteration_limit {
      return SubtypingResult {
        is_subtype: false,
        normalization_too_complex: true,
        ..Default::default()
      };
    }

    let mut sub_ty = follow_type_id(sub_ty);
    let super_ty = follow_type_id(super_ty);

    if let Some(sub_it) = env.try_find_substitution(sub_ty)
      && !sub_it.is_null()
    {
      sub_ty = sub_it;
    }

    if let Some(super_it) = env.try_find_substitution(super_ty)
      && !super_it.is_null()
    {
      // NOTE: This faithfully mirrors the C++, which assigns the
      // super substitution back into `sub_ty` (apparent upstream typo).
      sub_ty = super_it;
    }

    if let Some(cached_result) = self.result_cache.find(&(sub_ty, super_ty)) {
      return cached_result.clone();
    }

    if let Some(cached_result) = env.try_find_subtyping_result((sub_ty, super_ty)) {
      return cached_result.clone();
    }

    // TODO: Do we care about returning a proof that this is error-suppressing?
    if sub_ty == super_ty {
      return SubtypingResult {
        is_subtype: true,
        ..Default::default()
      };
    }

    let type_pair = (sub_ty, super_ty);
    // seenTypes.insert(typePair) — Luau::Set semantics over a DenseHashMap<_, bool>.
    let fresh = {
      let entry = self.seen_types.get_or_insert(type_pair);
      let fresh = !*entry;
      if fresh {
        *entry = true;
      }
      fresh
    };
    if !fresh {
      // We've encountered a cycle; conservatively assume subtype and refuse to
      // cache anything that touches the cycle.
      let res = SubtypingResult {
        is_subtype: true,
        is_cacheable: false,
        ..Default::default()
      };

      *env.seen_set_cache.get_or_insert(type_pair) = res.clone();

      return res;
    }

    // ScopedSeenSet ssp{seenTypes, typePair}; — re-insert (no-op) and erase on
    // every exit below. With no `erase`, mirror `Luau::Set::erase` by clearing
    // the slot's bool before each return point.

    let mut result = SubtypingResult::default();

    let pair_ff = get2::<FreeType, FreeType, _>(sub_ty, super_ty);
    if !pair_ff.first.is_null() {
      // Any two free types are potentially subtypes of one another because
      // both of them could be narrowed to never.
      result = SubtypingResult {
        is_subtype: true,
        ..Default::default()
      };
      result.with_assumed_constraint(ConstraintV::Subtype(SubtypeConstraint {
        sub_type: sub_ty,
        super_type: super_ty,
      }));
    } else if let Some(super_free) = get_type_id::<FreeType>(super_ty) {
      // FIXME CLI-185582.
      result = self.is_covariant_with_subtyping_environment_type_id_type_id_not_null_scope(
        env,
        sub_ty,
        super_free.upper_bound,
        scope,
      );

      if result.is_subtype {
        result.with_assumed_constraint(ConstraintV::Subtype(SubtypeConstraint {
          sub_type: sub_ty,
          super_type: super_ty,
        }));
      }
    } else if let Some(sub_free) = get_type_id::<FreeType>(sub_ty) {
      // FIXME CLI-185582.
      if self
        .is_covariant_with_subtyping_environment_type_id_type_id_not_null_scope(
          env,
          sub_free.lower_bound,
          super_ty,
          scope,
        )
        .is_subtype
      {
        result = SubtypingResult {
          is_subtype: true,
          ..Default::default()
        };
        result.with_assumed_constraint(ConstraintV::Subtype(SubtypeConstraint {
          sub_type: sub_ty,
          super_type: super_ty,
        }));
      } else {
        result = SubtypingResult {
          is_subtype: false,
          ..Default::default()
        };
      }
    } else if get_type_id::<BlockedType>(sub_ty).is_some()
      || get_type_id::<BlockedType>(super_ty).is_some()
    {
      result = SubtypingResult {
        is_subtype: true,
        ..Default::default()
      };
      result.with_assumed_constraint(ConstraintV::Subtype(SubtypeConstraint {
        sub_type: sub_ty,
        super_type: super_ty,
      }));
    }
    // TODO: These branches are entirely incorrect (per upstream).
    else if let Some(sub_generic) =
      get_type_id::<GenericType>(sub_ty).filter(|g| subsumes(g.scope, scope))
    {
      let _ = sub_generic;
      self.seen_set_erase(type_pair);
      return self.is_covariant_with_subtyping_environment_type_id_type_id_not_null_scope(
        env,
        unsafe { (*self.builtin_types).never_type },
        super_ty,
        scope,
      );
    } else if let Some(super_generic) =
      get_type_id::<GenericType>(super_ty).filter(|g| subsumes(g.scope, scope))
    {
      let _ = super_generic;
      self.seen_set_erase(type_pair);
      return self.is_covariant_with_subtyping_environment_type_id_type_id_not_null_scope(
        env,
        sub_ty,
        unsafe { (*self.builtin_types).unknown_type },
        scope,
      );
    } else if get_type_id::<AnyType>(super_ty).is_some()
      || (get_type_id::<AnyType>(sub_ty).is_some()
        && get_type_id::<UnknownType>(super_ty).is_some())
    {
      // any = err | unknown.
      result = SubtypingResult {
        is_subtype: true,
        ..Default::default()
      };
    } else if get_type_id::<AnyType>(sub_ty).is_some() {
      // any = unknown | error, so we rewrite this to match.
      result = self.is_covariant_with_subtyping_environment_type_id_type_id_not_null_scope(
        env,
        unsafe { (*self.builtin_types).unknown_type },
        super_ty,
        scope,
      );
      let err_part = self.is_covariant_with_subtyping_environment_type_id_type_id_not_null_scope(
        env,
        unsafe { (*self.builtin_types).error_type },
        super_ty,
        scope,
      );
      result.and_also(err_part, SubtypingSuppressionPolicy::Any);
      result.is_error_suppressing = true;
    } else if get_type_id::<UnknownType>(super_ty).is_some()
      && get_type_id::<UnionType>(sub_ty).is_none()
      && get_type_id::<IntersectionType>(sub_ty).is_none()
    {
      let error_suppressing = get_type_id::<ErrorType>(sub_ty).is_some();
      result.is_subtype = !error_suppressing;
      result.is_error_suppressing = error_suppressing;
    } else if get_type_id::<NeverType>(sub_ty).is_some() {
      result = SubtypingResult {
        is_subtype: true,
        ..Default::default()
      };
    } else if get_type_id::<ErrorType>(super_ty).is_some() {
      result = SubtypingResult {
        is_subtype: false,
        ..Default::default()
      };
    } else if get_type_id::<ErrorType>(sub_ty).is_some() {
      result = SubtypingResult {
        is_subtype: true,
        ..Default::default()
      };
      result.is_error_suppressing = true;
    } else if get_type_id::<TypeFunctionInstanceType>(sub_ty).is_some() {
      let mut sub_type_function_instance = get_type_id::<TypeFunctionInstanceType>(sub_ty);
      let mut mapped_generics_applied = false;
      if let Some(subst_sub_ty) =
        env.apply_mapped_generics(self.builtin_types, self.arena, sub_ty, self.ice_reporter)
      {
        mapped_generics_applied = subst_sub_ty != sub_ty;
        sub_type_function_instance = get_type_id::<TypeFunctionInstanceType>(subst_sub_ty);
      }

      result = self
        .is_covariant_with_subtyping_environment_type_function_instance_type_type_id_not_null_scope(
          env,
          sub_type_function_instance.unwrap(),
          super_ty,
          scope,
        );
      result.is_cacheable = !mapped_generics_applied;
    } else if get_type_id::<TypeFunctionInstanceType>(super_ty).is_some() {
      let mut super_type_function_instance = get_type_id::<TypeFunctionInstanceType>(super_ty);
      let mut mapped_generics_applied = false;
      if let Some(subst_super_ty) =
        env.apply_mapped_generics(self.builtin_types, self.arena, super_ty, self.ice_reporter)
      {
        mapped_generics_applied = subst_super_ty != super_ty;
        super_type_function_instance = get_type_id::<TypeFunctionInstanceType>(subst_super_ty);
      }

      result = self
        .is_covariant_with_subtyping_environment_type_id_type_function_instance_type_not_null_scope(
          env,
          sub_ty,
          super_type_function_instance.unwrap(),
          scope,
        );
      result.is_cacheable = !mapped_generics_applied;
    } else if get_type_id::<GenericType>(sub_ty).is_some()
      || get_type_id::<GenericType>(super_ty).is_some()
    {
      let sub_has_bounds = dense_hash_map_find_no_default(&env.mapped_generics, &sub_ty)
        .is_some_and(|b| !b.is_empty());
      let super_has_bounds = dense_hash_map_find_no_default(&env.mapped_generics, &super_ty)
        .is_some_and(|b| !b.is_empty());
      if sub_has_bounds || super_has_bounds {
        let ok = self.bind_generic(env, sub_ty, super_ty);
        result.is_subtype = ok;
        result.is_cacheable = false;
      }
    } else if let Some(sub_union) = get_type_id::<UnionType>(sub_ty) {
      result = self.is_covariant_with_subtyping_environment_union_type_type_id_not_null_scope(
        env, sub_union, super_ty, scope,
      );
    } else if let Some(super_union) = get_type_id::<UnionType>(super_ty) {
      result = self.is_covariant_with_subtyping_environment_type_id_union_type_not_null_scope(
        env,
        sub_ty,
        super_union,
        scope,
      );
      if !result.is_subtype && !result.normalization_too_complex {
        result = self.try_semantic_subtyping(env, sub_ty, super_ty, scope, &mut result);
      }
    } else if let Some(super_intersection) = get_type_id::<IntersectionType>(super_ty) {
      result = self
        .is_covariant_with_subtyping_environment_type_id_intersection_type_not_null_scope(
          env,
          sub_ty,
          super_intersection,
          scope,
        );
    } else if let Some(sub_intersection) = get_type_id::<IntersectionType>(sub_ty) {
      result = self
        .is_covariant_with_subtyping_environment_intersection_type_type_id_not_null_scope(
          env,
          sub_intersection,
          super_ty,
          scope,
        );
      if !result.is_subtype && !result.normalization_too_complex {
        result = self.try_semantic_subtyping(env, sub_ty, super_ty, scope, &mut result);
      }
    } else if let p = get2::<NegationType, NegationType, _>(sub_ty, super_ty)
      && !p.first.is_null()
    {
      let p = get2::<NegationType, NegationType, _>(sub_ty, super_ty);
      // We use `isContravariantWith` here in order to make sure that the
      // type paths still look coherent.
      result = self.is_contravariant_with_subtyping_environment_sub_ty_super_ty_not_null_scope(
        env,
        unsafe { (*p.first).ty },
        unsafe { (*p.second).ty },
        scope,
      );
      result.with_both_component(Component::TypeField(TypeField::Negated));
    } else if let Some(sub_negation) = get_type_id::<NegationType>(sub_ty) {
      result = self.is_covariant_with_subtyping_environment_negation_type_type_id_not_null_scope(
        env,
        sub_negation,
        super_ty,
        scope,
      );
      if !result.is_subtype && !result.normalization_too_complex {
        result = self.try_semantic_subtyping(env, sub_ty, super_ty, scope, &mut result);
      }
    } else if let Some(super_negation) = get_type_id::<NegationType>(super_ty) {
      result = self.is_covariant_with_subtyping_environment_type_id_negation_type_not_null_scope(
        env,
        sub_ty,
        super_negation,
        scope,
      );
      if !result.is_subtype && !result.normalization_too_complex {
        result = self.try_semantic_subtyping(env, sub_ty, super_ty, scope, &mut result);
      }
    } else if let p = get2::<PrimitiveType, PrimitiveType, _>(sub_ty, super_ty)
      && !p.first.is_null()
    {
      let p = get2::<PrimitiveType, PrimitiveType, _>(sub_ty, super_ty);
      result = self
        .is_covariant_with_subtyping_environment_primitive_type_primitive_type_not_null_scope(
          env,
          unsafe { &*p.first },
          unsafe { &*p.second },
          scope,
        );
    } else if let p = get2::<SingletonType, PrimitiveType, _>(sub_ty, super_ty)
      && !p.first.is_null()
    {
      let p = get2::<SingletonType, PrimitiveType, _>(sub_ty, super_ty);
      result = self
        .is_covariant_with_subtyping_environment_singleton_type_primitive_type_not_null_scope(
          env,
          unsafe { &*p.first },
          unsafe { &*p.second },
          scope,
        );
    } else if let p = get2::<SingletonType, SingletonType, _>(sub_ty, super_ty)
      && !p.first.is_null()
    {
      let p = get2::<SingletonType, SingletonType, _>(sub_ty, super_ty);
      result = self
        .is_covariant_with_subtyping_environment_singleton_type_singleton_type_not_null_scope(
          env,
          unsafe { &*p.first },
          unsafe { &*p.second },
          scope,
        );
    } else if let p = get2::<FunctionType, PrimitiveType, _>(sub_ty, super_ty)
      && !p.first.is_null()
    {
      let p = get2::<FunctionType, PrimitiveType, _>(sub_ty, super_ty);
      let _sub_function = p.first;
      let super_primitive = p.second;
      result.is_subtype = unsafe { (*super_primitive).r#type == PrimType::Function };
    } else if let p = get2::<FunctionType, FunctionType, _>(sub_ty, super_ty)
      && !p.first.is_null()
    {
      let p = get2::<FunctionType, FunctionType, _>(sub_ty, super_ty);
      result = unsafe {
        self.is_covariant_with_subtyping_environment_function_type_function_type_not_null_scope(
          env, &*p.first, &*p.second, scope,
        )
      };
    } else if let p = get2::<TableType, TableType, _>(sub_ty, super_ty)
      && !p.first.is_null()
    {
      let p = get2::<TableType, TableType, _>(sub_ty, super_ty);
      let force_covariant_test =
        !self.unique_types.is_null() && unsafe { (*self.unique_types).contains(&sub_ty) };
      result = if FFlag::LuauSubtypingTablesHasBetterErrorSuppression.get() {
        self.is_covariant_with_subtyping_environment_table_type_table_type_bool_not_null_scope(
          env,
          unsafe { &*p.first },
          unsafe { &*p.second },
          force_covariant_test,
          scope,
        )
      } else {
        self.is_covariant_with_deprecated(
          env,
          unsafe { &*p.first },
          unsafe { &*p.second },
          force_covariant_test,
          scope,
        )
      };
      if result.is_subtype
        && unsafe { (*p.first).indexer.is_none() }
        && unsafe { (*p.second).indexer.is_some() }
        && unsafe { (*p.first).state != TableState::Sealed }
      {
        // FIXME CLI-182960.
        result.with_assumed_constraint(ConstraintV::Subtype(SubtypeConstraint {
          sub_type: sub_ty,
          super_type: super_ty,
        }));
      }
    } else if let p = get2::<MetatableType, MetatableType, _>(sub_ty, super_ty)
      && !p.first.is_null()
    {
      let p = get2::<MetatableType, MetatableType, _>(sub_ty, super_ty);
      result = self
        .is_covariant_with_subtyping_environment_metatable_type_metatable_type_not_null_scope(
          env,
          unsafe { &*p.first },
          unsafe { &*p.second },
          scope,
        );
    } else if let p = get2::<MetatableType, TableType, _>(sub_ty, super_ty)
      && !p.first.is_null()
    {
      let p = get2::<MetatableType, TableType, _>(sub_ty, super_ty);
      result = self
        .is_covariant_with_subtyping_environment_metatable_type_table_type_not_null_scope(
          env,
          unsafe { &*p.first },
          unsafe { &*p.second },
          scope,
        );
    } else if FFlag::LuauTableFreezeCheckIsSubtype.get() && {
      let p = get2::<MetatableType, PrimitiveType, _>(sub_ty, super_ty);
      !p.first.is_null()
    } {
      let p = get2::<MetatableType, PrimitiveType, _>(sub_ty, super_ty);
      result = self
        .is_covariant_with_subtyping_environment_metatable_type_primitive_type_not_null_scope(
          env,
          unsafe { &*p.first },
          unsafe { &*p.second },
          scope,
        );
    } else if let p = get2::<ExternType, ExternType, _>(sub_ty, super_ty)
      && !p.first.is_null()
    {
      let p = get2::<ExternType, ExternType, _>(sub_ty, super_ty);
      result = self.is_covariant_with_subtyping_environment_extern_type_extern_type_not_null_scope(
        env,
        unsafe { &*p.first },
        unsafe { &*p.second },
        scope,
      );
    } else if let p = get2::<ExternType, TableType, _>(sub_ty, super_ty)
      && !p.first.is_null()
    {
      let p = get2::<ExternType, TableType, _>(sub_ty, super_ty);
      result = self
                .is_covariant_with_subtyping_environment_type_id_extern_type_type_id_table_type_not_null_scope(
                    env,
                    sub_ty,
                    unsafe { &*p.first },
                    super_ty,
                    unsafe { &*p.second },
                    scope,
                );
    } else if let p = get2::<TableType, PrimitiveType, _>(sub_ty, super_ty)
      && !p.first.is_null()
    {
      let p = get2::<TableType, PrimitiveType, _>(sub_ty, super_ty);
      result = self
        .is_covariant_with_subtyping_environment_table_type_primitive_type_not_null_scope(
          env,
          unsafe { &*p.first },
          unsafe { &*p.second },
          scope,
        );
    } else if let p = get2::<PrimitiveType, TableType, _>(sub_ty, super_ty)
      && !p.first.is_null()
    {
      let p = get2::<PrimitiveType, TableType, _>(sub_ty, super_ty);
      result = self
        .is_covariant_with_subtyping_environment_primitive_type_table_type_not_null_scope(
          env,
          unsafe { &*p.first },
          unsafe { &*p.second },
          scope,
        );
    } else if let p = get2::<SingletonType, TableType, _>(sub_ty, super_ty)
      && !p.first.is_null()
    {
      let p = get2::<SingletonType, TableType, _>(sub_ty, super_ty);
      result = self
        .is_covariant_with_subtyping_environment_singleton_type_table_type_not_null_scope(
          env,
          unsafe { &*p.first },
          unsafe { &*p.second },
          scope,
        );
    }

    assert_reasoning_valid(sub_ty, super_ty, &result, self.builtin_types, self.arena);

    // ScopedSeenSet destructor — erase the pair before returning.
    self.seen_set_erase(type_pair);

    self.cache(env, result, sub_ty, super_ty)
  }

  /// Mirror `Luau::Set::erase` over `seen_types` (a `DenseHashMap<_, bool>`): set
  /// the slot's value to `false` rather than removing the key.
  #[inline]
  fn seen_set_erase(&mut self, type_pair: (TypeId, TypeId)) {
    let entry = self.seen_types.get_or_insert(type_pair);
    if *entry {
      *entry = false;
    }
  }
}
