//! 泛化前的直接界环塌缩预处理（`cpp/Analysis/src/Generalization.cpp:824-986` 的
//! `collapseInvariantFreeType` / `collapseDirectBoundCycleAt` /
//! `collapseFreeTypeCycles`）。
//!
//! 直接自由界（`A.lower`/`A.upper` 即另一个自由类型）构成的环若不先塌缩，
//! 泛化时环内成员会被 `generalizeType` 的防自指逻辑剥成无界泛型，导致本可
//! 判定为具体类型的解被错误地保留为 generic。预处理先把「上下界 follow 后
//! 相等」的成员原地绑定（invariant-free），再把环成员合并到一个代表自由
//! 类型：外部下界取并、外部上界取交，环内自引用从界中 `remove_type` 剥除。

use ulua_common::{
  macros::luau_assert::LUAU_ASSERT, records::insertion_ordered_map::InsertionOrderedMap,
};

use crate::{
  functions::{
    follow_type, generalize_type::emplace_bound, get_mutable_type, get_type,
    remove_type::remove_type,
  },
  records::{
    arena_handle::Handle, builtin_types::BuiltinTypes, free_type::FreeType,
    free_type_finder::FreeTypeFinder, generalization_params::GeneralizationParams,
    intersection_builder::IntersectionBuilder, iterative_type_visitor::IterativeTypeVisitorTrait,
    never_type::NeverType, type_arena::TypeArena, type_ids::TypeIds, union_builder::UnionBuilder,
    unknown_type::UnknownType,
  },
  type_aliases::type_id::TypeId,
};

/// C++ `getDirectFreeNeighbor`：界 follow 后仍是自由类型才返回，否则 `None`。
fn get_direct_free_neighbor(ty: TypeId) -> Option<TypeId> {
  let ty = follow_type::follow(ty);
  get_type::get::<FreeType>(ty).map(|_| ty)
}

/// C++ `collapseInvariantFreeType`：把从 `start_ty` 可达、且上下界 follow 后
/// 相等（并指向自身之外）的自由类型直接绑定到该公共界上。
fn collapse_invariant_free_type(arena: Handle<TypeArena>, start_ty: TypeId) {
  let mut finder = FreeTypeFinder::new(arena);
  finder.run_type_id(start_ty);

  for ty in finder.free_tys.begin() {
    let Some(ft) = get_mutable_type::get_mutable::<FreeType>(ty) else {
      continue;
    };
    let upper_bound = follow_type::follow(ft.upper_bound);
    let lower_bound = follow_type::follow(ft.lower_bound);
    if upper_bound == lower_bound && upper_bound != ty {
      emplace_bound(ty, upper_bound);
    }
  }
}

/// C++ `collapseDirectBoundCycleAt`：从 `start_ty` 沿直接自由界行走，一旦发现
/// 环，就把环成员合并到环的入边目标（代表）：各成员的外部下界并入代表的
/// 下界、外部上界交入代表的上界，其余成员绑定到代表。命中塌缩返回 `true`。
pub fn collapse_direct_bound_cycle_at(
  arena: Handle<TypeArena>,
  builtin_types: Handle<BuiltinTypes>,
  start_ty: TypeId,
) -> bool {
  collapse_invariant_free_type(arena, start_ty);

  let start_ty = follow_type::follow(start_ty);
  if get_type::get::<FreeType>(start_ty).is_none() {
    return false;
  }

  let mut path = TypeIds::new();
  let mut cur = Some(start_ty);

  while let Some(cur_ty) = cur {
    if path.count(cur_ty) != 0 {
      // 环成员 = path 上从 `cur_ty`（入边目标）起的后缀。
      let mut cycle_members = TypeIds::new();
      let mut in_cycle = false;
      for member in path.begin() {
        in_cycle |= member == cur_ty;
        if in_cycle {
          cycle_members.insert_type_id(member);
        }
      }
      LUAU_ASSERT!(cycle_members.size() > 0);

      // 合并外部界：下界取并、上界取交；整条界只是环自引用的直接跳过，
      // 嵌套自引用先用 `remove_type` 剥除。
      let mut merged_lowers = UnionBuilder::new(arena, builtin_types);
      let mut merged_uppers = IntersectionBuilder::new(arena, builtin_types);
      for member in cycle_members.begin() {
        let Some(ft) = get_mutable_type::get_mutable::<FreeType>(member) else {
          continue;
        };

        let lower_bound = follow_type::follow(ft.lower_bound);
        if cycle_members.count(lower_bound) == 0 {
          for other in cycle_members.begin() {
            remove_type(arena, builtin_types, lower_bound, other);
          }
          let lower_bound = follow_type::follow(lower_bound);
          if get_type::get::<NeverType>(lower_bound).is_none()
            && cycle_members.count(lower_bound) == 0
          {
            merged_lowers.add(lower_bound);
          }
        }

        let upper_bound = follow_type::follow(ft.upper_bound);
        if cycle_members.count(upper_bound) == 0 {
          for other in cycle_members.begin() {
            remove_type(arena, builtin_types, upper_bound, other);
          }
          let upper_bound = follow_type::follow(upper_bound);
          if get_type::get::<UnknownType>(upper_bound).is_none()
            && cycle_members.count(upper_bound) == 0
          {
            merged_uppers.add(upper_bound);
          }
        }
      }

      // 代表（环入边目标）接过合并后的界，其余成员绑定到它。
      let rep = cycle_members.front();
      let rep_free = get_mutable_type::get_mutable::<FreeType>(rep);
      LUAU_ASSERT!(rep_free.is_some());
      let rep_free = rep_free.expect("LUAU_ASSERT(rep_free.is_some()) 蕴含");
      rep_free.lower_bound = merged_lowers.build();
      rep_free.upper_bound = merged_uppers.build();

      for member in cycle_members.begin() {
        if member != rep {
          emplace_bound(member, rep);
        }
      }
      return true;
    }

    path.insert_type_id(cur_ty);

    let Some(ft) = get_mutable_type::get_mutable::<FreeType>(cur_ty) else {
      break;
    };

    // 优先沿上界、再下界走直接自由邻居；自指邻居视为无邻居。
    cur = match get_direct_free_neighbor(ft.upper_bound) {
      Some(neighbor) if neighbor != cur_ty => Some(neighbor),
      _ => get_direct_free_neighbor(ft.lower_bound).filter(|neighbor| *neighbor != cur_ty),
    };
  }

  false
}

/// C++ `collapseFreeTypeCycles`：对泛化前沿中的每个自由类型跑一轮
/// [`collapse_direct_bound_cycle_at`]。塌缩后成员不再是自由类型，后续起点
/// 立即终止，故迭代顺序无关紧要。
pub fn collapse_free_type_cycles(
  arena: Handle<TypeArena>,
  builtin_types: Handle<BuiltinTypes>,
  free_types: &InsertionOrderedMap<TypeId, GeneralizationParams>,
) {
  for (start_ty, _) in free_types {
    collapse_direct_bound_cycle_at(arena, builtin_types, *start_ty);
  }
}
