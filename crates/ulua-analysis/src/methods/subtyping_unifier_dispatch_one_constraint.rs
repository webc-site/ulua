//! Source: `Analysis/src/SubtypingUnifier.cpp:83-183` — `SubtypingUnifier::dispatchOneConstraint`.

use ulua_common::{FFlag, macros::luau_assert::LUAU_ASSERT};

use crate::{
  enums::{occurs_check_result::OccursCheckResult, unify_result::UnifyResult},
  functions::{
    as_mutable_type_pack::as_mutable_type_pack_id, emplace_type_pack::emplace_type_pack,
    follow_type::follow_type_id, follow_type_pack::follow_type_pack_id, get_2::get2,
    get_mutable_type::get_mutable_type_id, get_type_pack::get_type_pack_id,
    is_blocked_type_utils::is_blocked,
    occurs_check_type_utils_alt_b::occurs_check_type_pack_id_type_pack_id,
    simplify_intersection_simplify::simplify_intersection, simplify_union::simplify_union,
  },
  records::{
    constraint::Constraint, free_type::FreeType, free_type_pack::FreeTypePack,
    pack_subtype_constraint::PackSubtypeConstraint, subtype_constraint::SubtypeConstraint,
    subtyping_unifier::SubtypingUnifier, table_indexer::TableIndexer, table_type::TableType,
  },
  type_aliases::{
    constraint_v::{ConstraintV, ConstraintVMember},
    type_pack_variant::TypePackVariant,
    upper_bounds::UpperBounds,
  },
};

impl SubtypingUnifier {
  /// # Safety
  /// 调用方须保证满足 C++ 原实现的调用契约。
  pub unsafe fn dispatch_one_constraint(
    &self,
    constraint: *const Constraint,
    cv: &ConstraintV,
    upper_bound_contributors: &mut UpperBounds,
  ) -> (UnifyResult, bool) {
    let builtin_types = self.builtin_types;
    let arena = self.arena;

    if let Some(sc) = SubtypeConstraint::get_if(cv) {
      let sub_ty = follow_type_id(sc.sub_type);
      let super_ty = follow_type_id(sc.super_type);

      LUAU_ASSERT!(self.can_be_unified(sub_ty) || self.can_be_unified(super_ty));

      if is_blocked(sub_ty) || is_blocked(super_ty) {
        return (UnifyResult::Ok, false);
      }

      if let Some(super_free_ty) = get_mutable_type_id::<FreeType>(super_ty) {
        let lower = super_free_ty.lower_bound;
        let result = simplify_union(builtin_types, arena, sub_ty, lower).result;
        super_free_ty.lower_bound = result;
      }

      if let Some(sub_free_type) = get_mutable_type_id::<FreeType>(sub_ty) {
        let upper = sub_free_type.upper_bound;
        let result = simplify_intersection(builtin_types, arena, upper, super_ty).result;
        sub_free_type.upper_bound = result;
        let location = unsafe { (*constraint).location };
        upper_bound_contributors
          .get_or_insert(sub_ty)
          .push((location, super_ty));
      }

      // FIXME CLI-182960: Unification shouldn't be the mechanism for adding table indexers
      let pair = get2::<TableType, TableType, _>(sub_ty, super_ty);
      if !pair.first.is_null() && unsafe { (*pair.first).indexer }.is_none() {
        let super_indexer = unsafe { (*pair.second).indexer }.unwrap();
        if let Some(sub_table) = get_mutable_type_id::<TableType>(sub_ty) {
          sub_table.indexer = Some(TableIndexer {
            index_type: super_indexer.index_type,
            index_result_type: super_indexer.index_result_type,
            is_read_only: false,
          });
        }
      }
    } else if let Some(psc) = PackSubtypeConstraint::get_if(cv) {
      let sub_tp = unsafe { follow_type_pack_id(psc.sub_pack) };
      let super_tp = unsafe { follow_type_pack_id(psc.super_pack) };
      // There *should* be an assertion here that either part of the constraint is
      // free, but because free type packs are replaced on-the-spot, we *may*
      // encounter conflicting constraints. One example is for:
      //
      //  local callbacks: { () -> () } = {
      //      function () end,
      //      function () end
      //  }
      //
      // We'll end up minting two sets of constraints for each lambda (as we need
      // them to be _exactly_ `()` as per the table type).
      let error_pack = unsafe { (*builtin_types).error_type_pack };

      if get_type_pack_id::<FreeTypePack>(sub_tp).is_some() {
        if FFlag::LuauOccursCheckForAllBindings.get() {
          if OccursCheckResult::Fail == occurs_check_type_pack_id_type_pack_id(sub_tp, super_tp) {
            unsafe {
              emplace_type_pack(
                as_mutable_type_pack_id(sub_tp),
                TypePackVariant::Bound(error_pack),
              )
            };
            return (UnifyResult::OccursCheckFailed, true);
          }
        } else {
          if OccursCheckResult::Fail == self.occurs_check_deprecated(sub_tp, super_tp) {
            unsafe {
              emplace_type_pack(
                as_mutable_type_pack_id(sub_tp),
                TypePackVariant::Bound(error_pack),
              )
            };
            return (UnifyResult::OccursCheckFailed, true);
          }
        }
        unsafe {
          emplace_type_pack(
            as_mutable_type_pack_id(sub_tp),
            TypePackVariant::Bound(super_tp),
          )
        };
        return (UnifyResult::Ok, true);
      }

      if get_type_pack_id::<FreeTypePack>(super_tp).is_some() {
        if FFlag::LuauOccursCheckForAllBindings.get() {
          if OccursCheckResult::Fail == occurs_check_type_pack_id_type_pack_id(super_tp, sub_tp) {
            unsafe {
              emplace_type_pack(
                as_mutable_type_pack_id(super_tp),
                TypePackVariant::Bound(error_pack),
              )
            };
            return (UnifyResult::OccursCheckFailed, true);
          }
        } else {
          if OccursCheckResult::Fail == self.occurs_check_deprecated(super_tp, sub_tp) {
            unsafe {
              emplace_type_pack(
                as_mutable_type_pack_id(super_tp),
                TypePackVariant::Bound(error_pack),
              )
            };
            return (UnifyResult::OccursCheckFailed, true);
          }
        }

        unsafe {
          emplace_type_pack(
            as_mutable_type_pack_id(super_tp),
            TypePackVariant::Bound(sub_tp),
          )
        };
        return (UnifyResult::Ok, true);
      }
    } else {
      LUAU_ASSERT!(false); // Unreachable, unexpected constraint in subtyping unifier.
      return (UnifyResult::Ok, false);
    }

    (UnifyResult::Ok, true)
  }
}
