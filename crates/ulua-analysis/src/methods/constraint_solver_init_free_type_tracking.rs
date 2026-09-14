use ulua_common::{FFlag, macros::luau_assert::LUAU_ASSERT};

use crate::{
  functions::{
    to_string_to_string_alt_m::to_string_type_id_to_string_options,
    to_string_to_string_alt_n::to_string_type_pack_id_to_string_options,
    to_string_to_string_alt_q::to_string_constraint_to_string_options,
  },
  records::{constraint::Constraint, constraint_solver::ConstraintSolver},
  type_aliases::constraint_vertex::ConstraintVertex,
};
impl ConstraintSolver {
  pub fn init_free_type_tracking(&mut self) {
    if FFlag::LuauConstraintGraph.get() {
      for c in &self.constraints {
        let borrow = unsafe { &**c };
        self.unsolved_constraints.push(borrow as *const Constraint);

        let (types, type_packs) = borrow.get_maybe_mutated_types();

        for ty in types.order.iter() {
          unsafe {
            (*self.cgraph).add_dependency_of_constraint_vertex_constraint_vertex(
              ConstraintVertex::V2(borrow as *const Constraint),
              ConstraintVertex::V0(*ty),
            )
          };
          if FFlag::DebugLuauLogSolver.get() {
            let ty_str = to_string_type_id_to_string_options(*ty, &mut self.opts.clone());
            let c_str = to_string_constraint_to_string_options(borrow, &mut self.opts.clone());
            println!("Type {} depends on constraint {}", ty_str, c_str);
          }
        }

        for tp in type_packs.iter() {
          unsafe {
            (*self.cgraph).add_dependency_of_constraint_vertex_constraint_vertex(
              ConstraintVertex::V2(borrow as *const Constraint),
              ConstraintVertex::V1(*tp),
            )
          };
          if FFlag::DebugLuauLogSolver.get() {
            let tp_str = to_string_type_pack_id_to_string_options(*tp, &mut self.opts.clone());
            let c_str = to_string_constraint_to_string_options(borrow, &mut self.opts.clone());
            println!("Type pack {} depends on constraint {}", tp_str, c_str);
          }
        }
      }
    } else {
      let constraints = self.constraints.clone();
      for c in &constraints {
        let borrow = unsafe { &**c };
        self.unsolved_constraints.push(borrow as *const Constraint);

        let (types, _type_packs) = borrow.get_maybe_mutated_types();

        for ty in types.order.iter() {
          let key = *ty;
          let entry = self
            .deprecated_type_to_constraint_set
            .entry(key)
            .or_default();
          // We don't care if this is fresh, we can blindly insert.
          entry.insert(borrow as *const Constraint);
        }

        let (_types, fresh1) = self
          .deprecated_constraint_to_mutated_types
          .try_insert(borrow as *const Constraint, types);
        LUAU_ASSERT!(fresh1);

        for dep in &borrow.deprecated_dependencies {
          self.block_not_null_constraint_not_null_constraint(*dep, borrow as *const Constraint);
        }
      }
    }
  }
}
