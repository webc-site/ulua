use alloc::{string::ToString, sync::Arc, vec::Vec};
use core::{cmp::min, mem::swap, ptr::null};

use ulua_common::{dfint, fflag, fint, macros::luau_assert::LUAU_ASSERT};

use crate::{
  enums::{
    early_exit::EarlyExit, pack_field::PackField,
    subtyping_suppression_policy::SubtypingSuppressionPolicy,
    subtyping_variance::SubtypingVariance, table_state::TableState, type_field::TypeField,
    variant::Variant,
  },
  functions::{
    assert_reasoning_valid_subtyping::assert_reasoning_valid,
    begin_type::{begin_intersection_type, begin_union_type},
    flatten_type_pack::flatten_type_pack_id,
    follow_type, follow_type_pack,
    get_2::get2,
    get_metatable_type::get_metatable_type_id_not_null_builtin_types,
    get_singleton_type::get_singleton_type,
    get_type, get_type_pack,
    is_subclass_type::is_subclass_extern_type_extern_type,
    is_subtype_normalized_string::is_subtype_normalized_string,
    lookup_extern_type_prop::lookup_extern_type_prop,
    merge_reasonings::k_empty_reasoning,
    subsumes_scope::subsumes,
  },
  methods::{
    path_builder_build::PathBuilderBuild,
    path_builder_mt::PathBuilderMt,
    subtyping_bind_generic::dense_hash_map_find_no_default,
    subtyping_is_covariant_with_super_tail::SuperTailCovariantArgs,
    subtyping_is_sub_tail_covariant_with::SubTailCovariantArgs,
    subtyping_path_components::{index_result_component, path_property},
  },
  records::{
    any_type::AnyType,
    blocked_type::BlockedType,
    boolean_singleton::BooleanSingleton,
    extern_type::ExternType,
    free_type::FreeType,
    free_type_pack::FreeTypePack,
    function_type::FunctionType,
    generic_bounds::GenericBounds,
    generic_type::GenericType,
    generic_type_count_mismatch::GenericTypeCountMismatch,
    generic_type_pack::GenericTypePack,
    generic_type_pack_count_mismatch::GenericTypePackCountMismatch,
    index::Index,
    intersection_type::IntersectionType,
    metatable_type::MetatableType,
    negation_type::NegationType,
    never_type::NeverType,
    non_exceptional_recursion_limiter::NonExceptionalRecursionLimiter,
    normalized_extern_type::NormalizedExternType,
    normalized_function_type::NormalizedFunctionType,
    normalized_string_type::NormalizedStringType,
    normalized_type::NormalizedType,
    nothing::Nothing,
    pack_subtype_constraint::PackSubtypeConstraint,
    path::Path,
    path_builder::PathBuilder,
    primitive_type::{PrimitiveType, Type as PrimType},
    property_type::Property,
    property_type_path::Property as PathProperty,
    reduction::Reduction,
    scope::Scope,
    singleton_type::SingletonType,
    string_singleton::StringSingleton,
    subtype_constraint::SubtypeConstraint,
    subtyping::Subtyping,
    subtyping_environment::SubtypingEnvironment,
    subtyping_reasoning::SubtypingReasoning,
    subtyping_result::SubtypingResult,
    table_indexer::TableIndexer,
    table_type::TableType,
    type_error::TypeError,
    type_function_instance_type::TypeFunctionInstanceType,
    type_ids::TypeIds,
    unexpected_type_pack_in_subtyping::UnexpectedTypePackInSubtyping,
    union_type::UnionType,
    unknown_type::UnknownType,
    variadic_type_pack::VariadicTypePack,
  },
  type_aliases::{
    component::Component, constraint_v::ConstraintV, error_type::ErrorType,
    error_type_pack::ErrorTypePack, module_name_type::ModuleName,
    subtyping_reasonings::SubtypingReasonings, type_error_data::TypeErrorData, type_id::TypeId,
    type_pack_id::TypePackId,
  },
};

impl Subtyping {
  pub(crate) fn is_covariant_with_subtyping_environment_type_id_type_id_not_null_scope(
    &mut self,
    env: &mut SubtypingEnvironment,
    sub_ty: TypeId,
    super_ty: TypeId,
    scope: *mut Scope,
  ) -> SubtypingResult {
    // NonExceptionalRecursionLimiter nerl(&normalizer->sharedState->counters.recursionCount);
    // 计数器借 `&mut` 交给 RAII 限深守卫（守卫内部降为 Handle 别名，借用随构造语句结束释放）。
    let _nerl = NonExceptionalRecursionLimiter::new(
      &mut self
        .normalizer_mut()
        .shared_state_mut()
        .counters
        .recursion_count,
    );
    if !_nerl.is_ok(dfint::LuauSubtypingRecursionLimit.get()) {
      return SubtypingResult::too_complex();
    }

    env.iteration_count += 1;
    let iteration_limit = fint::LuauSubtypingIterationLimit.get();
    if iteration_limit > 0 && env.iteration_count >= iteration_limit {
      return SubtypingResult::too_complex();
    }

    let mut sub_ty = follow_type::follow(sub_ty);
    let super_ty = follow_type::follow(super_ty);

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
      return SubtypingResult::ok();
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
      let res = SubtypingResult::uncacheable_ok();

      *env.seen_set_cache.get_or_insert(type_pair) = res.clone();

      return res;
    }

    // ScopedSeenSet ssp{seenTypes, typePair}; — erased on every exit below.

    let mut result = SubtypingResult::default();

