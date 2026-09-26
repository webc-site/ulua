//! C++ `TypeFunctionReductionResult<TypeId> refineTypeFunction(TypeId instance,
//! const std::vector<TypeId>& typeParams, const std::vector<TypePackId>&
//! packParams, NotNull<TypeFunctionContext> ctx)`
//! (BuiltinTypeFunctions.cpp:1207-1432). The `refine` reducer.
use alloc::{vec, vec::Vec};
// Wire the two refinement visitors into `GenericTypeVisitorTrait` so the base
// `traverse_type_id` dispatches through their `visit_*` overrides. The inherent
// methods on `ContainsRefinableType` / `FindRefinementBlockers` carry the real
// logic; these impls just forward to them.
use core::ptr::{null, null_mut};

use ulua_common::{dfint, macros::luau_assert::LUAU_ASSERT, records::dense_hash_set::DenseHashSet};

use crate::{
  functions::{
    add_union::add_union, follow_type, get_type,
    intersect_with_simple_discriminant::intersect_with_simple_discriminant,
    is_blocked_or_unsolved_type::is_blocked_or_unsolved_type, is_pending::is_pending,
    is_truthy_or_falsy_type::is_truthy_or_falsy_type, occurs_builtin_type_functions::occurs,
    simplify_intersection_simplify::simplify_intersection, simplify_union::simplify_union,
  },
  records::{
    arena_handle::Handle,
    blocked_type::BlockedType,
    contains_refinable_type::ContainsRefinableType,
    extern_type::ExternType,
    find_refinement_blockers::FindRefinementBlockers,
    free_type::FreeType,
    function_type::FunctionType,
    generic_type::GenericType,
    generic_type_visitor::{GenericTypeVisitor, GenericTypeVisitorTrait},
    intersection_type::IntersectionType,
    metatable_type::MetatableType,
    negation_type::NegationType,
    no_refine_type::NoRefineType,
    pending_expansion_type::PendingExpansionType,
    primitive_type::PrimitiveType,
    recursion_limiter::RecursionLimiter,
    refine_type_scrubber::RefineTypeScrubber,
    table_type::TableType,
    type_function_context::TypeFunctionContext,
    type_function_reduction_result::TypeFunctionReductionResult,
    union_type::UnionType,
    unknown_type::UnknownType,
  },
  type_aliases::{type_id::TypeId, type_pack_id::TypePackId},
};
impl GenericTypeVisitorTrait for ContainsRefinableType {
  type Seen = DenseHashSet<*mut ()>;

  fn visitor_base(&mut self) -> &mut GenericTypeVisitor<Self::Seen> {
    &mut self.base.base
  }

  // `bool visit(TypeId) override { found = true; return false; }`
  fn visit_type_id(&mut self, ty: TypeId) -> bool {
    self.visit(ty)
  }
  fn visit_type_id_no_refine_type(&mut self, ty: TypeId, nrt: &NoRefineType) -> bool {
    self.visit_no_refine(ty, nrt)
  }
  fn visit_type_id_table_type(&mut self, ty: TypeId, ttv: &TableType) -> bool {
    self.visit_table(ty, ttv)
  }
  fn visit_type_id_metatable_type(&mut self, ty: TypeId, mtv: &MetatableType) -> bool {
    self.visit_metatable(ty, mtv)
  }
  fn visit_type_id_function_type(&mut self, ty: TypeId, ftv: &FunctionType) -> bool {
    self.visit_function(ty, ftv)
  }
  fn visit_type_id_union_type(&mut self, ty: TypeId, utv: &UnionType) -> bool {
    self.visit_union(ty, utv)
  }
  fn visit_type_id_intersection_type(&mut self, ty: TypeId, itv: &IntersectionType) -> bool {
    self.visit_intersection(ty, itv)
  }
  fn visit_type_id_negation_type(&mut self, ty: TypeId, ntv: &NegationType) -> bool {
    self.visit_negation(ty, ntv)
  }
}

