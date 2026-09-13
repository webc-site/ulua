use ulua_common::{FFlag, macros::luau_assert::LUAU_ASSERT};

use crate::{
  enums::{table_state::TableState, value_context::ValueContext},
  functions::{
    follow_type::follow_type_id, get_mutable_type::get_mutable, get_type_alt_j::get_type_id,
    lookup_extern_type_prop::lookup_extern_type_prop, maybe_string::maybe_string,
  },
  records::{
    assign_prop_constraint::AssignPropConstraint, blocked_type::BlockedType,
    constraint::Constraint, constraint_solver::ConstraintSolver, extern_type::ExternType,
    metatable_type::MetatableType, property_type::Property, table_type::TableType,
    union_type::UnionType,
  },
};

impl ConstraintSolver {
  /// # Safety
  /// 调用方须保证满足 C++ 原实现的调用契约。
  pub unsafe fn try_dispatch_assign_prop_constraint_not_null_constraint(
    &mut self,
    c: &AssignPropConstraint,
    constraint: *const Constraint,
  ) -> bool {
    let mut lhs_type = follow_type_id(c.lhs_type);
    let rhs_type = follow_type_id(c.rhs_type);

    if self.is_blocked_type_id(lhs_type) {
      return self.block_type_id_not_null_constraint(lhs_type, constraint);
    }

    if let Some(lhs_extern_type) = get_type_id::<ExternType>(lhs_type) {
      let Some(prop) = lookup_extern_type_prop(lhs_extern_type, &c.prop_name) else {
        unsafe {
          self.bind_not_null_constraint_type_id_type_id(
            constraint,
            c.prop_type,
            (*self.builtin_types).any_type,
          )
        };
        return true;
      };

      if let Some(write_ty) = prop.write_ty {
        unsafe { self.bind_not_null_constraint_type_id_type_id(constraint, c.prop_type, write_ty) };
        self.constraint_solver_unify(constraint, rhs_type, write_ty);
      } else {
        unsafe {
          self.bind_not_null_constraint_type_id_type_id(
            constraint,
            c.prop_type,
            (*self.builtin_types).any_type,
          )
        };
      }

      return true;
    }

    let lookup = self.lookup_table_prop_not_null_constraint_type_id_string_value_context_bool_bool(
      constraint,
      lhs_type,
      &c.prop_name,
      ValueContext::LValue,
      false,
      false,
    );
    if !lookup.blocked_types.is_empty() {
      for blocked in lookup.blocked_types {
        self.block_type_id_not_null_constraint(blocked, constraint);
      }
      return false;
    }

    if let Some(prop_ty) = lookup.prop_type {
      let bound_prop_ty = if lookup.is_index {
        unsafe {
          (*self.arena).add_type(UnionType {
            options: alloc::vec![prop_ty, (*self.builtin_types).nil_type],
          })
        }
      } else {
        prop_ty
      };

      unsafe {
        self.bind_not_null_constraint_type_id_type_id(constraint, c.prop_type, bound_prop_ty)
      };
      self.constraint_solver_unify(constraint, rhs_type, prop_ty);
      return true;
    }

    let mut lhs_table = get_mutable::<TableType>(lhs_type);
    if lhs_table.is_none()
      && let Some(lhs_meta) = get_type_id::<MetatableType>(lhs_type)
    {
      lhs_type = follow_type_id(lhs_meta.table);
      lhs_table = get_mutable::<TableType>(lhs_type);
    }

    if let Some(table) = lhs_table {
      if let Some(prop) = table.props.get_mut(&c.prop_name) {
        if let Some(write_ty) = prop.write_ty {
          unsafe {
            self.bind_not_null_constraint_type_id_type_id(constraint, c.prop_type, write_ty)
          };
          self.constraint_solver_unify(constraint, rhs_type, write_ty);
          return true;
        }

        if (table.state == TableState::Unsealed || table.state == TableState::Free)
          && prop.read_ty.is_some()
        {
          prop.write_ty = prop.read_ty;
          let write_ty = prop.write_ty.unwrap();
          unsafe {
            self.bind_not_null_constraint_type_id_type_id(constraint, c.prop_type, write_ty)
          };
          self.constraint_solver_unify(constraint, rhs_type, write_ty);
          return true;
        }

        unsafe {
          self.bind_not_null_constraint_type_id_type_id(
            constraint,
            c.prop_type,
            (*self.builtin_types).error_type,
          )
        };
        return true;
      }

      if let Some(indexer) = &table.indexer
        && maybe_string(indexer.index_type)
      {
        let prop_ty = indexer.index_result_type;
        let union = unsafe {
          (*self.arena).add_type(UnionType {
            options: alloc::vec![prop_ty, (*self.builtin_types).nil_type],
          })
        };
        unsafe { self.bind_not_null_constraint_type_id_type_id(constraint, c.prop_type, union) };
        self.constraint_solver_unify(constraint, rhs_type, prop_ty);
        return true;
      }

      if table.state == TableState::Unsealed || table.state == TableState::Free {
        if FFlag::LuauConstraintGraph.get() {
          LUAU_ASSERT!(!self.cgraph.is_null());
          unsafe { (*self.cgraph).copy_dependencies_of_type_id(lhs_type, rhs_type) };
        } else {
          self.deprecate_d_shift_references(lhs_type, rhs_type);
        }

        unsafe { self.bind_not_null_constraint_type_id_type_id(constraint, c.prop_type, rhs_type) };

        let mut prop = Property::rw_type_id(rhs_type);
        prop.location = c.prop_location;
        table.props.insert(c.prop_name.clone(), prop);

        if table.state == TableState::Unsealed && c.decrement_prop_count {
          LUAU_ASSERT!(table.remaining_props > 0);
          table.remaining_props -= 1;
          self.unblock_type_id_location(lhs_type, unsafe { (*constraint).location });
        }

        return true;
      }
    }

    let prop_type_is_blocked = get_type_id::<BlockedType>(c.prop_type).is_some();
    if prop_type_is_blocked {
      unsafe {
        self.bind_not_null_constraint_type_id_type_id(
          constraint,
          c.prop_type,
          (*self.builtin_types).error_type,
        )
      };
    }

    true
  }
}