    // Any two free types are potentially subtypes of one another because
    // both of them could be narrowed to never.
    if get2::<FreeType, FreeType, _>(sub_ty, super_ty).is_some() {
      result = SubtypingResult::ok();
      result.with_assumed_constraint(ConstraintV::Subtype(SubtypeConstraint {
        sub_type: sub_ty,
        super_type: super_ty,
      }));
    } else if let Some(super_free) = get_type::get::<FreeType>(super_ty) {
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
    } else if let Some(sub_free) = get_type::get::<FreeType>(sub_ty) {
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
        result = SubtypingResult::ok();
        result.with_assumed_constraint(ConstraintV::Subtype(SubtypeConstraint {
          sub_type: sub_ty,
          super_type: super_ty,
        }));
      } else {
        result = SubtypingResult::fail();
      }
    } else if get_type::get::<BlockedType>(sub_ty).is_some()
      || get_type::get::<BlockedType>(super_ty).is_some()
    {
      result = SubtypingResult::ok();
      result.with_assumed_constraint(ConstraintV::Subtype(SubtypeConstraint {
        sub_type: sub_ty,
        super_type: super_ty,
      }));
    }
    // TODO: These branches are entirely incorrect (per upstream).
    else if get_type::get::<GenericType>(sub_ty).is_some_and(|g| subsumes(g.scope, scope)) {
      self.seen_set_erase(type_pair);
      return self.is_covariant_with_subtyping_environment_type_id_type_id_not_null_scope(
        env,
        // SAFETY: builtin_types 为构造期传入、检查期内存活的 BuiltinTypes 指针（C++ 引用
        // 形参直译），此处仅只读读取 never_type 句柄。
        { self.builtin_types.get().never_type },
        super_ty,
        scope,
      );
    } else if get_type::get::<GenericType>(super_ty).is_some_and(|g| subsumes(g.scope, scope)) {
      self.seen_set_erase(type_pair);
      return self.is_covariant_with_subtyping_environment_type_id_type_id_not_null_scope(
        env,
        sub_ty,
        // SAFETY: 同一 builtin_types 存活不变量，此处读 unknown_type。
        { self.builtin_types.get().unknown_type },
        scope,
      );
    } else if get_type::get::<AnyType>(super_ty).is_some()
      || (get_type::get::<AnyType>(sub_ty).is_some()
        && get_type::get::<UnknownType>(super_ty).is_some())
    {
      // any = err | unknown.
      result = SubtypingResult::ok();
    } else if get_type::get::<AnyType>(sub_ty).is_some() {
      // any = unknown | error, so we rewrite this to match.
      result = self.is_covariant_with_subtyping_environment_type_id_type_id_not_null_scope(
        env,
        // SAFETY: builtin_types 指向存活只读的 BuiltinTypes，仅取 unknown_type 句柄。
        { self.builtin_types.get().unknown_type },
        super_ty,
        scope,
      );
      let err_part = self.is_covariant_with_subtyping_environment_type_id_type_id_not_null_scope(
        env,
        // SAFETY: 上一处同一不变量；error_type 亦为只读句柄读取。
        { self.builtin_types.get().error_type },
        super_ty,
        scope,
      );
      result.and_also(err_part, SubtypingSuppressionPolicy::Any);
      result.is_error_suppressing = true;
    } else if get_type::get::<UnknownType>(super_ty).is_some()
      && get_type::get::<UnionType>(sub_ty).is_none()
      && get_type::get::<IntersectionType>(sub_ty).is_none()
    {
      let error_suppressing = get_type::get::<ErrorType>(sub_ty).is_some();
      result.is_subtype = !error_suppressing;
      result.is_error_suppressing = error_suppressing;
    } else if get_type::get::<NeverType>(sub_ty).is_some() {
      result = SubtypingResult::ok();
    } else if get_type::get::<ErrorType>(super_ty).is_some() {
      result = SubtypingResult::fail();
    } else if get_type::get::<ErrorType>(sub_ty).is_some() {
      result = SubtypingResult::ok();
      result.is_error_suppressing = true;
    } else if get_type::get::<TypeFunctionInstanceType>(sub_ty).is_some() {
      // 映射泛型替换后需重新取实例；替换发生时结果不可缓存。
      let (sub_type_function_instance, mapped_generics_applied) = match env.apply_mapped_generics(
        self.builtin_types,
        self.arena,
        sub_ty,
        self.ice_reporter.as_ptr(),
      ) {
        Some(subst_sub_ty) => (
          get_type::get::<TypeFunctionInstanceType>(subst_sub_ty),
          subst_sub_ty != sub_ty,
        ),
        None => (get_type::get::<TypeFunctionInstanceType>(sub_ty), false),
      };

      result = self
        .is_covariant_with_subtyping_environment_type_function_instance_type_type_id_not_null_scope(
          env,
          sub_type_function_instance.expect(
            "TypeFunctionInstance 经 apply_mapped_generics 保持实例变体，同外层 is_some() 分派",
          ),
          super_ty,
          scope,
        );
      result.is_cacheable = !mapped_generics_applied;
    } else if get_type::get::<TypeFunctionInstanceType>(super_ty).is_some() {
      let (super_type_function_instance, mapped_generics_applied) = match env.apply_mapped_generics(
        self.builtin_types,
        self.arena,
        super_ty,
        self.ice_reporter.as_ptr(),
      ) {
        Some(subst_super_ty) => (
          get_type::get::<TypeFunctionInstanceType>(subst_super_ty),
          subst_super_ty != super_ty,
        ),
        None => (get_type::get::<TypeFunctionInstanceType>(super_ty), false),
      };

      result = self
        .is_covariant_with_subtyping_environment_type_id_type_function_instance_type_not_null_scope(
          env,
          sub_ty,
          super_type_function_instance.expect(
            "TypeFunctionInstance 经 apply_mapped_generics 保持实例变体，同外层 is_some() 分派",
          ),
          scope,
        );
      result.is_cacheable = !mapped_generics_applied;
    } else if get_type::get::<GenericType>(sub_ty).is_some()
      || get_type::get::<GenericType>(super_ty).is_some()
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
    } else if let Some((sub_u, sup_u)) = get2::<UnionType, UnionType, _>(sub_ty, super_ty) {
      result = self.is_covariant_with_subtyping_environment_union_type_union_type_not_null_scope(
        env, sub_u, sup_u, scope,
      );
      if !result.is_subtype && !result.normalization_too_complex {
        result = self.try_semantic_subtyping(env, sub_ty, super_ty, scope, &mut result);
      }
    } else if let Some(sub_union) = get_type::get::<UnionType>(sub_ty) {
      result = self.is_covariant_with_subtyping_environment_union_type_type_id_not_null_scope(
        env, sub_union, super_ty, scope,
      );
    } else if let Some(super_union) = get_type::get::<UnionType>(super_ty) {
      result = self.is_covariant_with_subtyping_environment_type_id_union_type_not_null_scope(
        env,
        sub_ty,
        super_union,
        scope,
      );
      if !result.is_subtype && !result.normalization_too_complex {
        result = self.try_semantic_subtyping(env, sub_ty, super_ty, scope, &mut result);
      }
    } else if let Some(super_intersection) = get_type::get::<IntersectionType>(super_ty) {
      result = self
        .is_covariant_with_subtyping_environment_type_id_intersection_type_not_null_scope(
          env,
          sub_ty,
          super_intersection,
          scope,
        );
    } else if let Some(sub_intersection) = get_type::get::<IntersectionType>(sub_ty) {
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
    } else if let Some((sub_neg, sup_neg)) = get2::<NegationType, NegationType, _>(sub_ty, super_ty)
    {
      // We use `isContravariantWith` here in order to make sure that the
      // type paths still look coherent.
      result = self.is_contravariant_with_subtyping_environment_sub_ty_super_ty_not_null_scope(
        env, sub_neg.ty, sup_neg.ty, scope,
      );
      result.with_both_component(Component::TypeField(TypeField::Negated));
    } else if let Some(sub_negation) = get_type::get::<NegationType>(sub_ty) {
      result = self.is_covariant_with_subtyping_environment_negation_type_type_id_not_null_scope(
        env,
        sub_negation,
        super_ty,
        scope,
      );
      if !result.is_subtype && !result.normalization_too_complex {
        result = self.try_semantic_subtyping(env, sub_ty, super_ty, scope, &mut result);
      }
    } else if let Some(super_negation) = get_type::get::<NegationType>(super_ty) {
      result = self.is_covariant_with_subtyping_environment_type_id_negation_type_not_null_scope(
        env,
        sub_ty,
        super_negation,
        scope,
      );
      if !result.is_subtype && !result.normalization_too_complex {
        result = self.try_semantic_subtyping(env, sub_ty, super_ty, scope, &mut result);
      }
    } else if let Some((sub_prim, sup_prim)) =
      get2::<PrimitiveType, PrimitiveType, _>(sub_ty, super_ty)
    {
      result = self
        .is_covariant_with_subtyping_environment_primitive_type_primitive_type_not_null_scope(
          env, sub_prim, sup_prim, scope,
        );
    } else if let Some((sub_singleton, sup_prim)) =
      get2::<SingletonType, PrimitiveType, _>(sub_ty, super_ty)
    {
      result = self
        .is_covariant_with_subtyping_environment_singleton_type_primitive_type_not_null_scope(
          env,
          sub_singleton,
          sup_prim,
          scope,
        );
    } else if let Some((sub_singleton, sup_singleton)) =
      get2::<SingletonType, SingletonType, _>(sub_ty, super_ty)
    {
      result = self
        .is_covariant_with_subtyping_environment_singleton_type_singleton_type_not_null_scope(
          env,
          sub_singleton,
          sup_singleton,
          scope,
        );
    } else if let Some((_, sup_prim)) = get2::<FunctionType, PrimitiveType, _>(sub_ty, super_ty) {
      result.is_subtype = sup_prim.r#type == PrimType::Function;
    } else if let Some((sub_fn, sup_fn)) = get2::<FunctionType, FunctionType, _>(sub_ty, super_ty) {
      // SAFETY: sub_fn/sup_fn 是 get2 自 arena 句柄取出的 FunctionType 有效借用，同步调用内
      // 存活；env 独占、scope 沿 NotNull<Scope> 链非空——满足被调 unsafe fn 的逐项入参契约。
      result = unsafe {
        self.is_covariant_with_subtyping_environment_function_type_function_type_not_null_scope(
          env, sub_fn, sup_fn, scope,
        )
      };
    } else if let Some((sub_table, sup_table)) = get2::<TableType, TableType, _>(sub_ty, super_ty) {
      let force_covariant_test =
        // SAFETY: 前一短路子句已排除空指针；unique_types 借自 TypeChecker 持有的存活
        // DenseHashSet（C++ `const DenseHashSet<TypeId>* uniqueTypes`，Subtyping.h:217），
        // contains 为纯只读查询。
        !self.unique_types.is_null() && unsafe { (*self.unique_types).contains(&sub_ty) };
      result = if fflag::LuauSubtypingTablesHasBetterErrorSuppression.get() {
        self.is_covariant_with_subtyping_environment_table_type_table_type_bool_not_null_scope(
          env,
          sub_table,
          sup_table,
          force_covariant_test,
          scope,
        )
      } else {
        self.is_covariant_with_deprecated(env, sub_table, sup_table, force_covariant_test, scope)
      };
      if result.is_subtype
        && sub_table.indexer.is_none()
        && sup_table.indexer.is_some()
        && sub_table.state != TableState::Sealed
      {
        // FIXME CLI-182960.
        result.with_assumed_constraint(ConstraintV::Subtype(SubtypeConstraint {
          sub_type: sub_ty,
          super_type: super_ty,
        }));
      }
    } else if let Some((sub_meta, sup_meta)) =
      get2::<MetatableType, MetatableType, _>(sub_ty, super_ty)
    {
      result = self
        .is_covariant_with_subtyping_environment_metatable_type_metatable_type_not_null_scope(
          env, sub_meta, sup_meta, scope,
        );
    } else if let Some((sub_meta, sup_table)) =
      get2::<MetatableType, TableType, _>(sub_ty, super_ty)
    {
      result = self
        .is_covariant_with_subtyping_environment_metatable_type_table_type_not_null_scope(
          env, sub_meta, sup_table, scope,
        );
    } else if fflag::LuauTableFreezeCheckIsSubtype.get()
      && let Some((sub_meta, sup_prim)) = get2::<MetatableType, PrimitiveType, _>(sub_ty, super_ty)
    {
      result = self
        .is_covariant_with_subtyping_environment_metatable_type_primitive_type_not_null_scope(
          env, sub_meta, sup_prim, scope,
        );
    } else if let Some((sub_ext, sup_ext)) = get2::<ExternType, ExternType, _>(sub_ty, super_ty) {
      result = self.is_covariant_with_subtyping_environment_extern_type_extern_type_not_null_scope(
        env, sub_ext, sup_ext, scope,
      );
    } else if let Some((sub_ext, sup_table)) = get2::<ExternType, TableType, _>(sub_ty, super_ty) {
      result = self
                .is_covariant_with_subtyping_environment_type_id_extern_type_type_id_table_type_not_null_scope(
                    env,
                    sub_ty,
                    sub_ext,
                    super_ty,
                    sup_table,
                    scope,
                );
    } else if let Some((sub_table, sup_prim)) =
      get2::<TableType, PrimitiveType, _>(sub_ty, super_ty)
    {
      result = self
        .is_covariant_with_subtyping_environment_table_type_primitive_type_not_null_scope(
          env, sub_table, sup_prim, scope,
        );
    } else if let Some((sub_prim, sup_table)) =
      get2::<PrimitiveType, TableType, _>(sub_ty, super_ty)
    {
      result = self
        .is_covariant_with_subtyping_environment_primitive_type_table_type_not_null_scope(
          env, sub_prim, sup_table, scope,
        );
    } else if let Some((sub_singleton, sup_table)) =
      get2::<SingletonType, TableType, _>(sub_ty, super_ty)
    {
      result = self
        .is_covariant_with_subtyping_environment_singleton_type_table_type_not_null_scope(
          env,
          sub_singleton,
          sup_table,
          scope,
        );
    }

    assert_reasoning_valid(sub_ty, super_ty, &result, self.builtin_types, self.arena);

    // ScopedSeenSet destructor — erase the pair before returning.
    self.seen_set_erase(type_pair);

    self.cache(env, result, sub_ty, super_ty)
  }

  /// `ScopedSeenSet` dtor -> `Luau::Set::erase` (`cpp/Analysis/include/Luau/Set.h`):
  /// keep the key and clear its slot bool; `insert` re-arms it.
  #[inline]
  fn seen_set_erase(&mut self, type_pair: (TypeId, TypeId)) {
    let entry = self.seen_types.get_or_insert(type_pair);
    if *entry {
      *entry = false;
    }
  }

  pub fn is_covariant_with_subtyping_environment_normalized_extern_type_type_ids_not_null_scope(
    &mut self,
    env: &mut SubtypingEnvironment,
    sub_extern_type: &NormalizedExternType,
    super_tables: &TypeIds,
    scope: *mut Scope,
  ) -> SubtypingResult {
    for sub_extern_type_ty in sub_extern_type.extern_types.keys() {
      let mut result = SubtypingResult::default();

      for super_table_ty in &super_tables.order {
        result.or_else(
          self.is_covariant_with_subtyping_environment_type_id_type_id_not_null_scope(
            env,
            *sub_extern_type_ty,
            *super_table_ty,
            scope,
          ),
        );
      }

      if !result.is_subtype {
        return result;
      }
    }

    SubtypingResult::ok()
  }

  pub fn is_covariant_with_subtyping_environment_normalized_string_type_normalized_string_type_not_null_scope(
    &mut self,
    _env: &mut SubtypingEnvironment,
    sub_string: &NormalizedStringType,
    super_string: &NormalizedStringType,
    _scope: *mut Scope,
  ) -> SubtypingResult {
    SubtypingResult::from_is_subtype(is_subtype_normalized_string(sub_string, super_string))
  }

  pub fn is_covariant_with_subtyping_environment_normalized_string_type_type_ids_not_null_scope(
    &mut self,
    env: &mut SubtypingEnvironment,
    sub_string: &NormalizedStringType,
    super_tables: &TypeIds,
    scope: *mut Scope,
  ) -> SubtypingResult {
    if sub_string.is_never() {
      return SubtypingResult::ok();
    }

    if sub_string.is_cofinite {
      let mut result = SubtypingResult::default();
      for super_table in &super_tables.order {
        let candidate = self
          .is_covariant_with_subtyping_environment_type_id_type_id_not_null_scope(
            env,
            // SAFETY: 构造契约保证 builtin_types 指向存活 BuiltinTypes；此处只读取 string 类型句柄。
            { self.builtin_types.get().string_type },
            *super_table,
            scope,
          );
        result.or_else(candidate);
        if result.is_subtype {
          return result;
        }
      }
      return result;
    }

    for super_table in &super_tables.order {
      let mut result = SubtypingResult::ok();
      for sub_string in sub_string.singletons.values() {
        let candidate = self
          .is_covariant_with_subtyping_environment_type_id_type_id_not_null_scope(
            env,
            *sub_string,
            *super_table,
            scope,
          );
        result.and_also(candidate, SubtypingSuppressionPolicy::Any);
        if !result.is_subtype {
          break;
        }
      }

      if result.is_subtype {
        return result;
      }
    }

    SubtypingResult::fail()
  }

  pub fn is_covariant_with_subtyping_environment_normalized_function_type_normalized_function_type_not_null_scope(
    &mut self,
    env: &mut SubtypingEnvironment,
    sub_function: &NormalizedFunctionType,
    super_function: &NormalizedFunctionType,
    scope: *mut Scope,
  ) -> SubtypingResult {
    if sub_function.is_never() || super_function.is_top {
      SubtypingResult::ok()
    } else {
      self.is_covariant_with_subtyping_environment_type_ids_type_ids_not_null_scope(
        env,
        &sub_function.parts,
        &super_function.parts,
        scope,
      )
    }
  }

  pub fn is_covariant_with_subtyping_environment_type_ids_type_ids_not_null_scope(
    &mut self,
    env: &mut SubtypingEnvironment,
    sub_types: &TypeIds,
    super_types: &TypeIds,
    scope: *mut Scope,
  ) -> SubtypingResult {
    let mut result = SubtypingResult::ok();

    for sub_ty in &sub_types.order {
      let mut inner_result = SubtypingResult::default();

      for super_ty in &super_types.order {
        let candidate = self
          .is_covariant_with_subtyping_environment_type_id_type_id_not_null_scope(
            env, *sub_ty, *super_ty, scope,
          );
        inner_result.or_else(candidate);

        if inner_result.normalization_too_complex {
          return SubtypingResult::too_complex();
        }
      }

      result.and_also(inner_result, SubtypingSuppressionPolicy::Any);
    }

    result
  }

  pub(crate) fn is_covariant_with_subtyping_environment_type_function_instance_type_type_id_not_null_scope(
    &mut self,
    env: &mut SubtypingEnvironment,
    sub_function_instance: &TypeFunctionInstanceType,
    super_ty: TypeId,
    scope: *mut Scope,
  ) -> SubtypingResult {
    let (ty, mut errors) =
      // SAFETY: 被调 unsafe fn 要求 self 的 arena/builtin_types/normalizer 等裸成员均指向
      // 存活对象且 scope 非空——这些由 Subtyping 构造契约与本链 C++ `NotNull<Scope>` 传参
      // 保证；sub_function_instance 借自 arena 节点，在本次同步调用内有效。
      unsafe { self.handle_type_function_reduction_result(sub_function_instance, scope) };

    self
      .is_covariant_with_subtyping_environment_type_id_type_id_not_null_scope(
        env, ty, super_ty, scope,
      )
      .with_errors(&mut errors)
      .with_sub_component(Component::Reduction(Reduction { result_type: ty }))
      .to_owned()
  }

  pub(crate) fn is_covariant_with_subtyping_environment_type_id_type_function_instance_type_not_null_scope(
    &mut self,
    env: &mut SubtypingEnvironment,
    sub_ty: TypeId,
    super_function_instance: &TypeFunctionInstanceType,
    scope: *mut Scope,
  ) -> SubtypingResult {
    let (ty, mut errors) =
      // SAFETY: 与 sub 侧同理——被调要求 self 裸成员存活、scope 非空（NotNull<Scope> 链），
      // super_function_instance 为 arena 节点的有效借用。
      unsafe { self.handle_type_function_reduction_result(super_function_instance, scope) };
    self
      .is_covariant_with_subtyping_environment_type_id_type_id_not_null_scope(
        env, sub_ty, ty, scope,
      )
      .with_errors(&mut errors)
      .with_super_component(Component::Reduction(Reduction { result_type: ty }))
      .to_owned()
  }

  /// 对应 C++ `Subtyping::isCovariantWith(env, TypePackId, TypePackId, NotNull<Scope>)`
  /// （`cpp/Analysis/src/Subtyping.cpp:998`）。
  ///
  /// # Safety
  /// - `scope` 须指向检查期间存活的 [`Scope`] 且非空（对应 C++ `NotNull<Scope>` 形参）——
  ///   尾包不匹配的错误分支会解引用读取 `location`。
  /// - `sub_tp`/`super_tp` 须为类型 arena 分配、在本次调用（含递归）期间存活的
  ///   `TypePackVar` 句柄；follow/flatten 只沿其内部结构读取，无并发写。
  /// - `env` 为调用方独占借用，其 mapped_generics/substitutions 在整条递归链中保持有效。
  pub unsafe fn is_covariant_with_subtyping_environment_type_pack_id_type_pack_id_not_null_scope(
    &mut self,
    env: &mut SubtypingEnvironment,
    mut sub_tp: TypePackId,
    mut super_tp: TypePackId,
    scope: *mut Scope,
  ) -> SubtypingResult {
    let _nerl = NonExceptionalRecursionLimiter::new(
      &mut self
        .normalizer_mut()
        .shared_state_mut()
        .counters
        .recursion_count,
    );
    if !_nerl.is_ok(dfint::LuauSubtypingRecursionLimit.get()) {
      return SubtypingResult::too_complex();
    }

    sub_tp = follow_type_pack::follow(sub_tp);
    super_tp = follow_type_pack::follow(super_tp);

    let type_pair = (sub_tp, super_tp);
    let fresh = {
      let entry = self.seen_packs.get_or_insert(type_pair);
      let fresh = !*entry;
      if fresh {
        *entry = true;
      }
      fresh
    };
    if !fresh {
      return SubtypingResult::uncacheable_ok();
    }

    let (sub_head, sub_tail) = flatten_type_pack_id(sub_tp);
    let (super_head, super_tail) = flatten_type_pack_id(super_tp);
    let head_size = min(sub_head.len(), super_head.len());

    let mut result = SubtypingResult::ok();

    if sub_tp == super_tp {
      self.seen_pack_set_erase(type_pair);
      return result;
    }

    for (i, (&sub_part, &super_part)) in sub_head
      .iter()
      .zip(super_head.iter())
      .take(head_size)
      .enumerate()
    {
      let mut part = self.is_covariant_with_subtyping_environment_type_id_type_id_not_null_scope(
        env, sub_part, super_part, scope,
      );
      part.with_both_component(Component::Index(Index {
        index: i,
        variant: Variant::Pack,
      }));
      result.and_also(part, SubtypingSuppressionPolicy::Any);
    }

    if sub_head.len() < super_head.len() {
      if let Some(sub_tail) = sub_tail {
        let early_exit = unsafe {
          // SAFETY: 被调 unsafe fn 的契约由本处参数满足——sub_tail/super_tail 是
          // flatten_type_pack_id 产出的有效 arena pack 句柄，env 与 output_result 为
          // 当前作用域独占可变借用，scope 沿本函数入口的非空契约透传。
          self.is_sub_tail_covariant_with(SubTailCovariantArgs {
            env,
            output_result: &mut result,
            sub_tp,
            sub_tail,
            super_tp,
            super_head_start_index: head_size,
            super_head: &super_head,
            super_tail,
            scope,
          })
        };
        if early_exit == EarlyExit::Yes {
          self.seen_pack_set_erase(type_pair);
          return result;
        }
      } else {
        result.and_also(SubtypingResult::fail(), SubtypingSuppressionPolicy::Any);
        self.seen_pack_set_erase(type_pair);
        return result;
      }
    } else if sub_head.len() > super_head.len() {
      if let Some(super_tail) = super_tail {
        let early_exit = unsafe {
          // SAFETY: super 侧对称情形——super_tail 与 sub_head 片段均来自同一 flatten 结果，
          // 句柄有效；env/output_result 借用独占，scope 非空契约透传。
          self.is_covariant_with_super_tail(SuperTailCovariantArgs {
            env,
            output_result: &mut result,
            sub_tp,
            sub_head_start_index: head_size,
            sub_head: &sub_head,
            sub_tail,
            super_tp,
            super_tail,
            scope,
          })
        };
        if early_exit == EarlyExit::Yes {
          self.seen_pack_set_erase(type_pair);
          return result;
        }
      } else {
        self.seen_pack_set_erase(type_pair);
        return SubtypingResult::fail();
      }
    }

    match (sub_tail, super_tail) {
      (Some(sub_tail), Some(super_tail)) => {
        if let (Some(sub), Some(super_)) = (
          get_type_pack::get::<VariadicTypePack>(sub_tail),
          get_type_pack::get::<VariadicTypePack>(super_tail),
        ) {
          let part = self
                        .is_tail_covariant_with_tail_subtyping_environment_not_null_scope_type_pack_id_variadic_type_pack_type_pack_id_variadic_type_pack(
                        env, scope, sub_tail, sub, super_tail, super_,
                    );
          result.and_also(part, SubtypingSuppressionPolicy::Any);
        } else if let (Some(sub), Some(super_)) = (
          get_type_pack::get::<GenericTypePack>(sub_tail),
          get_type_pack::get::<GenericTypePack>(super_tail),
        ) {
          let part = self
                        .is_tail_covariant_with_tail_subtyping_environment_not_null_scope_type_pack_id_generic_type_pack_type_pack_id_generic_type_pack(
                        env, scope, sub_tail, sub, super_tail, super_,
                    );
          result.and_also(part, SubtypingSuppressionPolicy::Any);
        } else if let (Some(sub), Some(super_)) = (
          get_type_pack::get::<VariadicTypePack>(sub_tail),
          get_type_pack::get::<GenericTypePack>(super_tail),
        ) {
          let part = self
                        .is_tail_covariant_with_tail_subtyping_environment_not_null_scope_type_pack_id_variadic_type_pack_type_pack_id_generic_type_pack(
                        env, scope, sub_tail, sub, super_tail, super_,
                    );
          result.and_also(part, SubtypingSuppressionPolicy::Any);
        } else if let (Some(sub), Some(super_)) = (
          get_type_pack::get::<GenericTypePack>(sub_tail),
          get_type_pack::get::<VariadicTypePack>(super_tail),
        ) {
          let part = self
                        .is_tail_covariant_with_tail_subtyping_environment_not_null_scope_type_pack_id_generic_type_pack_type_pack_id_variadic_type_pack(
                        env, scope, sub_tail, sub, super_tail, super_,
                    );
          result.and_also(part, SubtypingSuppressionPolicy::Any);
        } else if get_type_pack::get::<FreeTypePack>(sub_tail).is_some()
          || get_type_pack::get::<FreeTypePack>(super_tail).is_some()
        {
          let mut part = SubtypingResult::ok();
          part.with_both_component(Component::PackField(PackField::Tail));
          part.with_assumed_constraint(ConstraintV::PackSubtype(PackSubtypeConstraint {
            sub_pack: sub_tail,
            super_pack: super_tail,
            returns: false,
          }));
          result.and_also(part, SubtypingSuppressionPolicy::Any);
        } else if get_type_pack::get::<ErrorTypePack>(sub_tail).is_some()
          || get_type_pack::get::<ErrorTypePack>(super_tail).is_some()
        {
          let mut part = SubtypingResult::ok();
          part.with_both_component(Component::PackField(PackField::Tail));
          result.and_also(part, SubtypingSuppressionPolicy::Any);
        } else {
          let mut r = SubtypingResult::fail();
          r.with_both_component(Component::PackField(PackField::Tail));
          r.with_error(TypeError::type_error_location_type_error_data(
            // SAFETY: scope 遵循 C++ `NotNull<Scope>` 入参契约，非空且指向检查期内存活的
            // Scope；location 只读取值。
            unsafe { (*scope).location },
            UnexpectedTypePackInSubtyping { tp: sub_tail }.into(),
          ));
          r.with_error(TypeError::type_error_location_type_error_data(
            // SAFETY: 同上，scope 仍为调用期存活非空指针。
            unsafe { (*scope).location },
            UnexpectedTypePackInSubtyping { tp: super_tail }.into(),
          ));
          self.seen_pack_set_erase(type_pair);
          return r;
        }
      }
      (Some(sub_tail), None) => {
        if get_type_pack::get::<VariadicTypePack>(sub_tail).is_some() {
          let mut r = SubtypingResult::fail();
          r.with_sub_component(Component::PackField(PackField::Tail));
          self.seen_pack_set_erase(type_pair);
          return r;
        } else if let Some(g) = get_type_pack::get::<GenericTypePack>(sub_tail) {
          let r = self
                        .is_tail_covariant_with_tail_subtyping_environment_not_null_scope_type_pack_id_generic_type_pack_nothing(
                        env,
                        scope,
                        sub_tail,
                        g,
                        Nothing::default(),
                    );
          self.seen_pack_set_erase(type_pair);
          return r;
        } else if get_type_pack::get::<FreeTypePack>(sub_tail).is_some() {
          let mut r = SubtypingResult::ok();
          r.with_both_component(Component::PackField(PackField::Tail));
          r.with_assumed_constraint(ConstraintV::PackSubtype(PackSubtypeConstraint {
            sub_pack: sub_tail,
            // SAFETY: builtin_types 由构造契约存活；仅读取空 pack 句柄作为约束的另一侧。
            super_pack: { self.builtin_types.get().empty_type_pack },
            returns: false,
          }));
          self.seen_pack_set_erase(type_pair);
          return r;
        } else {
          let mut r = SubtypingResult::fail();
          r.with_sub_component(Component::PackField(PackField::Tail));
          r.with_error(TypeError::type_error_location_type_error_data(
            // SAFETY: NotNull<Scope> 契约下 scope 非空且存活，仅读 location 构造错误。
            unsafe { (*scope).location },
            UnexpectedTypePackInSubtyping { tp: sub_tail }.into(),
          ));
          self.seen_pack_set_erase(type_pair);
          return r;
        }
      }
      (None, Some(super_tail)) => {
        if get_type_pack::get::<VariadicTypePack>(super_tail).is_some() {
          // A variadic super tail accepts the empty finite remainder.
        } else if let Some(g) = get_type_pack::get::<GenericTypePack>(super_tail) {
          let part = self
                        .is_tail_covariant_with_tail_subtyping_environment_not_null_scope_nothing_type_pack_id_generic_type_pack(
                        env,
                        scope,
                        Nothing::default(),
                        super_tail,
                        g,
                    );
          result.and_also(part, SubtypingSuppressionPolicy::Any);
        } else if get_type_pack::get::<FreeTypePack>(super_tail).is_some() {
          let mut part = SubtypingResult::ok();
          part.with_both_component(Component::PackField(PackField::Tail));
          part.with_assumed_constraint(ConstraintV::PackSubtype(PackSubtypeConstraint {
            // SAFETY: builtin_types 指针在构造契约内存活，只读取 empty_type_pack 句柄。
            sub_pack: { self.builtin_types.get().empty_type_pack },
            super_pack: super_tail,
            returns: false,
          }));
          result.and_also(part, SubtypingSuppressionPolicy::Any);
        } else {
          let mut r = SubtypingResult::fail();
          r.with_super_component(Component::PackField(PackField::Tail));
          r.with_error(TypeError::type_error_location_type_error_data(
            // SAFETY: scope 非空由 NotNull<Scope> 传参契约保证，location 仅只读。
            unsafe { (*scope).location },
            UnexpectedTypePackInSubtyping { tp: super_tail }.into(),
          ));
          self.seen_pack_set_erase(type_pair);
          return r;
        }
      }
      (None, None) => {}
    }

    assert_reasoning_valid(sub_tp, super_tp, &result, self.builtin_types, self.arena);

    self.seen_pack_set_erase(type_pair);
    result
  }

  #[inline]
  /// `Luau::Set::erase`: clear the slot bool, keep the key (cpp/Analysis/include/Luau/Set.h).
  fn seen_pack_set_erase(&mut self, type_pair: (TypePackId, TypePackId)) {
    let entry = self.seen_packs.get_or_insert(type_pair);
    if *entry {
      *entry = false;
    }
  }

  pub fn is_covariant_with_subtyping_environment_type_id_union_type_not_null_scope(
    &mut self,
    env: &mut SubtypingEnvironment,
    sub_ty: TypeId,
    super_union: &UnionType,
    scope: *mut Scope,
  ) -> SubtypingResult {
    let mut result = SubtypingResult::fail();
    // C++ `for (TypeId ty : superUnion)` — UnionTypeIterator 防环并展平嵌套 union。
    for (index, ty) in begin_union_type(super_union).enumerate() {
      let mut next = self.is_covariant_with_subtyping_environment_type_id_type_id_not_null_scope(
        env, sub_ty, ty, scope,
      );
      if next.normalization_too_complex {
        return SubtypingResult::too_complex();
      }
      if next.is_subtype {
        return next;
      }
      next.with_super_component(Component::Index(Index {
        index,
        variant: Variant::Union,
      }));
      result.and_also(next, SubtypingSuppressionPolicy::Any);
    }
    result
  }

  pub fn is_covariant_with_subtyping_environment_union_type_type_id_not_null_scope(
    &mut self,
    env: &mut SubtypingEnvironment,
    sub_union: &UnionType,
    super_ty: TypeId,
    scope: *mut Scope,
  ) -> SubtypingResult {
    let mut result = SubtypingResult::ok();
    // C++ `for (TypeId ty : subUnion)` — UnionTypeIterator 防环并展平嵌套 union。
    for (index, ty) in begin_union_type(sub_union).enumerate() {
      let mut next = self.is_covariant_with_subtyping_environment_type_id_type_id_not_null_scope(
        env, ty, super_ty, scope,
      );
      if next.normalization_too_complex {
        return SubtypingResult::too_complex();
      }
      next.with_sub_component(Component::Index(Index {
        index,
        variant: Variant::Union,
      }));
      result.and_also(next, SubtypingSuppressionPolicy::Any);
    }
    result
  }

  pub fn is_covariant_with_subtyping_environment_type_id_intersection_type_not_null_scope(
    &mut self,
    env: &mut SubtypingEnvironment,
    sub_ty: TypeId,
    super_intersection: &IntersectionType,
    scope: *mut Scope,
  ) -> SubtypingResult {
    let mut result = SubtypingResult::ok();
    // C++ `for (TypeId ty : superIntersection)` — IntersectionTypeIterator 防环并展平。
    for (i, ty) in begin_intersection_type(super_intersection).enumerate() {
      let mut candidate = self
        .is_covariant_with_subtyping_environment_type_id_type_id_not_null_scope(
          env, sub_ty, ty, scope,
        );
      candidate.with_super_component(Component::Index(Index {
        index: i,
        variant: Variant::Intersection,
      }));
      result.and_also(candidate, SubtypingSuppressionPolicy::Any);

      if result.normalization_too_complex {
        return SubtypingResult::too_complex();
      }
    }

    result
  }

  pub fn is_covariant_with_subtyping_environment_intersection_type_type_id_not_null_scope(
    &mut self,
    env: &mut SubtypingEnvironment,
    sub_intersection: &IntersectionType,
    super_ty: TypeId,
    scope: *mut Scope,
  ) -> SubtypingResult {
    let mut result = SubtypingResult::default();
    // C++ `for (TypeId ty : subIntersection)` — IntersectionTypeIterator 防环并展平。
    for (i, ty) in begin_intersection_type(sub_intersection).enumerate() {
      let mut candidate = self
        .is_covariant_with_subtyping_environment_type_id_type_id_not_null_scope(
          env, ty, super_ty, scope,
        );
      candidate.with_sub_component(Component::Index(Index {
        index: i,
        variant: Variant::Intersection,
      }));
      result.or_else(candidate);

      if result.normalization_too_complex {
        return SubtypingResult::too_complex();
      }
    }

    result
  }

  pub fn is_covariant_with_subtyping_environment_negation_type_type_id_not_null_scope(
    &mut self,
    env: &mut SubtypingEnvironment,
    sub_negation: &NegationType,
    super_ty: TypeId,
    scope: *mut Scope,
  ) -> SubtypingResult {
    let negated_ty = follow_type::follow(sub_negation.ty);

    let mut result = SubtypingResult::default();

    if get_type::get::<NeverType>(negated_ty).is_some() {
      result = self.is_covariant_with_subtyping_environment_type_id_type_id_not_null_scope(
        env,
        // SAFETY: builtin_types 为构造契约保证的存活只读对象；¬never 归约为 unknown，仅取句柄。
        { self.builtin_types.get().unknown_type },
        super_ty,
        scope,
      );
      result.with_sub_component(Component::TypeField(TypeField::Negated));
    } else if get_type::get::<UnknownType>(negated_ty).is_some() {
      result = self.is_covariant_with_subtyping_environment_type_id_type_id_not_null_scope(
        env,
        // SAFETY: 同上，¬unknown 归约为 never 的只读句柄读取。
        { self.builtin_types.get().never_type },
        super_ty,
        scope,
      );
      result.with_sub_component(Component::TypeField(TypeField::Negated));
    } else if get_type::get::<AnyType>(negated_ty).is_some() {
      result = self.is_covariant_with_subtyping_environment_type_id_type_id_not_null_scope(
        env, negated_ty, super_ty, scope,
      );
      result.with_sub_component(Component::TypeField(TypeField::Negated));
    } else if let Some(u) = get_type::get::<UnionType>(negated_ty) {
      result = SubtypingResult::ok();

      // C++ `for (TypeId ty : u)`——UnionTypeIterator 展平嵌套 union 并
      // follow,裸遍历 options 会漏掉嵌套成员。
      for ty in begin_union_type(u) {
        if let Some(negated_part) = get_type::get::<NegationType>(follow_type::follow(ty)) {
          let mut inner = self
            .is_covariant_with_subtyping_environment_type_id_type_id_not_null_scope(
              env,
              negated_part.ty,
              super_ty,
              scope,
            );
          inner.with_sub_component(Component::TypeField(TypeField::Negated));
          result.and_also(inner, SubtypingSuppressionPolicy::Any);
        } else {
          let negated_tmp = NegationType { ty };
          result.and_also(
            self.is_covariant_with_subtyping_environment_negation_type_type_id_not_null_scope(
              env,
              &negated_tmp,
              super_ty,
              scope,
            ),
            SubtypingSuppressionPolicy::Any,
          );
        }
      }
    } else if let Some(i) = get_type::get::<IntersectionType>(negated_ty) {
      result = SubtypingResult::fail();

      // C++ `for (TypeId ty : i)`——IntersectionTypeIterator 同理展平。
      for ty in begin_intersection_type(i) {
        if let Some(negated_part) = get_type::get::<NegationType>(follow_type::follow(ty)) {
          let mut inner = self
            .is_covariant_with_subtyping_environment_type_id_type_id_not_null_scope(
              env,
              negated_part.ty,
              super_ty,
              scope,
            );
          inner.with_sub_component(Component::TypeField(TypeField::Negated));
          result.or_else(inner);
        } else {
          let negated_tmp = NegationType { ty };
          result.or_else(
            self.is_covariant_with_subtyping_environment_negation_type_type_id_not_null_scope(
              env,
              &negated_tmp,
              super_ty,
              scope,
            ),
          );
        }
      }
    } else if get_type::get::<ErrorType>(negated_ty).is_some()
      || get_type::get::<FunctionType>(negated_ty).is_some()
      || get_type::get::<TableType>(negated_ty).is_some()
      || get_type::get::<MetatableType>(negated_ty).is_some()
    {
      // SAFETY: ice_reporter 是构造期注入、整个检查期内存活的 InternalErrorReporter
      // （C++ `iceReporter->ice`）；ice_string 只记录诊断，不持有逃逸借用。
      {
        self
          .ice_reporter
          .get()
          .ice_string("attempting to negate a non-testable type");
      }
    } else {
      result = SubtypingResult::fail();
      result.with_sub_component(Component::TypeField(TypeField::Negated));
    }

    result
  }

  pub fn is_covariant_with_subtyping_environment_type_id_negation_type_not_null_scope(
    &mut self,
    env: &mut SubtypingEnvironment,
    sub_ty: TypeId,
    super_negation: &NegationType,
    scope: *mut Scope,
  ) -> SubtypingResult {
    let negated_ty = follow_type::follow(super_negation.ty);

    let mut result = SubtypingResult::default();

    // sub_ty 是否为 ExternType（get2 纯查询，提前绑定以简化 if 链条件）
    let sub_extern = get2::<ExternType, PrimitiveType, _>(sub_ty, negated_ty).is_some();

    if get_type::get::<NeverType>(negated_ty).is_some() {
      // ¬never ~ unknown
      result = self.is_covariant_with_subtyping_environment_type_id_type_id_not_null_scope(
        env,
        sub_ty,
        // SAFETY: builtin_types 存活且不变（构造契约），仅读取 unknown_type。
        { self.builtin_types.get().unknown_type },
        scope,
      );
    } else if get_type::get::<UnknownType>(negated_ty).is_some() {
      // ¬unknown ~ never
      result = self.is_covariant_with_subtyping_environment_type_id_type_id_not_null_scope(
        env,
        sub_ty,
        // SAFETY: 同分支链，读取 never_type 句柄无失效风险。
        { self.builtin_types.get().never_type },
        scope,
      );
    } else if get_type::get::<AnyType>(negated_ty).is_some() {
      // ¬any ~ any
      result = self.is_covariant_with_subtyping_environment_type_id_type_id_not_null_scope(
        env, sub_ty, negated_ty, scope,
      );
    } else if let Some(u) = get_type::get::<UnionType>(negated_ty) {
      // ¬(A ∪ B) ~ ¬A ∩ ¬B
      // follow intersection rules: A & B <: T iff A <: T && B <: T
      result = SubtypingResult::ok();

      // C++ `for (TypeId ty : u)`——UnionTypeIterator 展平嵌套 union 并
      // follow,裸遍历 options 会漏掉嵌套成员。
      for ty in begin_union_type(u) {
        if let Some(negated_part) = get_type::get::<NegationType>(follow_type::follow(ty)) {
          result.and_also(
            self.is_covariant_with_subtyping_environment_type_id_type_id_not_null_scope(
              env,
              sub_ty,
              negated_part.ty,
              scope,
            ),
            SubtypingSuppressionPolicy::Any,
          );
        } else {
          let negated_tmp = NegationType { ty };
          result.and_also(
            self.is_covariant_with_subtyping_environment_type_id_negation_type_not_null_scope(
              env,
              sub_ty,
              &negated_tmp,
              scope,
            ),
            SubtypingSuppressionPolicy::Any,
          );
        }
      }
    } else if let Some(i) = get_type::get::<IntersectionType>(negated_ty) {
      // ¬(A ∩ B) ~ ¬A ∪ ¬B
      // follow union rules: A | B <: T iff A <: T || B <: T
      result = SubtypingResult::fail();

      // C++ `for (TypeId ty : i)`——IntersectionTypeIterator 同理展平。
      for ty in begin_intersection_type(i) {
        if let Some(negated_part) = get_type::get::<NegationType>(follow_type::follow(ty)) {
          result.or_else(
            self.is_covariant_with_subtyping_environment_type_id_type_id_not_null_scope(
              env,
              sub_ty,
              negated_part.ty,
              scope,
            ),
          );
        } else {
          let negated_tmp = NegationType { ty };
          result.or_else(
            self.is_covariant_with_subtyping_environment_type_id_negation_type_not_null_scope(
              env,
              sub_ty,
              &negated_tmp,
              scope,
            ),
          );
        }
      }
    } else if let Some((sub_prim, sup_prim)) =
      get2::<PrimitiveType, PrimitiveType, _>(sub_ty, negated_ty)
    {
      // number <: ¬boolean
      // number </: ¬number
      result = SubtypingResult::from_is_subtype(sub_prim.r#type != sup_prim.r#type);
    } else if let Some((sub_singleton, sup_prim)) =
      get2::<SingletonType, PrimitiveType, _>(sub_ty, negated_ty)
    {
      // "foo" </: ¬string
      if (sub_singleton.variant.get_if::<StringSingleton>().is_some()
        && sup_prim.r#type == PrimType::String)
        || (sub_singleton.variant.get_if::<BooleanSingleton>().is_some()
          && sup_prim.r#type == PrimType::Boolean)
      {
        result = SubtypingResult::fail();
      }
      // other cases are true
      else {
        result = SubtypingResult::ok();
      }
    } else if let Some((sub_prim, sup_singleton)) =
      get2::<PrimitiveType, SingletonType, _>(sub_ty, negated_ty)
    {
      if (sub_prim.r#type == PrimType::String
        && sup_singleton.variant.get_if::<StringSingleton>().is_some())
        || (sub_prim.r#type == PrimType::Boolean
          && sup_singleton.variant.get_if::<BooleanSingleton>().is_some())
      {
        result = SubtypingResult::fail();
      } else {
        result = SubtypingResult::ok();
      }
    }
    // the top class type is not actually a primitive type, so the negation of
    // any one of them includes the top class type.
    else if sub_extern {
      result = SubtypingResult::ok();
    } else if get_type::get::<PrimitiveType>(negated_ty).is_some()
      && (get_type::get::<TableType>(sub_ty).is_some()
        || get_type::get::<MetatableType>(sub_ty).is_some())
    {
      let p = get_type::get::<PrimitiveType>(negated_ty)
        .expect("同判据 get_type::get::<PrimitiveType>(negated_ty).is_some() 刚在 if 条件命中");
      result = SubtypingResult::from_is_subtype(p.r#type != PrimType::Table);
    } else if let Some((_, sup_prim)) = get2::<FunctionType, PrimitiveType, _>(sub_ty, negated_ty) {
      result = SubtypingResult::from_is_subtype(sup_prim.r#type != PrimType::Function);
    } else if let Some((sub_singleton, sup_singleton)) =
      get2::<SingletonType, SingletonType, _>(sub_ty, negated_ty)
    {
      result = SubtypingResult::from_is_subtype(sub_singleton != sup_singleton);
    } else if let Some((sub_ext, sup_ext)) = get2::<ExternType, ExternType, _>(sub_ty, negated_ty) {
      let inner = self
        .is_covariant_with_subtyping_environment_extern_type_extern_type_not_null_scope(
          env, sub_ext, sup_ext, scope,
        );
      result = SubtypingResult::negate(&inner);
    } else if get2::<FunctionType, ExternType, _>(sub_ty, negated_ty).is_some() {
      result = SubtypingResult::ok();
    } else if get_type::get::<ErrorType>(negated_ty).is_some()
      || get_type::get::<FunctionType>(negated_ty).is_some()
      || get_type::get::<TableType>(negated_ty).is_some()
      || get_type::get::<MetatableType>(negated_ty).is_some()
    {
      // SAFETY: super 侧同情形——ice_reporter 由 Subtyping 构造时传入且指向存活 reporter，
      // 调用同步完成，仅内部记录 ICE 字符串。
      {
        self
          .ice_reporter
          .get()
          .ice_string("attempting to negate a non-testable type");
      }
    } else {
      result = SubtypingResult::fail();
    }

    result.with_super_component(Component::TypeField(TypeField::Negated));
    result
  }

  pub fn is_covariant_with_subtyping_environment_primitive_type_primitive_type_not_null_scope(
    &mut self,
    _env: &mut SubtypingEnvironment,
    sub_prim: &PrimitiveType,
    super_prim: &PrimitiveType,
    _scope: *mut Scope,
  ) -> SubtypingResult {
    SubtypingResult::from_is_subtype(sub_prim.r#type == super_prim.r#type)
  }

  pub fn is_covariant_with_subtyping_environment_singleton_type_primitive_type_not_null_scope(
    &mut self,
    _env: &mut SubtypingEnvironment,
    sub_singleton: &SingletonType,
    super_prim: &PrimitiveType,
    _scope: *mut Scope,
  ) -> SubtypingResult {
    if get_singleton_type::<StringSingleton>(sub_singleton).is_some()
      && super_prim.r#type == PrimitiveType::STRING
    {
      return SubtypingResult::ok();
    }

    if get_singleton_type::<BooleanSingleton>(sub_singleton).is_some()
      && super_prim.r#type == PrimitiveType::BOOLEAN
    {
      return SubtypingResult::ok();
    }

    SubtypingResult::fail()
  }

  pub fn is_covariant_with_subtyping_environment_singleton_type_singleton_type_not_null_scope(
    &mut self,
    _env: &mut SubtypingEnvironment,
    sub_singleton: &SingletonType,
    super_singleton: &SingletonType,
    _scope: *mut Scope,
  ) -> SubtypingResult {
    SubtypingResult::from_is_subtype(sub_singleton == super_singleton)
  }
}

/// C++ local `auto record = [&](SubtypingResult subResult) { ... }`
/// (Subtyping.cpp:1951-1956). Tracks the error-suppression bookkeeping and
/// folds `subResult` into `result` using the default `andAlso` policy (`Any`).
fn record(
  result: &mut SubtypingResult,
  has_error_suppression: &mut bool,
  should_suppress_errors: &mut bool,
  sub_result: SubtypingResult,
) {
  *has_error_suppression |= sub_result.is_error_suppressing;
  *should_suppress_errors &= sub_result.is_subtype || sub_result.is_error_suppressing;
  result.and_also(sub_result, SubtypingSuppressionPolicy::Any);
}

impl Subtyping {
  pub fn is_covariant_with_subtyping_environment_table_type_table_type_bool_not_null_scope(
    &mut self,
    env: &mut SubtypingEnvironment,
    sub_table: &TableType,
    super_table: &TableType,
    force_covariant_test: bool,
    scope: *mut Scope,
  ) -> SubtypingResult {
    let mut result = SubtypingResult::ok();

    if sub_table.props.is_empty()
      && sub_table.indexer.is_none()
      && sub_table.state == TableState::Sealed
      && super_table.indexer.is_some()
    {
      // While it is certainly the case that {} </: {T}, the story is a
      // little bit different for {| |} <: {T}: an unsealed table will
      // later gain the necessary indexer as inference proceeds.
      return SubtypingResult::fail();
    }

    // This is an unfortunately complicated state machine: an `any`-typed
    // property must be error-suppressing without surfacing a hard error,
    // but a genuinely mismatched property (e.g. `number != boolean`) must
    // still report even when another property suppressed.
    let mut has_error_suppression = false;
    let mut should_suppress_errors = true;

    for (name, super_prop) in &super_table.props {
      // If the sub table has the property with the specific name: check
      // whether the two are invariant subtypes.
      if let Some(sub_prop) = sub_table.props.get(name) {
        let sub_result = self
          .is_covariant_with_subtyping_environment_property_property_string_bool_not_null_scope(
            env,
            sub_prop,
            super_prop,
            name,
            force_covariant_test,
            scope,
          );
        record(
          &mut result,
          &mut has_error_suppression,
          &mut should_suppress_errors,
          sub_result,
        );
      }
      // Otherwise, if the sub table has an indexer whose key type is a
      // super type of string, then use that as the "property" type, e.g.
      // `{ [string]: number } <: { foo: number }`.
      else if let Some(sub_indexer) = sub_table.indexer.as_ref()
        && {
          // SAFETY: 块内唯一 unsafe 是 self.builtin_types.get().string_type——构造契约保证该
          // 指针存活且只读；其余参数为有效句柄/借用，被调方法本身安全。
          self
            .is_covariant_with_subtyping_environment_type_id_type_id_not_null_scope(
              env,
              self.builtin_types.get().string_type,
              sub_indexer.index_type,
              scope,
            )
            .is_subtype
        }
      {
        if super_prop.is_shared() {
          if fflag::LuauReadOnlyIndexers.get() && sub_indexer.is_read_only {
            // A read-only indexer cannot satisfy a read-write
            // property requirement.
            let mut sr = SubtypingResult::fail();
            sr.with_sub_component(index_result_component());
            sr.with_super_component(path_property(name, true));
            record(
              &mut result,
              &mut has_error_suppression,
              &mut should_suppress_errors,
              sr,
            );
          } else {
            let mut sr = self
              .is_invariant_with_subtyping_environment_sub_ty_super_ty_not_null_scope(
                env,
                sub_indexer.index_result_type,
                super_prop
                  .read_ty
                  .expect("外层 super_prop.is_shared() 定义蕴含 read_ty 为 Some"),
                scope,
              );
            sr.with_sub_component(index_result_component());
            sr.with_super_component(path_property(name, true));
            record(
              &mut result,
              &mut has_error_suppression,
              &mut should_suppress_errors,
              sr,
            );
          }
        } else {
          if let Some(super_read_ty) = super_prop.read_ty {
            let mut sr = self
              .is_covariant_with_subtyping_environment_type_id_type_id_not_null_scope(
                env,
                sub_indexer.index_result_type,
                super_read_ty,
                scope,
              );
            sr.with_sub_component(index_result_component());
            sr.with_super_component(path_property(name, true));
            record(
              &mut result,
              &mut has_error_suppression,
              &mut should_suppress_errors,
              sr,
            );
          }
          if let Some(super_write_ty) = super_prop.write_ty {
            if fflag::LuauReadOnlyIndexers.get() && sub_indexer.is_read_only {
              let mut sr = SubtypingResult::fail();
              sr.with_sub_component(index_result_component());
              sr.with_super_component(path_property(name, false));
              record(
                &mut result,
                &mut has_error_suppression,
                &mut should_suppress_errors,
                sr,
              );
            } else {
              let mut sr = self
                .is_contravariant_with_subtyping_environment_sub_ty_super_ty_not_null_scope(
                  env,
                  sub_indexer.index_result_type,
                  super_write_ty,
                  scope,
                );
              sr.with_sub_component(index_result_component());
              sr.with_super_component(path_property(name, false));
              record(
                &mut result,
                &mut has_error_suppression,
                &mut should_suppress_errors,
                sr,
              );
            }
          }
        }
      } else if fflag::LuauSubtypingMissingPropertiesAsNil.get() {
        // SAFETY: unsafe 仅为读取存活 builtin_types 的 nil_type 句柄；readonly 构造是安全函数。
        let nil_prop = { Property::readonly(self.builtin_types.get().nil_type) };
        let mut sr = self
          .is_covariant_with_subtyping_environment_property_property_string_bool_not_null_scope(
            env,
            &nil_prop,
            super_prop,
            name,
            force_covariant_test,
            scope,
          );
        // We must ignore the reasoning from here because the subtype
        // doesn't have a property to traverse into later.
        sr.reasoning.clear();
        record(
          &mut result,
          &mut has_error_suppression,
          &mut should_suppress_errors,
          sr,
        );
      }
      // If the subtable doesn't have a string indexer and the required
      // property does not exist, we can exit early.
      else {
        return SubtypingResult::fail();
      }
    }

    if let Some(super_indexer) = &super_table.indexer {
      if let Some(sub_indexer) = &sub_table.indexer {
        let sub_result = if fflag::LuauReadOnlyIndexers.get() {
          // isCovariantWith() properly handles variance of the index
          // result type.
          self.is_covariant_with_subtyping_environment_table_indexer_table_indexer_not_null_scope(
            env,
            sub_indexer,
            super_indexer,
            scope,
          )
        } else {
          self.is_invariant_with_subtyping_environment_sub_ty_super_ty_not_null_scope(
            env,
            *sub_indexer,
            *super_indexer,
            scope,
          )
        };
        record(
          &mut result,
          &mut has_error_suppression,
          &mut should_suppress_errors,
          sub_result,
        );
      } else if sub_table.state != TableState::Sealed {
        // As above, we assume that {| |} <: {T} because the unsealed
        // table on the left will eventually gain the necessary indexer.
        return SubtypingResult::ok();
      } else {
        return SubtypingResult::fail();
      }
    }

    result.is_error_suppressing = has_error_suppression && should_suppress_errors;
    result
  }

  pub fn is_covariant_with_subtyping_environment_metatable_type_metatable_type_not_null_scope(
    &mut self,
    env: &mut SubtypingEnvironment,
    sub_mt: &MetatableType,
    super_mt: &MetatableType,
    scope: *mut Scope,
  ) -> SubtypingResult {
    self
      .is_covariant_with_subtyping_environment_type_id_type_id_not_null_scope(
        env,
        sub_mt.table(),
        super_mt.table(),
        scope,
      )
      .with_both_component(Component::TypeField(TypeField::Table))
      .and_also(
        self
          .is_covariant_with_subtyping_environment_type_id_type_id_not_null_scope(
            env,
            sub_mt.metatable(),
            super_mt.metatable(),
            scope,
          )
          .with_both_component(Component::TypeField(TypeField::Metatable))
          .to_owned(),
        SubtypingSuppressionPolicy::Any,
      )
      .to_owned()
  }

  pub fn is_covariant_with_subtyping_environment_metatable_type_table_type_not_null_scope(
    &mut self,
    env: &mut SubtypingEnvironment,
    sub_mt: &MetatableType,
    super_table: &TableType,
    scope: *mut Scope,
  ) -> SubtypingResult {
    let sub_table_id = follow_type::follow(sub_mt.table());
    if let Some(sub_table) = get_type::get::<TableType>(sub_table_id) {
      let sub_mt_id = follow_type::follow(sub_mt.metatable());
      if let Some(sub_mt_table) = get_type::get::<TableType>(sub_mt_id)
        && let Some(index_prop) = sub_mt_table.props.get("__index")
        && let Some(read_ty) = index_prop.read_ty
      {
        let index_table_id = follow_type::follow(read_ty);
        if let Some(index_table) = get_type::get::<TableType>(index_table_id) {
          let mut faux_sub_table = sub_table.clone();
          for (name, prop) in &index_table.props {
            if let Some(read_ty) = prop.read_ty
              && !faux_sub_table.props.contains_key(name)
            {
              faux_sub_table
                .props
                .insert(name.clone(), Property::readonly(read_ty));
            }
          }
          return if fflag::LuauSubtypingTablesHasBetterErrorSuppression.get() {
            self.is_covariant_with_subtyping_environment_table_type_table_type_bool_not_null_scope(
              env,
              &faux_sub_table,
              super_table,
              false,
              scope,
            )
          } else {
            self.is_covariant_with_deprecated(env, &faux_sub_table, super_table, false, scope)
          };
        }
      }
      return if fflag::LuauSubtypingTablesHasBetterErrorSuppression.get() {
        self.is_covariant_with_subtyping_environment_table_type_table_type_bool_not_null_scope(
          env,
          sub_table,
          super_table,
          false,
          scope,
        )
      } else {
        self.is_covariant_with_deprecated(env, sub_table, super_table, false, scope)
      };
    }
    SubtypingResult::fail()
  }

  pub fn is_covariant_with_subtyping_environment_metatable_type_primitive_type_not_null_scope(
    &mut self,
    env: &mut SubtypingEnvironment,
    sub_mt: &MetatableType,
    super_prim: &PrimitiveType,
    scope: *mut Scope,
  ) -> SubtypingResult {
    if super_prim.r#type == PrimitiveType::TABLE {
      let followed = follow_type::follow(sub_mt.table());
      if let Some(sub_table) = get_type::get::<TableType>(followed) {
        return self
          .is_covariant_with_subtyping_environment_table_type_primitive_type_not_null_scope(
            env, sub_table, super_prim, scope,
          );
      } else if let Some(sub_nested_mt) = get_type::get::<MetatableType>(followed) {
        return self
          .is_covariant_with_subtyping_environment_metatable_type_primitive_type_not_null_scope(
            env,
            sub_nested_mt,
            super_prim,
            scope,
          );
      }
    }
    SubtypingResult::fail()
  }

  pub fn is_covariant_with_subtyping_environment_extern_type_extern_type_not_null_scope(
    &mut self,
    _env: &mut SubtypingEnvironment,
    sub_extern_type: &ExternType,
    super_extern_type: &ExternType,
    _scope: *mut Scope,
  ) -> SubtypingResult {
    SubtypingResult::from_is_subtype(is_subclass_extern_type_extern_type(
      sub_extern_type,
      super_extern_type,
    ))
  }

  pub fn is_covariant_with_subtyping_environment_type_id_extern_type_type_id_table_type_not_null_scope(
    &mut self,
    env: &mut SubtypingEnvironment,
    sub_ty: TypeId,
    sub_extern_type: &ExternType,
    super_ty: TypeId,
    super_table: &TableType,
    scope: *mut Scope,
  ) -> SubtypingResult {
    let mut result = SubtypingResult::ok();

    *env.substitutions.get_or_insert(super_ty) = sub_ty;

    for (name, prop) in &super_table.props {
      if let Some(prop_ref) = lookup_extern_type_prop(sub_extern_type, name) {
        result.and_also(
          self
            .is_covariant_with_subtyping_environment_property_property_string_bool_not_null_scope(
              env,
              prop_ref,
              prop,
              name.as_ref(),
              false,
              scope,
            ),
          SubtypingSuppressionPolicy::Any,
        );
      } else {
        result.is_subtype = false;
        break;
      }
    }

    if let (Some(sub_indexer), Some(super_indexer)) = (
      sub_extern_type.indexer.as_ref(),
      super_table.indexer.as_ref(),
    ) {
      result.and_also(
        self.is_covariant_with_subtyping_environment_table_indexer_table_indexer_not_null_scope(
          env,
          sub_indexer,
          super_indexer,
          scope,
        ),
        SubtypingSuppressionPolicy::Any,
      );
    } else if super_table.indexer.is_some() && sub_extern_type.indexer.is_none() {
      result.is_subtype = false;
    }

    *env.substitutions.get_or_insert(super_ty) = null();

    result
  }

  /// C++ `isContravariantWith(env, subTp, superTp, scope)` instantiated for the
  /// `TypePackId` overload set: `isCovariantWith(env, superTp, subTp, scope)`
  /// (note the swap) followed by the contravariant reasoning transform. We
  /// inline it here rather than routing through the generic `isContravariantWith`
  /// helper, because that helper's `IntoCovOperand` dispatch models only the
  /// `TypeId` / `TableIndexer` instantiations; the pack overload is selected by
  /// C++ overload resolution and must call the pack `isCovariantWith` directly.
  fn is_contravariant_with_packs(
    &mut self,
    env: &mut SubtypingEnvironment,
    sub_tp: TypePackId,
    super_tp: TypePackId,
    scope: *mut Scope,
  ) -> SubtypingResult {
    // SAFETY: sub_tp/super_tp 为调用方传入的 arena pack 句柄（C++ NotNull 语义），scope 沿
    // 本链的 NotNull<Scope> 契约非空，env 独占借用——满足被调 unsafe fn 的逐项入参要求。
    let mut result = unsafe {
      self.is_covariant_with_subtyping_environment_type_pack_id_type_pack_id_not_null_scope(
        env, super_tp, sub_tp, scope,
      )
    };

    if result.reasoning.empty() {
      result.reasoning.insert(SubtypingReasoning {
        sub_path: Path::default(),
        super_path: Path::default(),
        variance: SubtypingVariance::Contravariant,
        is_property_modifier_violation: false,
      });
    } else {
      let mut updated = SubtypingReasonings::new(k_empty_reasoning());
      for r in result.reasoning.iter() {
        let mut r = r.clone();
        swap(&mut r.sub_path, &mut r.super_path);
        if r.variance == SubtypingVariance::Covariant {
          r.variance = SubtypingVariance::Contravariant;
        } else if r.variance == SubtypingVariance::Contravariant {
          r.variance = SubtypingVariance::Covariant;
        }
        updated.insert(r);
      }
      result.reasoning = updated;
    }

    result
  }

  /// 对应 C++ `Subtyping::isCovariantWith(env, const FunctionType*, const FunctionType*,
  /// NotNull<Scope>)`（`cpp/Analysis/src/Subtyping.cpp:2373`）。
  ///
  /// # Safety
  /// - `scope` 须为非空且存活的 [`Scope`] 指针（C++ `NotNull<Scope>`）——泛型计数不匹配分支
  ///   会读取 `scope->location`。
  /// - `sub_function`/`super_function` 须借用 arena 中的 `FunctionType` 节点，在本次同步
  ///   比较期间有效且不被外部改写（其 arg/ret pack 句柄会被解引用比较）。
  /// - 要求 `self` 的 `arena`/`builtin_types`/`normalizer`/`ice_reporter` 裸成员指向存活对象
  ///   （截断隐藏可变参、递归 pack 比较会解引用），由 `Subtyping::subtyping_owned` 构造契约保证。
  pub unsafe fn is_covariant_with_subtyping_environment_function_type_function_type_not_null_scope(
    &mut self,
    env: &mut SubtypingEnvironment,
    sub_function: &FunctionType,
    super_function: &FunctionType,
    scope: *mut Scope,
  ) -> SubtypingResult {
    let mut result = SubtypingResult::default();

    if !sub_function.generics.is_empty() {
      for &g in sub_function.generics.iter() {
        let g = follow_type::follow(g);
        if get_type::get::<GenericType>(g).is_some() {
          if let Some(bounds) = env.mapped_generics.find_mut(&g) {
            // g may shadow an existing generic, so push a fresh set of bounds
            bounds.push(GenericBounds::default());
          } else {
            *env.mapped_generics.get_or_insert(g) = alloc::vec![GenericBounds::default()];
          }
        }
      }
    }

    if !sub_function.generic_packs.is_empty() {
      let mut packs: Vec<TypePackId> = Vec::with_capacity(sub_function.generic_packs.len());

      for &g in sub_function.generic_packs.iter() {
        let g = follow_type_pack::follow(g);
        if get_type_pack::get::<GenericTypePack>(g).is_some() {
          packs.push(g);
        }
      }

      env.mapped_generic_packs.push_frame(&packs);
    }

    {
      let mut arg_result = self.is_contravariant_with_packs(
        env,
        sub_function.arg_types,
        super_function.arg_types,
        scope,
      );
      arg_result.with_both_component(Component::PackField(PackField::Arguments));
      result.or_else(arg_result);

      // If subtyping failed in the argument packs, we should check if there's a hidden variadic tail and try ignoring it.
      // This might cause subtyping correctly because the sub type here may not have a hidden variadic tail or equivalent.
      if !result.is_subtype {
        let (arguments, tail) = flatten_type_pack_id(super_function.arg_types);

        let hidden_variadic = match tail {
          Some(t) => get_type_pack::get::<VariadicTypePack>(t).is_some_and(|v| v.hidden),
          None => false,
        };

        if hidden_variadic {
          let truncated = {
            // SAFETY: arena 为构造契约传入且存活；新 pack 由 arena bump 分配持有，
            // 追加不会使既有句柄失效（C++ arena->addTypePack 同构）。
            self
              .arena
              .get_mut()
              .add_type_pack_vector_type_id_optional_type_pack_id(arguments, None)
          };
          let mut retry =
            self.is_contravariant_with_packs(env, sub_function.arg_types, truncated, scope);
          retry.with_both_component(Component::PackField(PackField::Arguments));
          result.or_else(retry);
        }
      }
    }

    let mut ret_result = unsafe {
      // SAFETY: 良好构造的 FunctionType 的 ret_types 为非空 arena pack 句柄（C++ 直接以
      // subFunction->retTypes 传参），scope/env 契约同上，满足被调 unsafe fn 入参要求。
      self.is_covariant_with_subtyping_environment_type_pack_id_type_pack_id_not_null_scope(
        env,
        sub_function.ret_types,
        super_function.ret_types,
        scope,
      )
    };
    ret_result.with_both_component(Component::PackField(PackField::Returns));
    result.and_also(ret_result, SubtypingSuppressionPolicy::Any);

    // SAFETY: 正常 FunctionType 的 arg_types/ret_types 必为 arena 分配的非空 TypePackVar
    // 句柄（C++ 同样直接解引用比较）；四个解引用均为只读结构比较，无重叠可变借用。
    if unsafe {
      *sub_function.arg_types == *super_function.arg_types
        && *sub_function.ret_types == *super_function.ret_types
    } {
      // It's fine to upcast a function with generics to a function without.
      // Intuitively: a generic function should always be a subtype of its instantiations.
      if super_function.generics.len() != sub_function.generics.len()
        && !super_function.generics.is_empty()
      {
        result.and_also(SubtypingResult::fail(), SubtypingSuppressionPolicy::Any);
        result.with_error(TypeError {
          // SAFETY: scope 非空且存活（NotNull<Scope> 契约），仅值读取 location。
          location: unsafe { (*scope).location },
          module_name: ModuleName::new(),
          data: TypeErrorData::GenericTypeCountMismatch(GenericTypeCountMismatch {
            sub_ty_generic_count: super_function.generics.len(),
            super_ty_generic_count: sub_function.generics.len(),
          }),
        });
      }

      if super_function.generic_packs.len() != sub_function.generic_packs.len()
        && !super_function.generic_packs.is_empty()
      {
        result.and_also(SubtypingResult::fail(), SubtypingSuppressionPolicy::Any);
        result.with_error(TypeError {
          // SAFETY: 同上——scope 由整条 covariant 链保持有效，读取 location 构造 pack 计数错误。
          location: unsafe { (*scope).location },
          module_name: ModuleName::new(),
          data: TypeErrorData::GenericTypePackCountMismatch(GenericTypePackCountMismatch {
            sub_ty_generic_pack_count: super_function.generic_packs.len(),
            super_ty_generic_pack_count: sub_function.generic_packs.len(),
          }),
        });
      }
    }

    if !sub_function.generics.is_empty() {
      for &g in sub_function.generics.iter() {
        let g = follow_type::follow(g);
        if let Some(r#gen) = get_type::get::<GenericType>(g) {
          let generic_name = r#gen.name.clone();

          let last_bounds = {
            let bounds = env.mapped_generics.find(&g).expect(
              "cpp LUAU_ASSERT 判据：generic 帧由 bind_generic 在派发前压入 mapped_generics，find 必命中",
            );
            LUAU_ASSERT!(!bounds.is_empty());
            bounds
              .last()
              .expect("上方 LUAU_ASSERT 已断言该帧非空")
              .clone()
          };

          let bounds_result =
            self.subtyping_check_generic_bounds(&last_bounds, env, scope, &generic_name);
          result.and_also(bounds_result, SubtypingSuppressionPolicy::Any);

          env
            .mapped_generics
            .find_mut(&g)
            .expect("上方同键 find 已命中且帧未被改写，find_mut 必命中")
            .pop();
        }
      }
    }

    if !sub_function.generic_packs.is_empty() {
      env.mapped_generic_packs.pop_frame();
      // This result isn't cacheable, because we may need it to populate the generic pack mapping environment again later
      result.is_cacheable = false;
    }

    result
  }

  pub fn is_covariant_with_subtyping_environment_table_type_primitive_type_not_null_scope(
    &mut self,
    _env: &mut SubtypingEnvironment,
    _sub_table: &TableType,
    super_prim: &PrimitiveType,
    _scope: *mut Scope,
  ) -> SubtypingResult {
    let mut result = SubtypingResult::default();
    if super_prim.r#type == PrimitiveType::TABLE {
      result.is_subtype = true;
    }
    result
  }

  pub fn is_covariant_with_subtyping_environment_primitive_type_table_type_not_null_scope(
    &mut self,
    env: &mut SubtypingEnvironment,
    sub_prim: &PrimitiveType,
    super_table: &TableType,
    scope: *mut Scope,
  ) -> SubtypingResult {
    let mut result = SubtypingResult::fail();

    if sub_prim.r#type == PrimitiveType::STRING {
      // SAFETY: builtin_types 裸指针由构造契约保证有效；解引用不借用 self。
      let bt = self.builtin_types.get();
      if let Some(metatable) = get_metatable_type_id_not_null_builtin_types(bt.string_type, bt)
        && let Some(mttv) = get_type::get::<TableType>(follow_type::follow(metatable))
        && let Some(it) = mttv.props.get("__index")
      {
        // the `string` metatable should not have any write-only types.
        LUAU_ASSERT!(
          !it
            .read_ty
            .expect("string 元表 __index 属性由内建定义接线 read_ty，恒 Some（上方注释契约）")
            .is_null()
        );

        if let Some(string_table) = get_type::get::<TableType>(
          it.read_ty
            .expect("string 元表 __index 属性由内建定义接线 read_ty，恒 Some（上方注释契约）"),
        ) {
          if fflag::LuauSubtypingTablesHasBetterErrorSuppression.get() {
            let mut sub_result = self
              .is_covariant_with_subtyping_environment_table_type_table_type_bool_not_null_scope(
                env,
                string_table,
                super_table,
                false,
                scope,
              );
            sub_result.with_sub_path(
              PathBuilder {
                components: Vec::new(),
              }
              .mt()
              .read_prop("__index")
              .build(),
            );
            result.or_else(sub_result);
          } else {
            let mut sub_result =
              self.is_covariant_with_deprecated(env, string_table, super_table, false, scope);
            sub_result.with_sub_path(
              PathBuilder {
                components: Vec::new(),
              }
              .mt()
              .read_prop("__index")
              .build(),
            );
            result.or_else(sub_result);
          }
        }
      }
    } else if sub_prim.r#type == PrimitiveType::TABLE {
      let is_subtype = super_table.props.is_empty()
        && (super_table.indexer.is_none() || super_table.state == TableState::Generic);
      result.is_subtype = is_subtype;
      return result;
    }

    result
  }

  pub fn is_covariant_with_subtyping_environment_singleton_type_table_type_not_null_scope(
    &mut self,
    env: &mut SubtypingEnvironment,
    sub_singleton: &SingletonType,
    super_table: &TableType,
    scope: *mut Scope,
  ) -> SubtypingResult {
    let mut result = SubtypingResult::fail();

    // SAFETY: builtin_types 裸指针由构造契约保证有效；解引用不借用 self。
    let bt = self.builtin_types.get();
    if get_singleton_type::<StringSingleton>(sub_singleton).is_some()
      && let Some(metatable) = get_metatable_type_id_not_null_builtin_types(bt.string_type, bt)
      && let Some(mttv) = get_type::get::<TableType>(follow_type::follow(metatable))
      && let Some(it) = mttv.props.get("__index")
    {
      // the `string` metatable should not have any write-only types.
      LUAU_ASSERT!(
        !it
          .read_ty
          .expect("string 元表 __index 属性由内建定义接线 read_ty，恒 Some（上方注释契约）")
          .is_null()
      );

      if let Some(string_table) = get_type::get::<TableType>(
        it.read_ty
          .expect("string 元表 __index 属性由内建定义接线 read_ty，恒 Some（上方注释契约）"),
      ) {
        if fflag::LuauSubtypingTablesHasBetterErrorSuppression.get() {
          let mut sub_result = self
            .is_covariant_with_subtyping_environment_table_type_table_type_bool_not_null_scope(
              env,
              string_table,
              super_table,
              false,
              scope,
            );
          let mut pb = PathBuilder {
            components: Vec::new(),
          };
          pb.mt();
          pb.read_prop("__index");
          sub_result.with_sub_path(pb.build());
          result.or_else(sub_result);
        } else {
          let mut sub_result =
            self.is_covariant_with_deprecated(env, string_table, super_table, false, scope);
          let mut pb = PathBuilder {
            components: Vec::new(),
          };
          pb.mt();
          pb.read_prop("__index");
          sub_result.with_sub_path(pb.build());
          result.or_else(sub_result);
        }
      }
    }

    result
  }

  pub fn is_covariant_with_subtyping_environment_table_indexer_table_indexer_not_null_scope(
    &mut self,
    env: &mut SubtypingEnvironment,
    sub_indexer: &TableIndexer,
    super_indexer: &TableIndexer,
    scope: *mut Scope,
  ) -> SubtypingResult {
    if fflag::LuauReadOnlyIndexers.get() {
      let mut result = SubtypingResult::fail();

      if sub_indexer.is_read_only && !super_indexer.is_read_only {
        result.with_both_component(Component::TypeField(TypeField::IndexResult));
        return result;
      }

      result = self.is_invariant_with_subtyping_environment_sub_ty_super_ty_not_null_scope(
        env,
        sub_indexer.index_type,
        super_indexer.index_type,
        scope,
      );
      result.with_both_component(Component::TypeField(TypeField::IndexLookup));

      let mut value_result = if super_indexer.is_read_only {
        self.is_covariant_with_subtyping_environment_type_id_type_id_not_null_scope(
          env,
          sub_indexer.index_result_type,
          super_indexer.index_result_type,
          scope,
        )
      } else {
        self.is_invariant_with_subtyping_environment_sub_ty_super_ty_not_null_scope(
          env,
          sub_indexer.index_result_type,
          super_indexer.index_result_type,
          scope,
        )
      };
      value_result.with_both_component(Component::TypeField(TypeField::IndexResult));
      result.and_also(value_result, SubtypingSuppressionPolicy::Any);

      result
    } else {
      let mut result = self.is_invariant_with_subtyping_environment_sub_ty_super_ty_not_null_scope(
        env,
        sub_indexer.index_type,
        super_indexer.index_type,
        scope,
      );
      result.with_both_component(Component::TypeField(TypeField::IndexLookup));

      let mut value_result = self
        .is_invariant_with_subtyping_environment_sub_ty_super_ty_not_null_scope(
          env,
          sub_indexer.index_result_type,
          super_indexer.index_result_type,
          scope,
        );
      value_result.with_both_component(Component::TypeField(TypeField::IndexResult));
      result.and_also(value_result, SubtypingSuppressionPolicy::Any);
      result
    }
  }
}

fn property_component(name: &str, read: bool) -> Component {
  Component::Property(PathProperty {
    name: name.to_string(),
    is_read: read,
  })
}

impl Subtyping {
  pub fn is_covariant_with_subtyping_environment_property_property_string_bool_not_null_scope(
    &mut self,
    env: &mut SubtypingEnvironment,
    sub_prop: &Property,
    super_prop: &Property,
    name: &str,
    force_covariant_test: bool,
    scope: *mut Scope,
  ) -> SubtypingResult {
    let mut res = SubtypingResult::ok();

    if super_prop.is_shared() && sub_prop.is_shared() {
      let sub_ty = sub_prop
        .read_ty
        .expect("is_shared() 定义蕴含 read_ty 为 Some（property_is_shared 首条件）");
      let super_ty = super_prop
        .read_ty
        .expect("is_shared() 定义蕴含 read_ty 为 Some（property_is_shared 首条件）");
      let mut part = if force_covariant_test {
        self.is_covariant_with_subtyping_environment_type_id_type_id_not_null_scope(
          env, sub_ty, super_ty, scope,
        )
      } else {
        self.is_invariant_with_subtyping_environment_sub_ty_super_ty_not_null_scope(
          env, sub_ty, super_ty, scope,
        )
      };
      part.with_both_component(property_component(name, true));
      res.and_also(part, SubtypingSuppressionPolicy::Any);
    } else {
      if let (Some(sub_read), Some(super_read)) = (sub_prop.read_ty, super_prop.read_ty) {
        let mut part = self.is_covariant_with_subtyping_environment_type_id_type_id_not_null_scope(
          env, sub_read, super_read, scope,
        );
        part.with_both_component(property_component(name, true));
        res.and_also(part, SubtypingSuppressionPolicy::Any);
      }

      if let (Some(sub_write), Some(super_write)) = (sub_prop.write_ty, super_prop.write_ty)
        && !force_covariant_test
      {
        let mut part = self
          .is_contravariant_with_subtyping_environment_sub_ty_super_ty_not_null_scope(
            env,
            sub_write,
            super_write,
            scope,
          );
        part.with_both_component(property_component(name, false));
        res.and_also(part, SubtypingSuppressionPolicy::Any);
      }

      let super_is_read_write = super_prop.read_ty.is_some() && super_prop.write_ty.is_some();
      let sub_is_read_only = sub_prop.read_ty.is_some() && sub_prop.write_ty.is_none();
      let sub_is_write_only = sub_prop.read_ty.is_none() && sub_prop.write_ty.is_some();

      if super_is_read_write {
        if sub_is_read_only {
          let mut part = SubtypingResult::fail();
          part.with_both_component(property_component(name, true));
          if fflag::LuauPropertyModifierMismatchErrors.get() {
            part.with_property_modifier_violation();
          }
          res.and_also(part, SubtypingSuppressionPolicy::Any);
        } else if sub_is_write_only {
          let mut part = SubtypingResult::fail();
          part.with_both_component(property_component(name, false));
          if fflag::LuauPropertyModifierMismatchErrors.get() {
            part.with_property_modifier_violation();
          }
          res.and_also(part, SubtypingSuppressionPolicy::Any);
        }
      }
    }

    res
  }

  pub fn is_covariant_with_subtyping_environment_shared_ptr_normalized_type_shared_ptr_normalized_type_not_null_scope(
    &mut self,
    env: &mut SubtypingEnvironment,
    sub_norm: &Arc<NormalizedType>,
    super_norm: &Arc<NormalizedType>,
    scope: *mut Scope,
  ) -> SubtypingResult {
    let mut result = self.is_covariant_with_subtyping_environment_type_id_type_id_not_null_scope(
      env,
      sub_norm.tops,
      super_norm.tops,
      scope,
    );
    result.and_also(
      self.is_covariant_with_subtyping_environment_type_id_type_id_not_null_scope(
        env,
        sub_norm.booleans,
        super_norm.booleans,
        scope,
      ),
      SubtypingSuppressionPolicy::Any,
    );

    let mut extern_result = self.is_covariant_with_subtyping_environment_normalized_extern_type_normalized_extern_type_not_null_scope(env, &sub_norm.extern_types, &super_norm.extern_types, scope);
    extern_result.or_else(
      self.is_covariant_with_subtyping_environment_normalized_extern_type_type_ids_not_null_scope(
        env,
        &sub_norm.extern_types,
        &super_norm.tables,
        scope,
      ),
    );
    result.and_also(extern_result, SubtypingSuppressionPolicy::Any);

    result.and_also(
      self.is_covariant_with_subtyping_environment_type_id_type_id_not_null_scope(
        env,
        sub_norm.errors,
        super_norm.errors,
        scope,
      ),
      SubtypingSuppressionPolicy::Any,
    );
    result.and_also(
      self.is_covariant_with_subtyping_environment_type_id_type_id_not_null_scope(
        env,
        sub_norm.nils,
        super_norm.nils,
        scope,
      ),
      SubtypingSuppressionPolicy::Any,
    );
    result.and_also(
      self.is_covariant_with_subtyping_environment_type_id_type_id_not_null_scope(
        env,
        sub_norm.numbers,
        super_norm.numbers,
        scope,
      ),
      SubtypingSuppressionPolicy::Any,
    );

    let mut string_result = self.is_covariant_with_subtyping_environment_normalized_string_type_normalized_string_type_not_null_scope(env, &sub_norm.strings, &super_norm.strings, scope);
    string_result.or_else(
      self.is_covariant_with_subtyping_environment_normalized_string_type_type_ids_not_null_scope(
        env,
        &sub_norm.strings,
        &super_norm.tables,
        scope,
      ),
    );
    result.and_also(string_result, SubtypingSuppressionPolicy::Any);

    result.and_also(
      self.is_covariant_with_subtyping_environment_type_id_type_id_not_null_scope(
        env,
        sub_norm.threads,
        super_norm.threads,
        scope,
      ),
      SubtypingSuppressionPolicy::Any,
    );
    result.and_also(
      self.is_covariant_with_subtyping_environment_type_id_type_id_not_null_scope(
        env,
        sub_norm.buffers,
        super_norm.buffers,
        scope,
      ),
      SubtypingSuppressionPolicy::Any,
    );
    result.and_also(
      self.is_covariant_with_subtyping_environment_type_ids_type_ids_not_null_scope(
        env,
        &sub_norm.tables,
        &super_norm.tables,
        scope,
      ),
      SubtypingSuppressionPolicy::Any,
    );
    result.and_also(self.is_covariant_with_subtyping_environment_normalized_function_type_normalized_function_type_not_null_scope(env, &sub_norm.functions, &super_norm.functions, scope), SubtypingSuppressionPolicy::Any);

    result
  }

  pub fn is_covariant_with_subtyping_environment_normalized_extern_type_normalized_extern_type_not_null_scope(
    &mut self,
    env: &mut SubtypingEnvironment,
    sub_extern_type: &NormalizedExternType,
    super_extern_type: &NormalizedExternType,
    scope: *mut Scope,
  ) -> SubtypingResult {
    for sub_extern_type_ty in sub_extern_type.extern_types.keys() {
      let mut result = SubtypingResult::default();

      for (super_extern_type_ty, super_negations) in &super_extern_type.extern_types {
        let candidate = self
          .is_covariant_with_subtyping_environment_type_id_type_id_not_null_scope(
            env,
            *sub_extern_type_ty,
            *super_extern_type_ty,
            scope,
          );
        result.or_else(candidate);
        if !result.is_subtype {
          continue;
        }

        for negation in &super_negations.order {
          let negated = SubtypingResult::negate(
            &self.is_covariant_with_subtyping_environment_type_id_type_id_not_null_scope(
              env,
              *sub_extern_type_ty,
              *negation,
              scope,
            ),
          );
          result.and_also(negated, SubtypingSuppressionPolicy::Any);
          if result.is_subtype {
            break;
          }
        }
      }

      if !result.is_subtype {
        return result;
      }
    }

    SubtypingResult::ok()
  }
}