impl GenericTypeVisitorTrait for FindRefinementBlockers {
  type Seen = DenseHashSet<*mut ()>;

  fn visitor_base(&mut self) -> &mut GenericTypeVisitor<Self::Seen> {
    &mut self.base.base
  }

  fn visit_type_id_blocked_type(&mut self, ty: TypeId, btv: &BlockedType) -> bool {
    self.visit_blocked_type(ty, btv)
  }
  fn visit_type_id_pending_expansion_type(
    &mut self,
    ty: TypeId,
    petv: &PendingExpansionType,
  ) -> bool {
    self.visit_pending_expansion_type(ty, petv)
  }
  fn visit_type_id_extern_type(&mut self, ty: TypeId, etv: &ExternType) -> bool {
    self.visit_extern_type(ty, etv)
  }
}

/// Result of `stepRefine`: `(result : TypeId (null on failure), toBlockOn :
/// Vec<TypeId>)`.
type StepResult = (TypeId, Vec<TypeId>);

/// # Safety
///
/// C++ `refineTypeFunction(...)` 直译，经 `ReducerFunction` 指针在分派点物化调用。调用方须保证：
/// - `ctx` 为本次归约独占借用的 `TypeFunctionContext`（分派点自 NonNull 物化，会话期存活）；其
///   `arena`/`builtins`/`normalizer`/`ice` 各 `NonNull` 字段与 `solver` 裸指针均指向
///   存活对象，且整场归约为单线程独占访问（cpp `NotNull<TypeFunctionContext>` 语义），
///   不存在并存的可变借用；
/// - `instance` 为指向 `ctx.arena` 内该类型函数实例节点的存活 `TypeId`；
/// - `type_params`/`pack_params` 各元素均为指向存活类型/pack arena 的有效句柄，
///   且在 `type_params.len() >= 2` 且 `pack_params` 为空时才进入实际归约。
pub unsafe fn refine_type_function(
  instance: TypeId,
  type_params: &[TypeId],
  pack_params: &[TypePackId],
  ctx: &mut TypeFunctionContext,
) -> TypeFunctionReductionResult {
  // Safety: 分派点自 `Handle<TypeFunctionContext>`（构造点为真实 `&mut` 借用）物化的
  // 本次调用独占借用，会话期存活。
  let ctx_ref = &mut *ctx;

  if type_params.len() < 2 || !pack_params.is_empty() {
    ctx_ref.ice().ice_string("refine type function: encountered a type function instance without the required argument structure");
    LUAU_ASSERT!(false);
  }

  let mut target_ty = follow_type::follow(type_params[0]);

  // If we end up minting a refine type like `t1 where t1 = refine<T | t1, Y>`
  // this can create a degenerate set type `t1 where t1 = (T | t1) & Y`. Instead,
  // we clip the recursive part: `refine<T | t1, Y> => refine<T, Y>`.
  if occurs(target_ty, instance) {
    // Safety: 对 ctx_ref 的瞬态独占再借用仅覆盖 `RefineTypeScrubber::new` 调用；scrubber
    // 内部以 Handle 存柄（非空由类型编码）、不持借用，`instance` 为指向 ctx.arena 内实例节点的存活 `TypeId`。
    let mut rts = unsafe { RefineTypeScrubber::new(&mut *ctx_ref, instance) };
    if let Some(result) = rts.substitute_type_id(target_ty) {
      target_ty = result;
    }
  }

  let mut discriminant_types: Vec<TypeId> = Vec::new();
  for &param in type_params.iter().skip(1) {
    let discriminant = follow_type::follow(param);

    // Filter out any top level types that are meaningless to refine against.
    let is_unknown = get_type::get::<UnknownType>(discriminant).is_some();
    let is_no_refine = get_type::get::<NoRefineType>(discriminant).is_some();
    if is_unknown || is_no_refine {
      continue;
    }

    // If the discriminant type is only the `*no-refine*` type, or tables/
    // metatables/unions/intersections/functions/negations containing
    // `*no-refine*`, there's no point in refining against it.
    let mut crt = ContainsRefinableType::new();
    crt.traverse_type_id(discriminant);

    if crt.found {
      discriminant_types.push(discriminant);
    }
  }

  // if we don't have any real refinements, i.e. they're all `*no-refine*`, then
  // we can reduce immediately.
  if discriminant_types.is_empty() {
    return TypeFunctionReductionResult::reduction(target_ty);
  }

  let target_is_pending = is_blocked_or_unsolved_type(target_ty);

  // check to see if both operand types are resolved enough, and wait to reduce if not
  if target_is_pending {
    return TypeFunctionReductionResult::no_reduction(vec![target_ty]);
  } else {
    for &t in &discriminant_types {
      // Safety: `ctx_ref.solver` 依入参契约指向存活 `ConstraintSolver`；`t` 为存活
      // `TypeId`。`is_pending` 仅只读探查 solver，借用半径止于本条件表达式。
      if unsafe { is_pending(t, ctx_ref.solver) } {
        return TypeFunctionReductionResult::no_reduction(vec![t]);
      }
    }
  }

  // If we have a blocked type in the target, we *could* potentially refine it,
  // but more likely we end up with some type explosion in normalization.
  let mut frb = FindRefinementBlockers::new();
  frb.traverse_type_id(target_ty);
  if !frb.found.empty() {
    let blocked: Vec<TypeId> = frb.found.iter().copied().collect();
    return TypeFunctionReductionResult::no_reduction(blocked);
  }

  let mut step_refine_count: i32 = 0;

  // Refine a target type and a discriminant one at a time.
  // Returns (result, toBlockOn). result is null (ptr::null) on definite failure.
  let step_refine = |ctx: &mut TypeFunctionContext,
                     step_refine_count: &mut i32,
                     target: TypeId,
                     discriminant: TypeId|
   -> StepResult {
    let _rl = RecursionLimiter::new(
      "BuiltInTypeFunctions::stepRefine",
      step_refine_count,
      dfint::LuauStepRefineRecursionLimit.get(),
    );

    // we need a more complex check for blocking on the discriminant in particular
    let mut frb = FindRefinementBlockers::new();
    frb.traverse_type_id(discriminant);

    if !frb.found.empty() {
      return (null(), frb.found.iter().copied().collect());
    }

    if let Some(ty) = intersect_with_simple_discriminant(
      Handle::from_ref(ctx.builtins()),
      Handle::from_nonnull(ctx.arena),
      target,
      discriminant,
    ) {
      return (ty, vec![]);
    }

    // NOTE: This block causes us to refine too early in some cases.
    if let Some(negation) = get_type::get::<NegationType>(discriminant) {
      let inner = follow_type::follow(negation.ty);
      if let Some(primitive) = get_type::get::<PrimitiveType>(inner)
        && primitive.r#type == PrimitiveType::NIL_TYPE
      {
        let result = simplify_intersection(
          Handle::from_ref(ctx.builtins()),
          Handle::from_nonnull(ctx.arena),
          target,
          discriminant,
        );
        return (result.result, vec![]);
      }
    }

    // If the target type is a table, then simplification already implements
    // the logic to deal with refinements properly. We also fire for simple
    // discriminants such as false? and ~(false?): the falsy and truthy types.
    if get_type::get::<TableType>(target).is_some() || is_truthy_or_falsy_type(discriminant) {
      let result = simplify_intersection(
        Handle::from_ref(ctx.builtins()),
        Handle::from_nonnull(ctx.arena),
        target,
        discriminant,
      );
      // Simplification considers free and generic types to be 'blocking',
      // but that's not suitable for refine<>. If we are only blocked on
      // those, we consider the simplification a success and reduce.
      let all_free_or_generic = result.blocked_types.iter().all(|&v| {
        let followed = follow_type::follow(v);
        get_type::get::<FreeType>(followed).is_some()
          || get_type::get::<GenericType>(followed).is_some()
      });
      if all_free_or_generic {
        return (result.result, vec![]);
      } else {
        return (null(), result.blocked_types.iter().copied().collect());
      }
    }

    // In the general case, we'll still use normalization though.
    let intersection = ctx.arena_mut().add_type(IntersectionType {
      parts: vec![target, discriminant],
    });
    let norm_intersection = ctx.normalizer_mut().try_normalize(intersection);
    let norm_type = ctx.normalizer_mut().try_normalize(target);

    let (Some(norm_intersection), Some(norm_type)) = (norm_intersection, norm_type) else {
      return (null_mut(), vec![]);
    };

    let mut result_ty = ctx.normalizer_mut().type_from_normal(&norm_intersection);
    // include the error type if the target type is error-suppressing and the
    // intersection we computed is not
    if norm_type.should_suppress_errors() && !norm_intersection.should_suppress_errors() {
      result_ty = add_union(
        Handle::from_nonnull(ctx.arena),
        Handle::from_ref(ctx.builtins()),
        &[result_ty, ctx.builtins().error_type],
      );
    }

    (result_ty, vec![])
  };

  // refine target with each discriminant type in sequence (reverse of insertion
  // order). If we cannot proceed, block. If all refine successfully, return.
  let mut target = target_ty;
  while !discriminant_types.is_empty() {
    // 循环头 `!is_empty()` 蕴含 last() 命中 Some。
    let mut discriminant = *discriminant_types
      .last()
      .expect("while 头 !discriminant_types.is_empty() 蕴含非空");

    discriminant = follow_type::follow(discriminant);

    // first, we'll see if simplifying the discriminant alone will solve our problem...
    if let Some(ut) = get_type::get::<UnionType>(discriminant) {
      let mut working_type = ctx_ref.builtins().never_type;

      for &option_as_discriminant in &ut.options {
        let simplified = simplify_union(
          Handle::from_ref(ctx_ref.builtins()),
          Handle::from_nonnull(ctx_ref.arena),
          working_type,
          option_as_discriminant,
        );

        if !simplified.blocked_types.empty() {
          return TypeFunctionReductionResult::no_reduction(
            simplified.blocked_types.iter().copied().collect(),
          );
        }

        working_type = simplified.result;
      }

      discriminant = working_type;
    }

    // if not, we try distributivity: a & (b | c) <=> (a & b) | (a & c)
    if let Some(ut) = get_type::get::<UnionType>(discriminant) {
      let mut final_refined = ctx_ref.builtins().never_type;

      for &option_as_discriminant in &ut.options {
        let (refined, blocked) = step_refine(
          ctx_ref,
          &mut step_refine_count,
          target,
          follow_type::follow(option_as_discriminant),
        );

        if blocked.is_empty() && refined.is_null() {
          return TypeFunctionReductionResult::no_reduction(Vec::new());
        }

        if !blocked.is_empty() {
          return TypeFunctionReductionResult::no_reduction(blocked);
        }

        let simplified = simplify_union(
          Handle::from_ref(ctx_ref.builtins()),
          Handle::from_nonnull(ctx_ref.arena),
          final_refined,
          refined,
        );

        if !simplified.blocked_types.empty() {
          return TypeFunctionReductionResult::no_reduction(
            simplified.blocked_types.iter().copied().collect(),
          );
        }

        final_refined = simplified.result;
      }

      target = final_refined;
      discriminant_types.pop();

      continue;
    }

    let (refined, blocked) = step_refine(ctx_ref, &mut step_refine_count, target, discriminant);

    if blocked.is_empty() && refined.is_null() {
      return TypeFunctionReductionResult::no_reduction(Vec::new());
    }

    if !blocked.is_empty() {
      return TypeFunctionReductionResult::no_reduction(blocked);
    }

    target = refined;
    discriminant_types.pop();
  }

  TypeFunctionReductionResult::reduction(target)
}
