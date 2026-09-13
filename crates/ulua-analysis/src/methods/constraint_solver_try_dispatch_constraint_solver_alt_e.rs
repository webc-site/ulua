use alloc::vec::Vec;

use crate::{
  enums::{polarity::Polarity, table_state::TableState},
  functions::{
    extend_type_pack::extend_type_pack, follow_type::follow_type_id, fresh_type::fresh_type,
    get_type_alt_j::get_type_id, track_interior_free_type::track_interior_free_type,
  },
  records::{
    constraint::Constraint, constraint_solver::ConstraintSolver, free_type::FreeType,
    function_type::FunctionType, iterable_constraint::IterableConstraint,
    table_indexer::TableIndexer, table_type::TableType, type_level::TypeLevel,
  },
  type_aliases::props_type::Props,
};
impl ConstraintSolver {
  /// # Safety
  /// 调用方须保证满足 C++ 原实现的调用契约。
  pub unsafe fn try_dispatch_iterable_constraint_not_null_constraint_bool(
    &mut self,
    c: &IterableConstraint,
    constraint: *const Constraint,
    force: bool,
  ) -> bool {
    let iterator = unsafe {
      extend_type_pack(
        &mut *self.arena,
        self.builtin_types,
        c.iterator,
        3,
        Vec::new(),
      )
    };

    if iterator.head.len() < 3
      && let Some(tail) = iterator.tail
      && self.is_blocked_type_pack_id(tail)
    {
      return if force {
        true
      } else {
        self.block_type_pack_id_not_null_constraint(tail, constraint)
      };
    }

    let mut blocked = false;
    for ty in &iterator.head {
      if self.is_blocked_type_id(*ty) {
        self.block_type_id_not_null_constraint(*ty, constraint);
        blocked = true;
      }
    }

    if blocked {
      return false;
    }

    if iterator.head.is_empty() {
      for ty in &c.variables {
        unsafe {
          self.bind_not_null_constraint_type_id_type_id(
            constraint,
            *ty,
            (*self.builtin_types).error_type,
          )
        };
      }
      return true;
    }

    let next_ty = follow_type_id(iterator.head[0]);
    if get_type_id::<FreeType>(next_ty).is_some() {
      let scope = unsafe { (*constraint).scope };
      let key_ty = unsafe {
        fresh_type(
          &mut *self.arena,
          &*self.builtin_types,
          scope,
          Polarity::Mixed,
        )
      };
      let value_ty = unsafe {
        fresh_type(
          &mut *self.arena,
          &*self.builtin_types,
          scope,
          Polarity::Mixed,
        )
      };
      track_interior_free_type(scope, key_ty);
      track_interior_free_type(scope, value_ty);

      let props = Props::default();
      let table_ty = unsafe {
        (*self.arena).add_type(
          TableType::table_type_props_optional_table_indexer_type_level_scope_table_state(
            &props,
            Some(TableIndexer {
              index_type: key_ty,
              index_result_type: value_ty,
              is_read_only: false,
            }),
            TypeLevel::default(),
            scope,
            TableState::Free,
          ),
        )
      };
      track_interior_free_type(scope, table_ty);

      self.constraint_solver_unify(constraint, next_ty, table_ty);

      let mut it = c.variables.iter();
      if let Some(ty) = it.next() {
        unsafe { self.bind_not_null_constraint_type_id_type_id(constraint, *ty, key_ty) };
      }
      if let Some(ty) = it.next() {
        unsafe { self.bind_not_null_constraint_type_id_type_id(constraint, *ty, value_ty) };
      }
      for ty in it {
        unsafe {
          self.bind_not_null_constraint_type_id_type_id(
            constraint,
            *ty,
            (*self.builtin_types).nil_type,
          )
        };
      }

      return true;
    }

    if get_type_id::<FunctionType>(next_ty).is_some() {
      let table_ty = iterator
        .head
        .get(1)
        .copied()
        .unwrap_or_else(|| unsafe { (*self.builtin_types).nil_type });

      return unsafe { self.try_dispatch_iterable_function(next_ty, table_ty, c, &*constraint) };
    }

    // SAFETY: constraint 由约束分发器保证有效（NotNull 语义）。
    self.try_dispatch_iterable_table(iterator.head[0], c, unsafe { &*constraint }, force)
  }
}
