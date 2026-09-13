use alloc::vec::Vec;
use core::ptr::NonNull;

use ulua_ast::records::location::Location;
use ulua_common::FFlag;

use crate::{
  enums::{polarity::Polarity, table_state::TableState},
  functions::{
    extend_type_pack::extend_type_pack, find_metatable_entry::find_metatable_entry,
    follow_type::follow_type_id, fresh_type::fresh_type, get_type_alt_j::get_type_id,
    instantiate::instantiate, track_interior_free_type::track_interior_free_type,
  },
  records::{
    any_type::AnyType, constraint::Constraint, constraint_solver::ConstraintSolver,
    free_type::FreeType, function_type::FunctionType, iterable_constraint::IterableConstraint,
    metatable_type::MetatableType, never_type::NeverType, primitive_type::PrimitiveType,
    reduce_constraint::ReduceConstraint, table_indexer::TableIndexer, table_type::TableType,
    type_level::TypeLevel, unification_too_complex::UnificationTooComplex,
  },
  type_aliases::{
    constraint_v::ConstraintV, props_type::Props, type_error_data::TypeErrorData, type_id::TypeId,
  },
};
impl ConstraintSolver {
  pub fn try_dispatch_iterable_table(
    &mut self,
    iterator_ty: TypeId,
    c: &IterableConstraint,
    constraint: &Constraint,
    force: bool,
  ) -> bool {
    let iterator_ty = follow_type_id(iterator_ty);

    if get_type_id::<FreeType>(iterator_ty).is_some() {
      let scope = constraint.scope;
      let key_ty = fresh_type(
        // SAFETY: arena 在 solver 存活期内有效。
        unsafe { &mut *self.arena },
        // SAFETY: builtin_types 在 solver 存活期内有效。
        unsafe { &*self.builtin_types },
        scope,
        Polarity::Mixed,
      );
      let value_ty = fresh_type(
        // SAFETY: arena 在 solver 存活期内有效。
        unsafe { &mut *self.arena },
        // SAFETY: builtin_types 在 solver 存活期内有效。
        unsafe { &*self.builtin_types },
        scope,
        Polarity::Mixed,
      );
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
            TableState::Sealed,
          ),
        )
      };

      self.constraint_solver_unify(constraint, iterator_ty, table_ty);

      let mut it = c.variables.iter();
      if let Some(ty) = it.next() {
        unsafe { self.bind_not_null_constraint_type_id_type_id(constraint, *ty, key_ty) };
      }
      if let Some(ty) = it.next() {
        unsafe { self.bind_not_null_constraint_type_id_type_id(constraint, *ty, value_ty) };
      }

      return true;
    }

    if get_type_id::<AnyType>(iterator_ty).is_some() {
      self.unpack_iterable_variables(constraint, c, unsafe { (*self.builtin_types).any_type });
      return true;
    }

    if get_type_id::<NeverType>(iterator_ty).is_some() {
      self.unpack_iterable_variables(constraint, c, unsafe { (*self.builtin_types).never_type });
      return true;
    }

    // Irksome: I don't think we have any way to guarantee that this table
    // type never has a metatable.

    if let Some(iterator_table) = get_type_id::<TableType>(iterator_ty) {
      if iterator_table.state == TableState::Free && !force {
        return self.block_type_id_not_null_constraint(iterator_ty, constraint);
      }

      if let Some(indexer) = iterator_table.indexer {
        let value_type = if FFlag::LuauRefineNilFromTableIndexerResultType.get() {
          let intersection_with_not_nil = unsafe {
            (*self.arena).add_type_function_type_function_initializer_list_type_id(
              &(*self.builtin_types).type_functions.intersect_func,
              &[
                indexer.index_result_type,
                (*self.builtin_types).not_nil_type,
              ],
            )
          };

          self.push_constraint(
            NonNull::new(constraint.scope).unwrap(),
            constraint.location,
            ConstraintV::Reduce(ReduceConstraint {
              ty: intersection_with_not_nil,
            }),
          );

          intersection_with_not_nil
        } else {
          indexer.index_result_type
        };

        let mut expected_variables = alloc::vec![indexer.index_type, value_type];
        while expected_variables.len() < c.variables.len() {
          expected_variables.push(unsafe { (*self.builtin_types).error_type });
        }

        for (variable, expected) in c.variables.iter().zip(expected_variables.iter()) {
          self.constraint_solver_unify(constraint, *variable, *expected);
          unsafe {
            self.bind_not_null_constraint_type_id_type_id(constraint, *variable, *expected)
          };
        }
      } else {
        self.unpack_iterable_variables(constraint, c, unsafe { (*self.builtin_types).error_type });
      }

      return true;
    }

    // else if (std::optional<TypeId> iterFn = findMetatableEntry(builtinTypes, errors, iteratorTy, "__iter", Location{}))
    let iter_fn = find_metatable_entry(
      self.builtin_types,
      &mut self.errors,
      iterator_ty,
      "__iter",
      Location::default(),
    );
    if let Some(iter_fn) = iter_fn {
      if self.is_blocked_type_id(iter_fn) {
        return self.block_type_id_not_null_constraint(iter_fn, constraint);
      }

      let scope = constraint.scope;
      let instantiated_iter_fn = instantiate(
        // SAFETY: builtin_types/arena 在 solver 存活期内有效。
        unsafe { &*self.builtin_types },
        unsafe { &mut *self.arena },
        &self.limits,
        scope,
        iter_fn,
      );

      if let Some(instantiated_iter_fn) = instantiated_iter_fn {
        if let Some(iter_ftv) = get_type_id::<FunctionType>(instantiated_iter_fn) {
          let expected_iter_args =
            unsafe { (*self.arena).add_type_pack_initializer_list_type_id(&[iterator_ty]) };
          self.constraint_solver_unify(constraint, iter_ftv.arg_types, expected_iter_args);

          let iter_rets = unsafe {
            extend_type_pack(
              &mut *self.arena,
              self.builtin_types,
              iter_ftv.ret_types,
              2,
              Vec::new(),
            )
          };

          if iter_rets.head.is_empty() {
            // We've done what we can; this will get reported as an
            // error by the type checker.
            return true;
          }

          let next_fn_ty = iter_rets.head[0];

          let instantiated_next_fn = instantiate(
            // SAFETY: builtin_types/arena 在 solver 存活期内有效。
            unsafe { &*self.builtin_types },
            unsafe { &mut *self.arena },
            &self.limits,
            scope,
            next_fn_ty,
          );

          if let Some(instantiated_next_fn) = instantiated_next_fn {
            // If nextFn is nullptr, then the iterator function has an improper signature.
            if let Some(next_fn) = get_type_id::<FunctionType>(instantiated_next_fn) {
              let ret_types = next_fn.ret_types;
              self.unpack_and_assign(c.variables.clone(), ret_types, NonNull::from(constraint));
            }

            return true;
          } else {
            let location = constraint.location;
            self.report_error_type_error_data_location(
              TypeErrorData::UnificationTooComplex(UnificationTooComplex::default()),
              &location,
            );
          }
        } else {
          // TODO: Support __call and function overloads (what does an overload even mean for this?)
        }
      } else {
        let location = constraint.location;
        self.report_error_type_error_data_location(
          TypeErrorData::UnificationTooComplex(UnificationTooComplex::default()),
          &location,
        );
      }

      return true;
    }

    // else if (auto iteratorMetatable = get<MetatableType>(iteratorTy))
    if let Some(iterator_metatable) = get_type_id::<MetatableType>(iterator_ty) {
      // If the metatable does not contain a `__iter` metamethod, then we iterate over the table part of the metatable.
      return self.try_dispatch_iterable_table(iterator_metatable.table, c, constraint, force);
    }

    if let Some(primitive_ty) = get_type_id::<PrimitiveType>(iterator_ty)
      && primitive_ty.r#type == PrimitiveType::TABLE
    {
      self.unpack_iterable_variables(constraint, c, unsafe { (*self.builtin_types).unknown_type });
      return true;
    }

    self.unpack_iterable_variables(constraint, c, unsafe { (*self.builtin_types).error_type });

    true
  }

  fn unpack_iterable_variables(
    &mut self,
    constraint: &Constraint,
    c: &IterableConstraint,
    ty: TypeId,
  ) {
    for var_ty in &c.variables {
      unsafe { self.bind_not_null_constraint_type_id_type_id(constraint, *var_ty, ty) };
    }
  }
}
