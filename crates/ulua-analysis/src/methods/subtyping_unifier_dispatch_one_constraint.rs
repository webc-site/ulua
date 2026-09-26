//! Source: `Analysis/src/SubtypingUnifier.cpp:83-183` — `SubtypingUnifier::dispatchOneConstraint`.

use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::{
  enums::{occurs_check_result::OccursCheckResult, unify_result::UnifyResult},
  functions::{
    as_mutable_type_pack::as_mutable_type_pack, emplace_type_pack::emplace_type_pack, follow_type,
    follow_type_pack, get_2::get2, get_mutable_type, get_type_pack,
    is_blocked_type_utils::is_blocked,
    occurs_check_type_utils::occurs_check_type_pack_id_type_pack_id,
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
      let sub_ty = follow_type::follow(sc.sub_type);
      let super_ty = follow_type::follow(sc.super_type);

      LUAU_ASSERT!(self.can_be_unified(sub_ty) || self.can_be_unified(super_ty));

      if is_blocked(sub_ty) || is_blocked(super_ty) {
        return (UnifyResult::Ok, false);
      }

      if let Some(super_free_ty) = get_mutable_type::get_mutable::<FreeType>(super_ty) {
        let lower = super_free_ty.lower_bound;
        let result = simplify_union(builtin_types, arena, sub_ty, lower).result;
        super_free_ty.lower_bound = result;
      }

      if let Some(sub_free_type) = get_mutable_type::get_mutable::<FreeType>(sub_ty) {
        let upper = sub_free_type.upper_bound;
        let result = simplify_intersection(builtin_types, arena, upper, super_ty).result;
        sub_free_type.upper_bound = result;
        // Safety: `constraint` 是本次正在消解的约束——由求解器 ConstraintArena
        // 分配、整次消解期间存活且地址稳定（C++ `constraint->location` 同源），
        // 此处仅只读出 `Copy` 的 location 存入 upper-bound 贡献表。
        let location = unsafe { (*constraint).location };
        upper_bound_contributors
          .get_or_insert(sub_ty)
          .push((location, super_ty));
      }

      // FIXME CLI-182960: Unification shouldn't be the mechanism for adding table indexers
      // C++ 直接 `pair.second->indexer->indexType`，两侧都无 indexer 时是 UB；
      // Rust 侧禁止 unwrap panic，空 optional 直接跳过。
      if let Some((sub_table_ref, super_table)) = get2::<TableType, TableType, _>(sub_ty, super_ty)
        && sub_table_ref.indexer.is_none()
        && let Some(super_indexer) = super_table.indexer.as_ref()
        && let Some(sub_table) = get_mutable_type::get_mutable::<TableType>(sub_ty)
      {
        sub_table.indexer = Some(TableIndexer {
          index_type: super_indexer.index_type,
          index_result_type: super_indexer.index_result_type,
          is_read_only: false,
        });
      }
    } else if let Some(psc) = PackSubtypeConstraint::get_if(cv) {
      let sub_tp = follow_type_pack::follow(psc.sub_pack);
      let super_tp = follow_type_pack::follow(psc.super_pack);
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
      // `builtin_types`（即 `self.builtin_types` 句柄）对应 C++
      // `NotNull<BuiltinTypes>`——构造期接线、非空且比本消解会话长寿，只读
      // `Copy` 的 error_type_pack 句柄。
      let error_pack = builtin_types.get().error_type_pack;

      // C++ 无条件调用 TypeUtils 自由函数 `occursCheck`；
      // 曾误加 LuauOccursCheckForAllBindings 门控（cpp 无此开关）走已废弃变体。
      if get_type_pack::get::<FreeTypePack>(sub_tp).is_some() {
        if OccursCheckResult::Fail == occurs_check_type_pack_id_type_pack_id(sub_tp, super_tp) {
          // Safety: 目标指针 `as_mutable_type_pack(sub_tp)` 由 follow 归一的
          // 非空 TypePackId 裸化——节点驻留类型 arena（bump arena 地址不移动、
          // 求解期存活），emplace 就地覆写其 Variant 即 C++
          // `emplaceTypePack(asMutable(subTp), BoundTypePack{errorPack})`，
          // 单线程下该短借用无并存的其它可变句柄。
          unsafe {
            emplace_type_pack(
              as_mutable_type_pack(sub_tp),
              TypePackVariant::Bound(error_pack),
            )
          };
          return (UnifyResult::OccursCheckFailed, true);
        }
        // Safety: 与 error 分支同一不变量——sub_tp 经 follow 证非空、arena
        // 驻留节点就地写 Bound(super_tp)（C++ `emplaceTypePack(asMutable(subTp),
        // BoundTypePack{superTp})`），写入的句柄皆为 arena 驻留 TypePackId。
        unsafe {
          emplace_type_pack(
            as_mutable_type_pack(sub_tp),
            TypePackVariant::Bound(super_tp),
          )
        };
        return (UnifyResult::Ok, true);
      }

      if get_type_pack::get::<FreeTypePack>(super_tp).is_some() {
        if OccursCheckResult::Fail == occurs_check_type_pack_id_type_pack_id(super_tp, sub_tp) {
          // Safety: 对称分支——super_tp 同为 follow 归一后的非空 arena 驻留
          // pack 指针，emplace 就地写 Bound(error_pack) 无并存可变借用
          // （同 sub 侧 error 分支的证成）。
          unsafe {
            emplace_type_pack(
              as_mutable_type_pack(super_tp),
              TypePackVariant::Bound(error_pack),
            )
          };
          return (UnifyResult::OccursCheckFailed, true);
        }

        // Safety: 同 sub 侧成功分支——arena 驻留非空 pack 节点的就地
        // emplaceTypePack(asMutable(superTp), BoundTypePack{subTp}) 直译。
        unsafe {
          emplace_type_pack(
            as_mutable_type_pack(super_tp),
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
