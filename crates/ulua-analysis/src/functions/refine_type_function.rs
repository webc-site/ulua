//! C++ `TypeFunctionReductionResult<TypeId> refineTypeFunction(TypeId instance,
//! const std::vector<TypeId>& typeParams, const std::vector<TypePackId>&
//! packParams, NotNull<TypeFunctionContext> ctx)`
//! (BuiltinTypeFunctions.cpp:1207-1432). The `refine` reducer.
use alloc::{vec, vec::Vec};
// Wire the two refinement visitors into `GenericTypeVisitorTrait` so the base
// `traverse_type_id` dispatches through their `visit_*` overrides. The inherent
// methods on `ContainsRefinableType` / `FindRefinementBlockers` carry the real
// logic; these impls just forward to them.
use core::{
  ffi::c_void,
  mem::zeroed,
  ptr::{NonNull, null, null_mut},
};

use ulua_common::{DFInt, macros::luau_assert::LUAU_ASSERT, records::dense_hash_set::DenseHashSet};

use crate::{
  enums::reduction::Reduction,
  functions::{
    add_union::add_union, follow_type::follow_type_id, get_type_alt_j::get_type_id,
    intersect_with_simple_discriminant::intersect_with_simple_discriminant,
    is_blocked_or_unsolved_type::is_blocked_or_unsolved_type, is_pending::is_pending,
    is_truthy_or_falsy_type::is_truthy_or_falsy_type, occurs_builtin_type_functions_alt_b::occurs,
    simplify_intersection_simplify::simplify_intersection, simplify_union::simplify_union,
  },
  records::{
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
  type Seen = DenseHashSet<*mut c_void>;

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
  type Seen = DenseHashSet<*mut c_void>;

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
/// 调用方须保证满足 C++ 原实现的调用契约。
pub unsafe fn refine_type_function(
  instance: TypeId,
  type_params: Vec<TypeId>,
  pack_params: Vec<TypePackId>,
  ctx: *mut TypeFunctionContext,
) -> TypeFunctionReductionResult {
  // SAFETY: reducer 由 ReducerFunction 裸指针调用，契约保证 ctx 非空且会话期有效。
  // SAFETY: 由 ReducerFunction 裸指针调用，契约保证 ctx 非空且会话期有效；
  // NonNull 封装解引用。
  let ctx_ref = unsafe { NonNull::new_unchecked(ctx).as_ref() };

  if type_params.len() < 2 || !pack_params.is_empty() {
    unsafe { ctx_ref.ice.as_ref() }.ice_string("refine type function: encountered a type function instance without the required argument structure");
    LUAU_ASSERT!(false);
  }

  let mut target_ty = follow_type_id(type_params[0]);

  // If we end up minting a refine type like `t1 where t1 = refine<T | t1, Y>`
  // this can create a degenerate set type `t1 where t1 = (T | t1) & Y`. Instead,
  // we clip the recursive part: `refine<T | t1, Y> => refine<T, Y>`.
  if occurs(target_ty, instance) {
    let mut rts = unsafe { RefineTypeScrubber::new(ctx, instance) };
    if let Some(result) = rts.substitute_type_id(target_ty) {
      target_ty = result;
    }
  }

  let mut discriminant_types: Vec<TypeId> = Vec::new();
  for &param in type_params.iter().skip(1) {
    let discriminant = follow_type_id(param);

    // Filter out any top level types that are meaningless to refine against.
    let is_unknown = get_type_id::<UnknownType>(discriminant).is_some();
    let is_no_refine = get_type_id::<NoRefineType>(discriminant).is_some();
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
    return TypeFunctionReductionResult {
      result: Some(target_ty),
      reduction_status: Reduction::MaybeOk,
      blocked_types: vec![],
      blocked_packs: vec![],
      error: None,
      messages: vec![],
    };
  }

  let target_is_pending = is_blocked_or_unsolved_type(target_ty);

  // check to see if both operand types are resolved enough, and wait to reduce if not
  if target_is_pending {
    return TypeFunctionReductionResult {
      result: None,
      reduction_status: Reduction::MaybeOk,
      blocked_types: vec![target_ty],
      blocked_packs: vec![],
      error: None,
      messages: vec![],
    };
  } else {
    for &t in &discriminant_types {
      if unsafe { is_pending(t, ctx_ref.solver) } {
        return TypeFunctionReductionResult {
          result: None,
          reduction_status: Reduction::MaybeOk,
          blocked_types: vec![t],
          blocked_packs: vec![],
          error: None,
          messages: vec![],
        };
      }
    }
  }

  // If we have a blocked type in the target, we *could* potentially refine it,
  // but more likely we end up with some type explosion in normalization.
  let mut frb = FindRefinementBlockers::new();
  frb.traverse_type_id(target_ty);
  if !frb.found.empty() {
    let blocked: Vec<TypeId> = frb.found.iter().copied().collect();
    return TypeFunctionReductionResult {
      result: None,
      reduction_status: Reduction::MaybeOk,
      blocked_types: blocked,
      blocked_packs: vec![],
      error: None,
      messages: vec![],
    };
  }

  let mut step_refine_count: i32 = 0;

  // Refine a target type and a discriminant one at a time.
  // Returns (result, toBlockOn). result is null (ptr::null) on definite failure.
  let step_refine =
    |step_refine_count: &mut i32, target: TypeId, discriminant: TypeId| -> StepResult {
      let mut _rl = RecursionLimiter {
        base: unsafe { zeroed() },
        native_stack_guard: unsafe { zeroed() },
      };
      _rl.recursion_limiter_recursion_limiter(
        "BuiltInTypeFunctions::stepRefine",
        step_refine_count as *mut i32,
        DFInt::LuauStepRefineRecursionLimit.get(),
      );

      // we need a more complex check for blocking on the discriminant in particular
      let mut frb = FindRefinementBlockers::new();
      frb.traverse_type_id(discriminant);

      if !frb.found.empty() {
        return (null(), frb.found.iter().copied().collect());
      }

      if let Some(ty) = intersect_with_simple_discriminant(
        ctx_ref.builtins.as_ptr(),
        ctx_ref.arena.as_ptr(),
        target,
        discriminant,
      ) {
        return (ty, vec![]);
      }

      // NOTE: This block causes us to refine too early in some cases.
      if let Some(negation) = get_type_id::<NegationType>(discriminant) {
        let inner = follow_type_id(negation.ty);
        if let Some(primitive) = get_type_id::<PrimitiveType>(inner)
          && primitive.r#type == PrimitiveType::NIL_TYPE
        {
          let result = simplify_intersection(
            ctx_ref.builtins.as_ptr(),
            ctx_ref.arena.as_ptr(),
            target,
            discriminant,
          );
          return (result.result, vec![]);
        }
      }

      // If the target type is a table, then simplification already implements
      // the logic to deal with refinements properly. We also fire for simple
      // discriminants such as false? and ~(false?): the falsy and truthy types.
      if get_type_id::<TableType>(target).is_some()
        || unsafe { is_truthy_or_falsy_type(discriminant) }
      {
        let result = simplify_intersection(
          ctx_ref.builtins.as_ptr(),
          ctx_ref.arena.as_ptr(),
          target,
          discriminant,
        );
        // Simplification considers free and generic types to be 'blocking',
        // but that's not suitable for refine<>. If we are only blocked on
        // those, we consider the simplification a success and reduce.
        let all_free_or_generic = result.blocked_types.iter().all(|&v| {
          let followed = follow_type_id(v);
          get_type_id::<FreeType>(followed).is_some()
            || get_type_id::<GenericType>(followed).is_some()
        });
        if all_free_or_generic {
          return (result.result, vec![]);
        } else {
          return (null(), result.blocked_types.iter().copied().collect());
        }
      }

      // In the general case, we'll still use normalization though.
      let intersection = unsafe { &mut *ctx_ref.arena.as_ptr() }.add_type(IntersectionType {
        parts: vec![target, discriminant],
      });
      let norm_intersection =
        unsafe { &mut *ctx_ref.normalizer.as_ptr() }.try_normalize(intersection);
      let norm_type = unsafe { &mut *ctx_ref.normalizer.as_ptr() }.try_normalize(target);

      let (Some(norm_intersection), Some(norm_type)) = (norm_intersection, norm_type) else {
        return (null_mut(), vec![]);
      };

      let mut result_ty =
        unsafe { &mut *ctx_ref.normalizer.as_ptr() }.type_from_normal(&norm_intersection);
      // include the error type if the target type is error-suppressing and the
      // intersection we computed is not
      if norm_type.should_suppress_errors() && !norm_intersection.should_suppress_errors() {
        result_ty = add_union(
          ctx_ref.arena.as_ptr(),
          ctx_ref.builtins.as_ptr(),
          &[result_ty, unsafe { ctx_ref.builtins.as_ref() }.error_type],
        );
      }

      (result_ty, vec![])
    };

  // refine target with each discriminant type in sequence (reverse of insertion
  // order). If we cannot proceed, block. If all refine successfully, return.
  let mut target = target_ty;
  while !discriminant_types.is_empty() {
    let mut discriminant = *discriminant_types.last().unwrap();

    discriminant = follow_type_id(discriminant);

    // first, we'll see if simplifying the discriminant alone will solve our problem...
    if let Some(ut) = get_type_id::<UnionType>(discriminant) {
      let mut working_type = unsafe { ctx_ref.builtins.as_ref() }.never_type;

      for &option_as_discriminant in &ut.options {
        let simplified = simplify_union(
          ctx_ref.builtins.as_ptr(),
          ctx_ref.arena.as_ptr(),
          working_type,
          option_as_discriminant,
        );

        if !simplified.blocked_types.empty() {
          return TypeFunctionReductionResult {
            result: None,
            reduction_status: Reduction::MaybeOk,
            blocked_types: simplified.blocked_types.iter().copied().collect(),
            blocked_packs: vec![],
            error: None,
            messages: vec![],
          };
        }

        working_type = simplified.result;
      }

      discriminant = working_type;
    }

    // if not, we try distributivity: a & (b | c) <=> (a & b) | (a & c)
    if let Some(ut) = get_type_id::<UnionType>(discriminant) {
      let mut final_refined = unsafe { ctx_ref.builtins.as_ref() }.never_type;

      for &option_as_discriminant in &ut.options {
        let (refined, blocked) = step_refine(
          &mut step_refine_count,
          target,
          follow_type_id(option_as_discriminant),
        );

        if blocked.is_empty() && refined.is_null() {
          return TypeFunctionReductionResult {
            result: None,
            reduction_status: Reduction::MaybeOk,
            blocked_types: vec![],
            blocked_packs: vec![],
            error: None,
            messages: vec![],
          };
        }

        if !blocked.is_empty() {
          return TypeFunctionReductionResult {
            result: None,
            reduction_status: Reduction::MaybeOk,
            blocked_types: blocked,
            blocked_packs: vec![],
            error: None,
            messages: vec![],
          };
        }

        let simplified = simplify_union(
          ctx_ref.builtins.as_ptr(),
          ctx_ref.arena.as_ptr(),
          final_refined,
          refined,
        );

        if !simplified.blocked_types.empty() {
          return TypeFunctionReductionResult {
            result: None,
            reduction_status: Reduction::MaybeOk,
            blocked_types: simplified.blocked_types.iter().copied().collect(),
            blocked_packs: vec![],
            error: None,
            messages: vec![],
          };
        }

        final_refined = simplified.result;
      }

      target = final_refined;
      discriminant_types.pop();

      continue;
    }

    let (refined, blocked) = step_refine(&mut step_refine_count, target, discriminant);

    if blocked.is_empty() && refined.is_null() {
      return TypeFunctionReductionResult {
        result: None,
        reduction_status: Reduction::MaybeOk,
        blocked_types: vec![],
        blocked_packs: vec![],
        error: None,
        messages: vec![],
      };
    }

    if !blocked.is_empty() {
      return TypeFunctionReductionResult {
        result: None,
        reduction_status: Reduction::MaybeOk,
        blocked_types: blocked,
        blocked_packs: vec![],
        error: None,
        messages: vec![],
      };
    }

    target = refined;
    discriminant_types.pop();
  }

  TypeFunctionReductionResult {
    result: Some(target),
    reduction_status: Reduction::MaybeOk,
    blocked_types: vec![],
    blocked_packs: vec![],
    error: None,
    messages: vec![],
  }
}
