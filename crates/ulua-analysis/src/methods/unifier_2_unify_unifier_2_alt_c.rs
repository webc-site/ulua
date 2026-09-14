//! Source: `Analysis/src/Unifier2.cpp:149-302` — `Unifier2::unify_(TypeId, TypeId)`,
//! the core of new-solver type unification.

use core::{ffi::c_void, mem::zeroed};

use ulua_common::{FFlag, FInt};

use crate::{
  enums::unify_result::UnifyResult,
  functions::{
    follow_type::follow_type_id, get_mutable_type::get_mutable_type_id,
    get_type_alt_j::get_type_id, is_irresolvable_unifier_2::is_irresolvable,
  },
  records::{
    any_type::AnyType, free_type::FreeType, function_type::FunctionType,
    intersection_type::IntersectionType, metatable_type::MetatableType,
    negation_type::NegationType, never_type::NeverType,
    non_exceptional_recursion_limiter::NonExceptionalRecursionLimiter,
    subtype_constraint::SubtypeConstraint, table_type::TableType, unifier_2::Unifier2,
    union_type::UnionType, unknown_type::UnknownType,
  },
  type_aliases::{constraint_v::ConstraintV, type_id::TypeId},
};
impl Unifier2 {
  /// # Safety
  /// 调用方须保证 `参数` 等裸指针参数有效，且满足 C++ 原实现的调用契约。
  pub(crate) fn unify_type_id_type_id(
    &mut self,
    mut sub_ty: TypeId,
    mut super_ty: TypeId,
  ) -> UnifyResult {
    if FInt::LuauTypeInferIterationLimit.get() > 0
      && self.iteration_count >= FInt::LuauTypeInferIterationLimit.get()
    {
      return UnifyResult::TooComplex;
    }

    self.iteration_count += 1;

    // NOTE: It's a little odd that we are doing something non-exceptional for
    // the core of unification but not for occurs check, which may throw an
    // exception. It would be nice if, in the future, this were unified.
    if FFlag::LuauLimitUnificationRecursion.get() {
      // ++(*count) — mirror the C++ NonExceptionalRecursionLimiter (RecursionCounter ctor).
      self.recursion_count += 1;
      let mut nerl = NonExceptionalRecursionLimiter {
        base: unsafe { zeroed() },
        native_stack_guard: unsafe { zeroed() },
      };
      nerl.non_exceptional_recursion_limiter_non_exceptional_recursion_limiter(
        &mut self.recursion_count,
      );
      if !nerl.is_ok(self.recursion_limit) {
        return UnifyResult::TooComplex;
      }
    }

    sub_ty = follow_type_id(sub_ty);
    super_ty = follow_type_id(super_ty);

    if let Some(sub_gen) = self.generic_substitutions.find(&sub_ty) {
      let sub_gen = *sub_gen;
      return self.unify_type_id_type_id(sub_gen, super_ty);
    }

    if let Some(super_gen) = self.generic_substitutions.find(&super_ty) {
      let super_gen = *super_gen;
      return self.unify_type_id_type_id(sub_ty, super_gen);
    }

    if self.seen_type_pairings.contains(&(sub_ty, super_ty)) {
      return UnifyResult::Ok;
    }
    self.seen_type_pairings.insert((sub_ty, super_ty));

    if sub_ty == super_ty {
      return UnifyResult::Ok;
    }

    // We have potentially done some unifications while dispatching either `SubtypeConstraint` or `PackSubtypeConstraint`,
    // so rather than implementing backtracking or traversing the entire type graph multiple times, we could push
    // additional constraints as we discover blocked types along with their proper bounds.
    //
    // But we exclude these two subtyping patterns, they are tautological:
    //   - never <: *blocked*
    //   - *blocked* <: unknown
    if (is_irresolvable(sub_ty) || is_irresolvable(super_ty))
      && get_type_id::<NeverType>(sub_ty).is_none()
      && get_type_id::<UnknownType>(super_ty).is_none()
    {
      if !self.uninhabited_type_functions.is_null()
        && unsafe {
          (*self.uninhabited_type_functions).contains(&(sub_ty as *const c_void))
            || (*self.uninhabited_type_functions).contains(&(super_ty as *const c_void))
        }
      {
        return UnifyResult::Ok;
      }

      self
        .incomplete_subtypes
        .push(ConstraintV::Subtype(SubtypeConstraint {
          sub_type: sub_ty,
          super_type: super_ty,
        }));
      return UnifyResult::Ok;
    }

    // C++ Unifier2.cpp:206-218：superFree 先合并 lowerBound，subFree 走 unifyFreeWithType
    if let Some(super_free) = get_mutable_type_id::<FreeType>(super_ty) {
      let instantiated = self.instantiate_with_bound_types(sub_ty);
      let new_lower = self.mk_union(super_free.lower_bound, instantiated);
      super_free.lower_bound = new_lower;
    }

    let sub_free = get_mutable_type_id::<FreeType>(sub_ty);
    if sub_free.is_some() {
      return self.unify_free_with_type(sub_ty, super_ty);
    }

    // subFree 已排除（否则上面已 return），仅剩 superFree 情形
    if sub_free.is_some() || get_mutable_type_id::<FreeType>(super_ty).is_some() {
      return UnifyResult::Ok;
    }

    let sub_fn = get_type_id::<FunctionType>(sub_ty);
    let super_fn = get_type_id::<FunctionType>(super_ty);
    if let (Some(_), Some(super_fn_ref)) = (sub_fn, super_fn) {
      return self.unify_type_id_function_type(sub_ty, super_fn_ref);
    }

    let sub_union = get_type_id::<UnionType>(sub_ty);
    let super_union = get_type_id::<UnionType>(super_ty);
    if let Some(sub_union) = sub_union {
      return unsafe { self.unify_union_type_type_id(sub_union, super_ty) };
    } else if let Some(super_union) = super_union {
      return self.unify_type_id_union_type(sub_ty, super_union);
    }

    let sub_intersection = get_type_id::<IntersectionType>(sub_ty);
    let super_intersection = get_type_id::<IntersectionType>(super_ty);
    if let Some(sub_intersection) = sub_intersection {
      return self.unify_intersection_type_type_id(sub_intersection, super_ty);
    } else if let Some(super_intersection) = super_intersection {
      return self.unify_type_id_intersection_type(sub_ty, super_intersection);
    }

    let sub_never = get_type_id::<NeverType>(sub_ty);
    let super_never = get_type_id::<NeverType>(super_ty);
    if let (Some(_), Some(_)) = (sub_never, super_never) {
      return UnifyResult::Ok;
    } else if let (Some(_), Some(super_fn_ref)) = (sub_never, super_fn) {
      // If `never` is the subtype, then we can propagate that inward.
      let builtin_types = unsafe { &*self.builtin_types.as_ptr() };
      let never_pack = builtin_types.never_type_pack;
      let arg_result = self.unify_type_pack_id_type_pack_id(super_fn_ref.arg_types, never_pack);
      let ret_result = self.unify_type_pack_id_type_pack_id(never_pack, super_fn_ref.ret_types);
      return arg_result & ret_result;
    } else if let (Some(sub_fn_ref), Some(_)) = (sub_fn, super_never) {
      // If `never` is the supertype, then we can propagate that inward.
      let builtin_types = unsafe { &*self.builtin_types.as_ptr() };
      let never_pack = builtin_types.never_type_pack;
      let arg_result = self.unify_type_pack_id_type_pack_id(never_pack, sub_fn_ref.arg_types);
      let ret_result = self.unify_type_pack_id_type_pack_id(sub_fn_ref.ret_types, never_pack);
      return arg_result & ret_result;
    }

    let sub_any = get_type_id::<AnyType>(sub_ty);
    let super_any = get_type_id::<AnyType>(super_ty);

    let mut sub_table = get_mutable_type_id::<TableType>(sub_ty);
    let super_table = get_type_id::<TableType>(super_ty);

    if let (Some(_), Some(_)) = (sub_any, super_any) {
      return UnifyResult::Ok;
    } else if let (Some(sub_any), Some(super_fn_ref)) = (sub_any, super_fn) {
      return self.unify_any_type_function_type(sub_any, super_fn_ref);
    } else if let (Some(sub_fn_ref), Some(super_any)) = (sub_fn, super_any) {
      return self.unify_function_type_any_type(sub_fn_ref, super_any);
    } else if let (Some(sub_any), Some(super_table_ref)) = (sub_any, super_table) {
      return self.unify_any_type_table_type(sub_any, super_table_ref);
    } else if let (Some(sub_table_ref), Some(super_any)) = (sub_table.as_deref_mut(), super_any) {
      return self.unify_table_type_any_type(sub_table_ref, super_any);
    }

    if let (Some(sub_table), Some(super_table_ref)) = (sub_table, super_table) {
      // `bound_to` works like a bound type, and therefore we'd replace it
      // with the `bound_to` and try unification again.
      //
      // However, these pointers should have been chased already by follow().
      ulua_common::macros::luau_assert::LUAU_ASSERT!(sub_table.bound_to.is_none());
      ulua_common::macros::luau_assert::LUAU_ASSERT!(super_table_ref.bound_to.is_none());

      return self.unify_table_type_table_type(sub_table, super_table_ref);
    }

    let sub_metatable = get_type_id::<MetatableType>(sub_ty);
    let super_metatable = get_type_id::<MetatableType>(super_ty);
    if let (Some(sub_mt), Some(super_mt)) = (sub_metatable, super_metatable) {
      return self.unify_metatable_type_metatable_type(sub_mt, super_mt);
    } else if let (Some(sub_mt), Some(super_any_ref)) = (sub_metatable, super_any) {
      return self.unify_metatable_type_any_type(sub_mt, super_any_ref);
    } else if let (Some(sub_any_ref), Some(super_mt)) = (sub_any, super_metatable) {
      return self.unify_any_type_metatable_type(sub_any_ref, super_mt);
    } else if let Some(sub_mt) = sub_metatable {
      // if we only have one metatable, unify with the inner table
      let inner = sub_mt.table();
      return self.unify_type_id_type_id(inner, super_ty);
    } else if let Some(super_mt) = super_metatable {
      // if we only have one metatable, unify with the inner table
      let inner = super_mt.table();
      return self.unify_type_id_type_id(sub_ty, inner);
    }

    let sub_negation = get_type_id::<NegationType>(sub_ty);
    let super_negation = get_type_id::<NegationType>(super_ty);
    if let (Some(sub_neg), Some(super_neg)) = (sub_negation, super_negation) {
      return self.unify_type_id_type_id(sub_neg.ty, super_neg.ty);
    }

    // The unification failed, but we're not doing type checking.
    UnifyResult::Ok
  }
}
