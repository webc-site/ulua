//! `void ConstraintSolver::run()` (`Analysis/src/ConstraintSolver.cpp:506-765`,
//! the main solver loop, hand-ported faithfully). The C++ `runSolverPass` lambda
//! is lowered to the private `run_solver_pass` method below.

use alloc::{string::String, vec::Vec};
use core::ptr::{NonNull, null};

use ulua_ast::records::location::Location;
use ulua_common::{
  FFlag, FInt,
  functions::get_clock::get_clock,
  records::{dense_hash_set::DenseHashSet, variant::Variant2},
};

use crate::{
  functions::{
    dump_bindings::dump_bindings, dump_constraint_solver::dump, follow_type::follow_type_id,
    to_string_to_string_alt_c::to_string_type_id,
    to_string_to_string_alt_q::to_string_constraint_to_string_options,
  },
  records::{
    constraint::Constraint, constraint_solver::ConstraintSolver,
    constraint_solving_incomplete_error::ConstraintSolvingIncompleteError,
  },
  type_aliases::{constraint_vertex::ConstraintVertex, type_id::TypeId},
};
impl ConstraintSolver {
  pub fn constraint_solver_run(&mut self) {
    // LUAU_TIMETRACE_SCOPE("ConstraintSolver::run", "Typechecking");

    if self.is_done() {
      return;
    }

    if FFlag::DebugLuauLogSolver.get() {
      let (human, name) = match &self.module {
        Some(m) => (m.human_readable_name.clone(), m.name.clone()),
        None => (String::new(), String::new()),
      };
      println!("Starting solver for module {} ({})", human, name);
      let mut opts = self.opts.clone();
      unsafe { dump(self as *mut ConstraintSolver, &mut opts) };
      self.opts = opts;
      println!("Bindings:");
      unsafe { dump_bindings(self.root_scope, &mut self.opts.clone()) };
    }

    if !self.logger.is_null() {
      let unsolved = self.unsolved_constraints.clone();
      unsafe { (*self.logger).capture_initial_solver_state(&*self.root_scope, &unsolved) };
    }

    // Free types that have no constraints at all can be generalized right away.
    if FFlag::LuauConstraintGraph.get() {
      ulua_common::macros::luau_assert::LUAU_ASSERT!(!self.cgraph.is_null());
      // TODO CLI-206649: We can fold constraint set into constraint graph.
      let free_types: Vec<TypeId> = self.constraint_set.free_types.order.clone();
      for ty in free_types {
        if !unsafe { (*self.cgraph).has_unsolved_dependencies(ConstraintVertex::V0(ty)) } {
          self.generalize_one_type(ty);
        }
      }
    } else {
      let free_types: Vec<TypeId> = self.constraint_set.free_types.order.clone();
      for ty in free_types {
        let empty = match self.deprecated_type_to_constraint_set.get(&ty) {
          Some(set) => set.is_empty(),
          None => true,
        };
        if empty {
          self.generalize_one_type(ty);
        }
      }
    }

    self.constraint_set.free_types.clear();

    let mut progress;
    loop {
      progress = self.run_solver_pass(false);
      if !progress {
        progress |= self.run_solver_pass(true);
      }
      if !progress {
        break;
      }
    }

    if !self.unsolved_constraints.is_empty() {
      self.report_error_type_error_data_location(
        ConstraintSolvingIncompleteError::default().into(),
        &Location::default(),
      );
    }

    // After we have run all the constraints, type functions should be generalized
    // At this point, we can try to perform one final simplification to suss out
    // whether type functions are truly uninhabited or if they can reduce

    self.constraint_solver_finalize_type_functions();

    if FFlag::DebugLuauLogSolver.get() || FFlag::DebugLuauLogBindings.get() {
      unsafe { dump_bindings(self.root_scope, &mut self.opts.clone()) };
    }

    if !self.logger.is_null() {
      let unsolved = self.unsolved_constraints.clone();
      unsafe { (*self.logger).capture_final_solver_state(&*self.root_scope, &unsolved) };
    }
  }

