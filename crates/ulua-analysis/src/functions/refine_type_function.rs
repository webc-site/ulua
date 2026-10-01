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

use ulua_common::{dfint, records::dense_hash_set::DenseHashSet};

use crate::{
  functions::{
    add_union::add_union, follow_type, get_type,
    intersect_with_simple_discriminant::intersect_with_simple_discriminant,
    is_blocked_or_unsolved_type::is_blocked_or_unsolved_type, is_pending::is_pending,
    is_truthy_or_falsy_type::is_truthy_or_falsy_type, occurs_builtin_type_functions::occurs,
    simplify_intersection_simplify::simplify_intersection, simplify_union::simplify_union,
  },
  macros::check_arity,
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
    visit_key::VisitKey,
  },
  type_aliases::{type_id::TypeId, type_pack_id::TypePackId},
};
impl GenericTypeVisitorTrait for ContainsRefinableType {
  type Seen = DenseHashSet<VisitKey>;

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
  type Seen = DenseHashSet<VisitKey>;

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

/// C++ `refineTypeFunction(...)` 直译。入参均为句柄值/切片/独占借用，无裸指针
/// 参数：`instance`/`type_params`/`pack_params` 只做 follow、比较与集合传递，
/// 实参形状由 `check_arity!` 守卫；会话对象经 `TypeFunctionContext` 安全访问器
/// 取用，仅仍带 `unsafe fn` 契约的 VM 桥对象（RefineTypeScrubber::new）调用留在
/// 收窄的 `unsafe {}` 内，签名无需 `unsafe`。
pub fn refine_type_function(
  instance: TypeId,
  type_params: &[TypeId],
  pack_params: &[TypePackId],
  ctx: &mut TypeFunctionContext,
) -> TypeFunctionReductionResult {
  check_arity!(ctx, guard type_params.len() < 2 || !pack_params.is_empty(), "refine");

  let mut target_ty = follow_type::follow(type_params[0]);

  // If we end up minting a refine type like `t1 where t1 = refine<T | t1, Y>`
  // this can create a degenerate set type `t1 where t1 = (T | t1) & Y`. Instead,
  // we clip the recursive part: `refine<T | t1, Y> => refine<T, Y>`.
  if occurs(target_ty, instance) {
    // Safety: RefineTypeScrubber::new 仍保留 `unsafe fn` 契约（methods/ 的替换器以
    // Handle 存柄并接管 arena 写入）；`ctx` 为本次归约独占借用、`instance` 为
    // ctx.arena 内实例节点的存活句柄，scrubber 不持借用跨调用。
    // 句柄先物化为局部值再转发（occurs 判定已确认其为存活 arena 节点）。
    let needle = instance;
    let mut rts = unsafe { RefineTypeScrubber::new(ctx, needle) };
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
      // is_pending 为安全函数：solver 句柄判空与只读探查收口其体内。
      if is_pending(t, ctx.solver) {
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
      let mut working_type = ctx.builtins().never_type;

      for &option_as_discriminant in &ut.options {
        let simplified = simplify_union(
          Handle::from_ref(ctx.builtins()),
          Handle::from_nonnull(ctx.arena),
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
      let mut final_refined = ctx.builtins().never_type;

      for &option_as_discriminant in &ut.options {
        let (refined, blocked) = step_refine(
          ctx,
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
          Handle::from_ref(ctx.builtins()),
          Handle::from_nonnull(ctx.arena),
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

    let (refined, blocked) = step_refine(ctx, &mut step_refine_count, target, discriminant);

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
