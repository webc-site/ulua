use ulua_common::macros::luau_assert::LUAU_ASSERT;

// C++ `tableStuff` lambda（ConstraintSolver.cpp:2612-2633）：
// 命中 indexer 或可生长 indexer 时返回 Some(结果)，否则 None 继续外层分支。
use crate::functions::get_mutable_type::get_mutable_type_id;
use crate::{
  enums::table_state::TableState,
  functions::{add_union::add_union, follow_type::follow_type_id, get_type_alt_j::get_type_id},
  records::{
    assign_index_constraint::AssignIndexConstraint, constraint::Constraint,
    constraint_solver::ConstraintSolver, extern_type::ExternType, free_type::FreeType,
    intersection_type::IntersectionType, table_indexer::TableIndexer, table_type::TableType,
    type_ids::TypeIds, type_level::TypeLevel,
  },
  type_aliases::type_id::TypeId,
};
fn table_stuff(
  solver: &mut ConstraintSolver,
  c: &AssignIndexConstraint,
  constraint: *const Constraint,
  index_type: TypeId,
  rhs_type: TypeId,
  lhs_table: &mut TableType,
) -> Option<bool> {
  if let Some(indexer) = &lhs_table.indexer {
    solver.constraint_solver_unify(constraint, index_type, indexer.index_type);
    solver.constraint_solver_unify(constraint, rhs_type, indexer.index_result_type);

    unsafe {
      solver.bind_not_null_constraint_type_id_type_id(
        constraint,
        c.prop_type,
        add_union(
          solver.arena,
          solver.builtin_types,
          &[indexer.index_result_type, (*solver.builtin_types).nil_type],
        ),
      )
    };

    return Some(true);
  }

  if lhs_table.state == TableState::Unsealed || lhs_table.state == TableState::Free {
    lhs_table.indexer = Some(TableIndexer {
      index_type,
      index_result_type: rhs_type,
      is_read_only: false,
    });

    unsafe { solver.bind_not_null_constraint_type_id_type_id(constraint, c.prop_type, rhs_type) };
    return Some(true);
  }

  None
}

impl ConstraintSolver {
  /// # Safety
  /// 调用方须保证满足 C++ 原实现的调用契约。
  pub unsafe fn try_dispatch_assign_index_constraint_not_null_constraint(
    &mut self,
    c: &AssignIndexConstraint,
    constraint: *const Constraint,
  ) -> bool {
    let lhs_type: TypeId = follow_type_id(c.lhs_type);
    let index_type: TypeId = follow_type_id(c.index_type);
    let rhs_type: TypeId = follow_type_id(c.rhs_type);

    if self.is_blocked_type_id(lhs_type) {
      return self.block_type_id_not_null_constraint(lhs_type, constraint);
    }

    if let Some(lhs_free) = get_mutable_type_id::<FreeType>(lhs_type) {
      let lhs_upper = follow_type_id(lhs_free.upper_bound);
      if let Some(lhs_table) = get_mutable_type_id::<TableType>(lhs_upper)
        && let Some(v) = table_stuff(self, c, constraint, index_type, rhs_type, lhs_table)
      {
        return v;
      }

      let new_upper_bound = unsafe {
        (*self.arena).add_type(
          TableType::table_type_props_optional_table_indexer_type_level_scope_table_state(
            &Default::default(),
            Some(TableIndexer {
              index_type,
              index_result_type: rhs_type,
              is_read_only: false,
            }),
            TypeLevel::default(),
            // SAFETY: constraint 由调用方保证有效（NotNull 语义）。
            (*constraint).scope,
            TableState::Free,
          ),
        )
      };

      self.constraint_solver_unify(constraint, lhs_type, new_upper_bound);

      // C++ `LUAU_ASSERT(newTable); LUAU_ASSERT(newTable->indexer);`
      let new_table = get_type_id::<TableType>(new_upper_bound).expect("fresh table is TableType");
      LUAU_ASSERT!(new_table.indexer.is_some());

      let idx_res = new_table.indexer.as_ref().unwrap().index_result_type;
      unsafe { self.bind_not_null_constraint_type_id_type_id(constraint, c.prop_type, idx_res) };
      return true;
    }

    if let Some(lhs_table) = get_mutable_type_id::<TableType>(lhs_type)
      && let Some(v) = table_stuff(self, c, constraint, index_type, rhs_type, lhs_table)
    {
      return v;
    }

    // C++ `if (auto lhsExternType = get<ExternType>(lhsType))`：仅 lhs 为
    // ExternType 时进入，循环退出后 return true。
    if let Some(first_et) = get_type_id::<ExternType>(lhs_type) {
      let mut et = Some(first_et);
      while let Some(cur) = et {
        if let Some(indexer) = &cur.indexer {
          self.constraint_solver_unify(constraint, index_type, indexer.index_type);
          self.constraint_solver_unify(constraint, rhs_type, indexer.index_result_type);

          let res_ty = add_union(
            self.arena,
            self.builtin_types,
            &[indexer.index_result_type, unsafe {
              (*self.builtin_types).nil_type
            }],
          );
          unsafe { self.bind_not_null_constraint_type_id_type_id(constraint, c.prop_type, res_ty) };
          return true;
        }

        et = cur.parent.and_then(get_type_id::<ExternType>);
      }
      return true;
    }

    if let Some(lhs_intersection) = get_mutable_type_id::<IntersectionType>(lhs_type) {
      let mut parts = TypeIds::new();

      for &t in lhs_intersection.parts.iter() {
        let followed = follow_type_id(t);

        if let Some(tbl) = get_mutable_type_id::<TableType>(followed) {
          if let Some(indexer) = &tbl.indexer {
            self.constraint_solver_unify(constraint, index_type, indexer.index_type);
            parts.insert_type_id(indexer.index_result_type);
          }

          if tbl.state == TableState::Unsealed || tbl.state == TableState::Free {
            tbl.indexer = Some(TableIndexer {
              index_type,
              index_result_type: rhs_type,
              is_read_only: false,
            });
            parts.insert_type_id(rhs_type);
          }

          continue;
        }

        let mut cls = get_type_id::<ExternType>(followed);
        while let Some(et) = cls {
          if let Some(indexer) = &et.indexer {
            self.constraint_solver_unify(constraint, index_type, indexer.index_type);
            parts.insert_type_id(indexer.index_result_type);
            break;
          }

          cls = et.parent.and_then(get_type_id::<ExternType>);
        }
      }

      // SAFETY: constraint 由调用方保证有效（NotNull 语义）。
      let scope = unsafe { (*constraint).scope };
      // SAFETY: 同上。
      let location = unsafe { (*constraint).location };

      let res = self.simplify_intersection_not_null_scope_location_type_ids(scope, location, parts);
      self.constraint_solver_unify(constraint, rhs_type, res);
    }

    // Other types do not support index assignment.
    unsafe {
      self.bind_not_null_constraint_type_id_type_id(
        constraint,
        c.prop_type,
        (*self.builtin_types).error_type,
      )
    };

    true
  }
}