  /// C++ `auto runSolverPass = [&](bool force) { ... };`
  fn run_solver_pass(&mut self, force: bool) -> bool {
    let mut progress = false;

    let mut i: usize = 0;
    while i < self.unsolved_constraints.len() {
      let c: *const Constraint = self.unsolved_constraints[i];
      if FFlag::LuauConstraintGraph.get() {
        if !force && unsafe { (*self.cgraph).has_unsolved_dependencies(ConstraintVertex::V2(c)) } {
          i += 1;
          continue;
        }
      } else if !force && self.deprecate_d_is_blocked(c) {
        i += 1;
        continue;
      }

      if let Some(finish_time) = self.limits.finish_time()
        && get_clock() > finish_time
      {
        self.constraint_solver_throw_time_limit_error();
      }
      if let Some(token) = self.limits.cancellation_token()
        && token.requested()
      {
        self.constraint_solver_throw_user_cancel_error();
      }

      // If we were _given_ a limit, and the current limit has hit zero,
      // then early exit from constraint solving.
      if FInt::LuauSolverConstraintLimit.get() > 0 && self.solver_constraint_limit == 0 {
        break;
      }

      let save_me: String = if FFlag::DebugLuauLogSolver.get() {
        to_string_constraint_to_string_options(unsafe { &*c }, &mut self.opts.clone())
      } else {
        String::new()
      };

      let mut snapshot = None;
      if !self.logger.is_null() {
        let unsolved = self.unsolved_constraints.clone();
        snapshot = Some(unsafe {
          (*self.logger).prepare_step_snapshot(&*self.root_scope, c, force, &unsolved)
        });
      }

      if FFlag::DebugLuauAssertOnForcedConstraint.get() {
        ulua_common::macros::luau_assert::LUAU_ASSERT!(!force);
      }

      let success = unsafe { self.try_dispatch_not_null_constraint_bool(c, force) };

      progress |= success;

      if success {
        if !self.logger.is_null()
          && let Some(snap) = snapshot.take()
        {
          unsafe { (*self.logger).commit_step_snapshot(Variant2::V0(snap)) };
        }

        if FFlag::LuauConstraintGraph.get() {
          ulua_common::macros::luau_assert::LUAU_ASSERT!(!self.cgraph.is_null());
          let unblock_result = unsafe {
            (*self.cgraph).unblock_constraint(NonNull::new(c as *mut Constraint).unwrap())
          };

          // We need to handle the logger here.
          if !self.logger.is_null() {
            unsafe { (*self.logger).pop_block_not_null_constraint(c) };
          }

          self.unsolved_constraints.remove(i);

          let unblocked_types: Vec<TypeId> = unblock_result.types.order.clone();
          for ty in unblocked_types {
            if !unsafe { (*self.cgraph).has_unsolved_dependencies(ConstraintVertex::V0(ty)) } {
              let mut snap = None;
              if !self.logger.is_null() {
                let unsolved = self.unsolved_constraints.clone();
                snap = Some(unsafe {
                  (*self.logger).prepare_generalization_snapshot(
                    to_string_type_id(ty),
                    &*self.root_scope,
                    &unsolved,
                  )
                });
              }

              self.generalize_one_type(ty);

              if !self.logger.is_null()
                && let Some(mut s) = snap.take()
              {
                s.after = to_string_type_id(ty);
                unsafe { (*self.logger).commit_step_snapshot(Variant2::V1(s)) };
              }

              self.unblock_type_id_location(ty, Location::default());
            }
          }

          // TODO CLI-206534: We never eagerly generalize free type
          // packs. Maybe we should.
        } else {
          self.constraint_solver_deprecate_d_unblock(c);
          self.unsolved_constraints.remove(i);
          if let Some(entry) = self.deprecated_constraint_to_mutated_types.find(&c) {
            let mutated: Vec<TypeId> = entry.order.clone();
            let mut seen: DenseHashSet<TypeId> = DenseHashSet::new(null());
            for ty in mutated {
              // There is a high chance that this type has been rebound
              // across blocked types, rebound free types, pending
              // expansion types, etc, so we need to follow it.
              let ty = follow_type_id(ty);
              if seen.contains(&ty) {
                continue;
              }
              seen.insert(ty);

              let present = self.deprecated_type_to_constraint_set.contains_key(&ty);
              if present {
                let (became_small, became_empty) = {
                  let set = self.deprecated_type_to_constraint_set.get_mut(&ty).unwrap();
                  set.remove(&c);
                  (set.len() <= 1, set.is_empty())
                };
                if became_small {
                  self.unblock_type_id_location(ty, Location::default());
                }

                if became_empty {
                  let mut snap = None;
                  if !self.logger.is_null() {
                    let unsolved = self.unsolved_constraints.clone();
                    snap = Some(unsafe {
                      (*self.logger).prepare_generalization_snapshot(
                        to_string_type_id(ty),
                        &*self.root_scope,
                        &unsolved,
                      )
                    });
                  }

                  self.generalize_one_type(ty);

                  if !self.logger.is_null()
                    && let Some(mut s) = snap.take()
                  {
                    s.after = to_string_type_id(ty);
                    unsafe { (*self.logger).commit_step_snapshot(Variant2::V1(s)) };
                  }
                }
              }
            }
          }
        }

        if FFlag::DebugLuauLogSolver.get() {
          if force {
            std::print!("Force ");
          }
          std::print!("Dispatched\n\t{}\n", save_me);

          if force && FFlag::LuauConstraintGraph.get() {
            let mut opts = self.opts.clone();
            unsafe {
              (*self.cgraph).dump_blocked(NonNull::new(c as *mut Constraint).unwrap(), &mut opts)
            };
            self.opts = opts;
          }

          let mut opts = self.opts.clone();
          unsafe { dump(self as *mut ConstraintSolver, &mut opts) };
          self.opts = opts;
        }
      } else {
        i += 1;
      }

      if force && success {
        return true;
      }
    }

    progress
  }
}
