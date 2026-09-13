//! Source: `Analysis/src/Unifier2.cpp:316-405` — `Unifier2::unifyFreeWithType`.
//!
//! If super_ty is a function and sub_ty already has a potentially-compatible
//! function in its upper bound, we assume that the function is not overloaded
//! and attempt to combine super_ty into sub_ty's existing function bound.

use alloc::vec::Vec;

use ulua_common::{FFlag, macros::luau_assert::LUAU_ASSERT};

use crate::{
  enums::unify_result::UnifyResult,
  functions::{
    flatten_type_pack::flatten_type_pack_id, follow_type::follow_type_id,
    get_mutable_type::get_mutable_type_id, get_type_alt_j::get_type_id,
  },
  records::{
    free_type::FreeType, function_type::FunctionType, intersection_type::IntersectionType,
    unifier_2::Unifier2, union_type::UnionType,
  },
  type_aliases::type_id::TypeId,
};
impl Unifier2 {
  pub fn unify_free_with_type(&mut self, sub_ty: TypeId, super_ty: TypeId) -> UnifyResult {
    // C++ Unifier2.cpp:321 LUAU_ASSERT(subFree)：入口处断言 subTy 为 FreeType
    let sub_free = get_mutable_type_id::<FreeType>(sub_ty);
    LUAU_ASSERT!(sub_free.is_some());
    let upper_bound = follow_type_id(sub_free.unwrap().upper_bound);

    if get_type_id::<FunctionType>(upper_bound).is_some() {
      // C++ 用未 follow 的 upperBound
      let existing_upper = get_mutable_type_id::<FreeType>(sub_ty).unwrap().upper_bound;
      return self.unify_type_id_type_id(existing_upper, super_ty);
    }

    // When super_ty is a union or intersection, propagate sub_ty as a lower bound into any
    // free-type members. Without this, `freeA <: 'T | nil` (or `freeA <: 'T & C`) never
    // constrains 'T, because the FreeType path intercepts before structural dispatch.
    // Members may be GenericTypes that map to FreeTypes via genericSubstitutions.
    if FFlag::LuauPropagateFreeTypesIntoUnionAndIntersectionBounds.get() {
      if let Some(super_union) = get_type_id::<UnionType>(super_ty) {
        let members: Vec<TypeId> = super_union.options.clone();
        self.propagate_to_free_members(&members, sub_ty);
        return self.do_default_unify_free(sub_ty, super_ty);
      }

      if let Some(super_intersection) = get_type_id::<IntersectionType>(super_ty) {
        let members: Vec<TypeId> = super_intersection.parts.clone();
        self.propagate_to_free_members(&members, sub_ty);
        return self.do_default_unify_free(sub_ty, super_ty);
      }
    }

    let Some(super_function) = get_type_id::<FunctionType>(super_ty) else {
      return self.do_default_unify_free(sub_ty, super_ty);
    };

    let (super_arg_head, super_arg_tail) = flatten_type_pack_id(super_function.arg_types);
    if super_arg_tail.is_some() {
      return self.do_default_unify_free(sub_ty, super_ty);
    }

    let Some(upper_bound_intersection) = get_type_id::<IntersectionType>(upper_bound) else {
      return self.do_default_unify_free(sub_ty, super_ty);
    };

    let mut result = UnifyResult::Ok;
    let mut found_one = false;

    for part in &upper_bound_intersection.parts {
      let Some(ft) = get_type_id::<FunctionType>(follow_type_id(*part)) else {
        continue;
      };

      let (sub_arg_head, sub_arg_tail) = flatten_type_pack_id(ft.arg_types);

      if sub_arg_tail.is_none() && sub_arg_head.len() == super_arg_head.len() {
        found_one = true;
        result &= self.unify_type_id_type_id(*part, super_ty);
      }
    }

    if found_one {
      result
    } else {
      self.do_default_unify_free(sub_ty, super_ty)
    }
  }

  /// C++ `doDefault` lambda from `unifyFreeWithType`.
  fn do_default_unify_free(&mut self, sub_ty: TypeId, super_ty: TypeId) -> UnifyResult {
    // SAFETY: 调用方（unify_free_with_type）已断言 sub_ty 为 FreeType
    let sub_free = get_mutable_type_id::<FreeType>(sub_ty).unwrap();
    let new_super_ty = self.instantiate_with_bound_types(super_ty);
    let new_upper = self.mk_intersection(sub_free.upper_bound, new_super_ty);
    sub_free.upper_bound = new_upper;
    self
      .expanded_free_types
      .get_or_insert(sub_ty)
      .push(new_super_ty);
    UnifyResult::Ok
  }

  /// C++ `propagateToFreeMembers` lambda from `unifyFreeWithType`.
  fn propagate_to_free_members(&mut self, member_range: &[TypeId], sub_ty: TypeId) {
    for &member in member_range {
      let mut m = follow_type_id(member);
      if let Some(subst) = self.generic_substitutions.find(&m) {
        m = follow_type_id(*subst);
      }
      if let Some(member_free) = get_mutable_type_id::<FreeType>(m) {
        let instantiated = self.instantiate_with_bound_types(sub_ty);
        let new_lower = self.mk_union(member_free.lower_bound, instantiated);
        member_free.lower_bound = new_lower;
      }
    }
  }
}
