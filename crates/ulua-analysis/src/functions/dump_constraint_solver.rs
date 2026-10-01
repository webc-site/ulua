//! `static void dump(ConstraintSolver* cs, ToStringOptions& opts)`
//! (`Analysis/src/ConstraintSolver.cpp:4315-4335`, hand-ported faithfully).

use alloc::vec::Vec;
use core::ptr::NonNull;

use ulua_common::fflag;

use crate::{
  functions::to_string_to_string::to_string_constraint_to_string_options,
  records::{
    arena_handle::{alias, alias_ref},
    constraint::Constraint,
    constraint_solver::ConstraintSolver,
    to_string_options::ToStringOptions,
  },
};

pub fn dump(cs: &mut ConstraintSolver, opts: &mut ToStringOptions) {
  if fflag::LuauConstraintGraph.get() {
    let unsolved: Vec<NonNull<Constraint>> = cs
      .unsolved_constraints
      .iter()
      // Safety: unsolved_constraints 值由 push_constraint 以 Box 堆地址登记，恒非空。
      .map(|c| {
        NonNull::new(*c as *mut Constraint)
          .expect("unsolved 表值由 Box 堆地址登记，恒非空")
      })
      .collect();
    alias(cs.cgraph).dump_with(&unsolved, opts);
  } else {
    for c in cs.unsolved_constraints.iter() {
      let c = *c;
      let block_count = cs
        .deprecated_blocked_constraints
        .get(&c)
        .map(|v| *v as i32)
        .unwrap_or(0);
      println!(
        "\t{}\t{}",
        block_count,
        to_string_constraint_to_string_options(alias_ref(c), opts)
      );
      for dep in &alias_ref(c).deprecated_dependencies {
        let dep_const = *dep as *const Constraint;
        if cs.unsolved_constraints.contains(&dep_const) {
          println!(
            "\t\t|\t{}",
            to_string_constraint_to_string_options(alias_ref(dep_const), opts)
          );
        }
      }
    }
  }
}
