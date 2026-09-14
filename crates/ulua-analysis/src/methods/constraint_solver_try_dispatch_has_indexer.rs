use alloc::vec::Vec;
use core::ptr::null;

use ulua_common::{FFlag, records::dense_hash_set::DenseHashSet};

use crate::{
  enums::{polarity::Polarity, table_state::TableState},
  functions::{
    as_mutable_type::as_mutable_type_id, follow_type::follow_type_id, fresh_type::fresh_type,
    get_mutable_type::get_mutable_type_id, get_type_alt_j::get_type_id,
    track_interior_free_type::track_interior_free_type,
  },
  records::{
    any_type::AnyType, blocked_type::BlockedType, constraint::Constraint,
    constraint_solver::ConstraintSolver, extern_type::ExternType, free_type::FreeType,
    intersection_builder::IntersectionBuilder, intersection_type::IntersectionType,
    metatable_type::MetatableType, never_type::NeverType, set::Set, table_indexer::TableIndexer,
    table_type::TableType, type_level::TypeLevel, union_builder::UnionBuilder,
    union_type::UnionType,
  },
  type_aliases::{error_type::ErrorType, type_id::TypeId, type_variant::TypeVariant},
};
impl ConstraintSolver {
  /// # Safety
  /// 调用方须保证满足 C++ 原实现的调用契约。
  pub unsafe fn constraint_solver_try_dispatch_has_indexer(
    &mut self,
    _recursion_depth: &mut i32,
    constraint: *const Constraint,
    subject_type: TypeId,
    index_type: TypeId,
    mut result_type: TypeId,
    _seen: &mut DenseHashSet<TypeId>,
  ) -> bool {
    let subject_type = follow_type_id(subject_type);
    let index_type = follow_type_id(index_type);

    if _seen.contains(&subject_type) {
      return false;
    }
    _seen.insert(subject_type);

    if get_type_id::<AnyType>(subject_type).is_some() {
      unsafe {
        self.bind_not_null_constraint_type_id_type_id(
          constraint,
          result_type,
          (*self.builtin_types).any_type,
        )
      };
      return true;
    }

    if let Some(free_type) = get_mutable_type_id::<FreeType>(subject_type) {
      let upper_bound = follow_type_id(free_type.upper_bound);

      if let Some(table) = get_type_id::<TableType>(upper_bound) {
        if let Some(indexer) = &table.indexer {
          self.constraint_solver_unify(constraint, index_type, indexer.index_type);
          unsafe {
            self.bind_not_null_constraint_type_id_type_id(
              constraint,
              result_type,
              indexer.index_result_type,
            )
          };
          return true;
        }
      } else if let Some(metatable) = get_type_id::<MetatableType>(upper_bound) {
        return unsafe {
          self.constraint_solver_try_dispatch_has_indexer(
            _recursion_depth,
            constraint,
            metatable.table(),
            index_type,
            result_type,
            _seen,
          )
        };
      }

      let scope = free_type.scope;
      let free_result = fresh_type(
        // SAFETY: arena 在 solver 存活期内有效。
        unsafe { &mut *self.arena },
        // SAFETY: builtin_types 在 solver 存活期内有效。
        unsafe { &*self.builtin_types },
        scope,
        Polarity::Mixed,
      );
      track_interior_free_type(scope, free_result);
      unsafe {
        self.bind_not_null_constraint_type_id_type_id(constraint, result_type, free_result)
      };
      result_type = free_result;

      let mut table = TableType::table_type_table_state_type_level_scope(
        TableState::Unsealed,
        TypeLevel::default(),
        scope,
      );
      table.indexer = Some(TableIndexer {
        index_type,
        index_result_type: free_result,
        is_read_only: false,
      });

      let upper_bound = unsafe { (*self.arena).add_type(table) };
      let simplified = self.simplify_intersection_not_null_scope_location_type_id_type_id(
        // SAFETY: constraint 由调用方保证有效（NotNull 语义）。
        unsafe { (*constraint).scope },
        // SAFETY: 同上。
        unsafe { (*constraint).location },
        free_type.upper_bound,
        upper_bound,
      );

      if get_type_id::<NeverType>(simplified).is_some() {
        unsafe {
          self.bind_not_null_constraint_type_id_type_id(
            constraint,
            result_type,
            (*self.builtin_types).error_type,
          )
        };
      } else {
        free_type.upper_bound = simplified;
      }

      return true;
    }

    if let Some(table) = get_mutable_type_id::<TableType>(subject_type) {
      if let Some(indexer) = &table.indexer {
        self.constraint_solver_unify(constraint, index_type, indexer.index_type);
        unsafe {
          self.bind_not_null_constraint_type_id_type_id(
            constraint,
            result_type,
            indexer.index_result_type,
          )
        };
        return true;
      }

      if table.state == TableState::Unsealed {
        let scope = table.scope;
        let free_result = fresh_type(
          // SAFETY: arena 在 solver 存活期内有效。
          unsafe { &mut *self.arena },
          // SAFETY: builtin_types 在 solver 存活期内有效。
          unsafe { &*self.builtin_types },
          scope,
          Polarity::Mixed,
        );
        track_interior_free_type(scope, free_result);
        unsafe {
          self.bind_not_null_constraint_type_id_type_id(constraint, result_type, free_result)
        };
        table.indexer = Some(TableIndexer {
          index_type,
          index_result_type: result_type,
          is_read_only: false,
        });
        return true;
      }
    }

    if let Some(metatable) = get_type_id::<MetatableType>(subject_type) {
      return unsafe {
        self.constraint_solver_try_dispatch_has_indexer(
          _recursion_depth,
          constraint,
          metatable.table(),
          index_type,
          result_type,
          _seen,
        )
      };
    }

    let mut extern_type = get_type_id::<ExternType>(subject_type);
    while let Some(et) = extern_type {
      if let Some(indexer) = &et.indexer {
        self.constraint_solver_unify(constraint, index_type, indexer.index_type);
        unsafe {
          self.bind_not_null_constraint_type_id_type_id(
            constraint,
            result_type,
            indexer.index_result_type,
          )
        };
        return true;
      }

      extern_type = et.parent.and_then(get_type_id::<ExternType>);
    }

    if let Some(it) = get_type_id::<IntersectionType>(subject_type) {
      // Indexing into an intersection of types is roughly akin to overload
      // selection: for every type in the intersection where it is well typed
      // to index into _that_ type, we construct an intersection of said result
      // types.
      if FFlag::LuauRemoveConstraintSolverEmplace.get() {
        let mut ib = IntersectionBuilder::new(self.arena, self.builtin_types);
        let mut success = false;

        let parts: Vec<TypeId> = it.parts.clone();
        for part in parts {
          let r = unsafe { (*self.arena).add_type(BlockedType::default()) };
          // C++ `getMutable<BlockedType>(r)->setOwner(...)`：r 刚 emplace 为
          // BlockedType，下转必命中。
          get_mutable_type_id::<BlockedType>(r)
            .expect("fresh blocked type")
            .set_owner(constraint);

          let ok = unsafe {
            self.constraint_solver_try_dispatch_has_indexer(
              _recursion_depth,
              constraint,
              part,
              index_type,
              r,
              _seen,
            )
          };
          // If we've cut a recursive loop short, skip it.
          if !ok {
            continue;
          }

          let r = follow_type_id(r);
          if get_type_id::<ErrorType>(r).is_none() {
            success = true;
            ib.add(r);
          }
        }

        // We need to distinguish between the empty case (there
        // were no valid indexable types) and the bottom type (one of the
        // indexable result types was never). UnionBuilder will opt to
        // only record that its seen a top type as an optimization. we
        // add a flag to distinguish these cases.
        if success {
          let built = ib.build();
          unsafe { self.bind_not_null_constraint_type_id_type_id(constraint, result_type, built) };
        } else {
          unsafe {
            self.bind_not_null_constraint_type_id_type_id(
              constraint,
              result_type,
              (*self.builtin_types).error_type,
            )
          };
        }
      } else {
        let mut parts: Set<TypeId> = Set::new(null());
        let part_list: Vec<TypeId> = it.parts.clone();
        for part in part_list {
          parts.insert(&follow_type_id(part));
        }

        let mut results: Set<TypeId> = Set::new(null());

        let parts_iter: Vec<TypeId> = parts.iter().copied().collect();
        for part in parts_iter {
          let r = unsafe { (*self.arena).add_type(BlockedType::default()) };
          // C++ `getMutable<BlockedType>(r)->setOwner(...)`：r 刚 emplace 为
          // BlockedType，下转必命中。
          get_mutable_type_id::<BlockedType>(r)
            .expect("fresh blocked type")
            .set_owner(constraint);

          let ok = unsafe {
            self.constraint_solver_try_dispatch_has_indexer(
              _recursion_depth,
              constraint,
              part,
              index_type,
              r,
              _seen,
            )
          };
          // If we've cut a recursive loop short, skip it.
          if !ok {
            continue;
          }

          let r = follow_type_id(r);
          if get_type_id::<ErrorType>(r).is_none() {
            results.insert(&r);
          }
        }

        if results.size() == 0 {
          unsafe {
            self.bind_not_null_constraint_type_id_type_id(
              constraint,
              result_type,
              (*self.builtin_types).error_type,
            )
          };
        } else if results.size() == 1 {
          let first = *results.iter().next().unwrap();
          unsafe { self.bind_not_null_constraint_type_id_type_id(constraint, result_type, first) };
        } else {
          let parts_vec: Vec<TypeId> = results.iter().copied().collect();
          let mutable_ty = { as_mutable_type_id(result_type) };
          unsafe {
            (*mutable_ty).ty = TypeVariant::Intersection(IntersectionType { parts: parts_vec });
          }
          let location = unsafe { (*constraint).location };
          self.unblock_type_id_location(result_type, location);
        }
      }

      return true;
    }

    if let Some(ut) = get_type_id::<UnionType>(subject_type) {
      // Indexing into a union of types means constructing a union of
      // results: we don't know _which_ type it could be.
      if FFlag::LuauRemoveConstraintSolverEmplace.get() {
        let mut ub = UnionBuilder::new(self.arena, self.builtin_types);
        let mut success = false;

        let options: Vec<TypeId> = ut.options.clone();
        for option in options {
          let r = unsafe { (*self.arena).add_type(BlockedType::default()) };
          // C++ `getMutable<BlockedType>(r)->setOwner(...)`：r 刚 emplace 为
          // BlockedType，下转必命中。
          get_mutable_type_id::<BlockedType>(r)
            .expect("fresh blocked type")
            .set_owner(constraint);

          let ok = unsafe {
            self.constraint_solver_try_dispatch_has_indexer(
              _recursion_depth,
              constraint,
              option,
              index_type,
              r,
              _seen,
            )
          };
          // If we've cut a recursive loop short, skip it.
          if !ok {
            continue;
          }

          let r = follow_type_id(r);
          success = true;
          ub.add(r);
        }

        // We need to distinguish between the empty case (there
        // were no valid indexable types) and the top type (one of the
        // indexable result types was unknown). UnionBuilder will opt to
        // only record that its seen a top type as an optimization. we
        // add a flag to distinguish these cases.
        if success {
          let built = ub.build();
          unsafe { self.bind_not_null_constraint_type_id_type_id(constraint, result_type, built) };
        } else {
          unsafe {
            self.bind_not_null_constraint_type_id_type_id(
              constraint,
              result_type,
              (*self.builtin_types).error_type,
            )
          };
        }
      } else {
        let mut parts: Set<TypeId> = Set::new(null());
        let option_list: Vec<TypeId> = ut.options.clone();
        for part in option_list {
          parts.insert(&follow_type_id(part));
        }

        let mut results: Set<TypeId> = Set::new(null());

        let parts_iter: Vec<TypeId> = parts.iter().copied().collect();
        for part in parts_iter {
          let r = unsafe { (*self.arena).add_type(BlockedType::default()) };
          // C++ `getMutable<BlockedType>(r)->setOwner(...)`：r 刚 emplace 为
          // BlockedType，下转必命中。
          get_mutable_type_id::<BlockedType>(r)
            .expect("fresh blocked type")
            .set_owner(constraint);

          let ok = unsafe {
            self.constraint_solver_try_dispatch_has_indexer(
              _recursion_depth,
              constraint,
              part,
              index_type,
              r,
              _seen,
            )
          };
          // If we've cut a recursive loop short, skip it.
          if !ok {
            continue;
          }

          let r = follow_type_id(r);
          results.insert(&r);
        }

        if results.size() == 0 {
          unsafe {
            self.bind_not_null_constraint_type_id_type_id(
              constraint,
              result_type,
              (*self.builtin_types).error_type,
            )
          };
        } else if results.size() == 1 {
          let first_result = *results.iter().next().unwrap();
          if !FFlag::LuauConstraintGraph.get() {
            // bind will already shift references.
            self.deprecate_d_shift_references(result_type, first_result);
          }
          unsafe {
            self.bind_not_null_constraint_type_id_type_id(constraint, result_type, first_result)
          };
        } else {
          let options_vec: Vec<TypeId> = results.iter().copied().collect();
          let mutable_ty = { as_mutable_type_id(result_type) };
          unsafe {
            (*mutable_ty).ty = TypeVariant::Union(UnionType {
              options: options_vec,
            });
          }
          let location = unsafe { (*constraint).location };
          self.unblock_type_id_location(result_type, location);
        }
      }

      return true;
    }

    unsafe {
      self.bind_not_null_constraint_type_id_type_id(
        constraint,
        result_type,
        (*self.builtin_types).error_type,
      )
    };
    true
  }
}
