use alloc::{collections::BTreeMap, string::String, sync::Arc, vec::Vec};
use core::{
  mem::transmute,
  ptr::{null, null_mut},
};

use ulua_ast::records::location::Location;
use ulua_common::{
  FInt,
  records::{
    dense_hash_map::DenseHashMap, dense_hash_set::DenseHashSet, dense_hash_table::DenseDefault,
  },
};

use crate::{
  enums::{solver_mode::SolverMode, variance::Variance},
  functions::{follow_type::follow_type_id, get_type_alt_j::get_type_id},
  records::{
    any_type::AnyType, builtin_types::BuiltinTypes, count_mismatch::CountMismatchContext,
    internal_error_reporter::InternalErrorReporter, module::Module, normalizer::Normalizer,
    pending_type::PendingType, pending_type_pack::PendingTypePack, scope::Scope,
    subtyping::Subtyping, txn_log::TxnLog, r#type::Type, type_arena::TypeArena,
    type_check_limits::TypeCheckLimits, type_function_runtime::TypeFunctionRuntime,
    type_id_pair_hash::TypeIdPairHash, type_pack_var::TypePackVar, unifiable::Error,
    unifier::Unifier, unifier_shared_state::UnifierSharedState, union_type::UnionType,
  },
  type_aliases::{
    seen_type_pack_set::SeenTypePackSet, type_id::TypeId, type_pack_id::TypePackId,
    type_pack_variant::TypePackVariant, type_variant::TypeVariant,
  },
};
impl DenseDefault for Box<PendingType> {
  fn dense_default() -> Self {
    Box::new(PendingType {
      pending: Type::new(TypeVariant::Any(AnyType::default())),
      dead: true,
    })
  }
}

impl DenseDefault for Box<PendingTypePack> {
  fn dense_default() -> Self {
    Box::new(PendingTypePack {
      pending: TypePackVar {
        ty: TypePackVariant::Error(Error::<TypePackId>::new()),
        persistent: false,
        owning_arena: null_mut(),
      },
    })
  }
}

fn empty_seen_type_pack_set() -> SeenTypePackSet {
  let type_seen: DenseHashMap<(TypeId, TypeId), bool, TypeIdPairHash> =
    DenseHashMap::new((null(), null()));
  unsafe { transmute(type_seen) }
}

/// C++ `checkTypeMatch(...)`.
/// # Safety
/// 调用方须保证满足 C++ 原实现的调用契约。
pub unsafe fn check_type_match(
  module: &Module,
  sub_ty: TypeId,
  super_ty: TypeId,
  scope: *mut Scope,
  type_arena: *mut TypeArena,
  builtin_types: *mut BuiltinTypes,
) -> bool {
  let sub_ty = follow_type_id(sub_ty);
  let super_ty = follow_type_id(super_ty);

  if let Some(super_union) = get_type_id::<UnionType>(super_ty) {
    return super_union.options.iter().any(|&option| unsafe {
      check_type_match(module, sub_ty, option, scope, type_arena, builtin_types)
    });
  }

  if let Some(sub_union) = get_type_id::<UnionType>(sub_ty) {
    return sub_union.options.iter().all(|&option| unsafe {
      check_type_match(module, option, super_ty, scope, type_arena, builtin_types)
    });
  }

  let mut ice_reporter = InternalErrorReporter {
    on_internal_error: None,
    module_name: String::new(),
  };
  let mut unifier_state = UnifierSharedState::new(&mut ice_reporter as *mut InternalErrorReporter);

  let solver_mode = if module.checked_in_new_solver {
    SolverMode::New
  } else {
    SolverMode::Old
  };

  let mut normalizer = Normalizer {
    cached_normals: BTreeMap::new(),
    cached_intersections: BTreeMap::new(),
    cached_unions: BTreeMap::new(),
    cached_type_ids: BTreeMap::new(),
    cached_is_inhabited: DenseHashMap::new(null()),
    cached_is_inhabited_intersection: DenseHashMap::new((null(), null())),
    fuel: None,
    arena: null_mut(),
    builtin_types: null_mut(),
    shared_state: null_mut(),
    cache_inhabitance: false,
    solver_mode,
  };
  normalizer
    .normalizer_type_arena_not_null_builtin_types_not_null_unifier_shared_state_solver_mode_bool(
      type_arena,
      builtin_types,
      &mut unifier_state as *mut UnifierSharedState,
      solver_mode,
      false,
    );

  if module.checked_in_new_solver {
    let limits = TypeCheckLimits::default();
    unsafe { Arc::increment_strong_count(scope) };
    let root_scope = unsafe { Arc::from_raw(scope) };
    let mut type_function_runtime = TypeFunctionRuntime {
      ice: ice_reporter.clone(),
      limits: limits.clone(),
      type_arena: Default::default(),
      type_pack_arena: Default::default(),
      state: (null_mut(), None),
      initialized: DenseHashSet::new(null_mut()),
      allow_evaluation: true,
      root_scope,
      messages: Vec::new(),
      runtime_builder: null_mut(),
    };

    unifier_state.counters.recursion_limit = FInt::LuauTypeInferRecursionLimit.get();
    unifier_state.counters.iteration_limit = FInt::LuauTypeInferIterationLimit.get();

    let mut subtyping = Subtyping {
      builtin_types: null_mut(),
      arena: null_mut(),
      normalizer: null_mut(),
      type_function_runtime: null_mut(),
      ice_reporter: null_mut(),
      limits: TypeCheckLimits::default(),
      unique_types: null(),
      seen_types: DenseHashMap::new((null(), null())),
      seen_packs: empty_seen_type_pack_set(),
      result_cache: DenseHashMap::new((null(), null())),
    };
    subtyping.subtyping_not_null_builtin_types_not_null_type_arena_not_null_normalizer_not_null_type_function_runtime_not_null_internal_error_reporter(
                builtin_types,
                type_arena,
                &mut normalizer as *mut Normalizer,
                &mut type_function_runtime as *mut TypeFunctionRuntime,
                &mut ice_reporter as *mut InternalErrorReporter,
            );

    let result = subtyping.is_subtype_type_id_type_id_not_null_scope(sub_ty, super_ty, scope);

    result.is_subtype
  } else {
    let location = Location::default();
    let mut unifier = Unifier {
      types: type_arena,
      builtin_types,
      normalizer: &mut normalizer as *mut Normalizer,
      scope,
      log: TxnLog {
        type_var_changes: DenseHashMap::new(null()),
        type_pack_changes: DenseHashMap::new(null()),
        parent: null_mut(),
        owned_seen: Vec::new(),
        // Empty; lazily owns a box on first `push_seen` (freed on drop).
        shared_seen: null_mut(),
        owned_seen_box: None,
        radioactive: false,
      },
      failure: false,
      errors: Vec::new(),
      location,
      variance: Variance::Covariant,
      normalize: true,
      check_inhabited: true,
      ctx: CountMismatchContext::Arg,
      shared_state: &mut unifier_state as *mut UnifierSharedState,
      blocked_types: Vec::new(),
      blocked_type_packs: Vec::new(),
      first_pack_error_pos: None,
    };

    // Cost of normalization can be too high for autocomplete response time requirements
    unifier.normalize = false;
    unifier.check_inhabited = false;

    unifier_state.counters.recursion_limit = FInt::LuauTypeInferRecursionLimit.get();
    unifier_state.counters.iteration_limit = FInt::LuauTypeInferIterationLimit.get();

    let errors = unifier.can_unify_type_id_type_id(sub_ty, super_ty);
    errors.is_empty()
  }
}
