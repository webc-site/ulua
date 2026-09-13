//! `static void dump(ConstraintSolver* cs, ToStringOptions& opts)`
//! (`Analysis/src/ConstraintSolver.cpp:4315-4335`, hand-ported faithfully).

use alloc::vec::Vec;
use core::ptr::NonNull;

use ulua_common::FFlag;

use crate::{
  functions::to_string_to_string_alt_q::to_string_constraint_to_string_options,
  records::{
    constraint::Constraint, constraint_solver::ConstraintSolver, to_string_options::ToStringOptions,
  },
};
/// # Safety
/// 调用方须保证 `cs` 等裸指针参数有效，且满足 C++ 原实现的调用契约。
pub unsafe fn dump(cs: *mut ConstraintSolver, opts: &mut ToStringOptions) {
  let cs_ref = unsafe { &mut *cs };

  if FFlag::LuauConstraintGraph.get() {
    let unsolved: Vec<NonNull<Constraint>> = cs_ref
      .unsolved_constraints
      .iter()
      .map(|c| NonNull::new(*c as *mut Constraint).unwrap())
      .collect();
    unsafe { (*cs_ref.cgraph).dump_with(&unsolved, opts) };
  } else {
    for c in cs_ref.unsolved_constraints.iter() {
      let c = *c;
      let block_count = cs_ref
        .deprecated_blocked_constraints
        .get(&c)
        .map(|v| *v as i32)
        .unwrap_or(0);
      println!(
        "\t{}\t{}",
        block_count,
        to_string_constraint_to_string_options(unsafe { &*c }, opts)
      );
      for dep in unsafe { &(*c).deprecated_dependencies } {
        let dep_const = *dep as *const Constraint;
        if cs_ref.unsolved_constraints.contains(&dep_const) {
          println!(
            "\t\t|\t{}",
            to_string_constraint_to_string_options(unsafe { &*dep_const }, opts)
          );
        }
      }
    }
  }
}
