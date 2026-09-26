//! `static void dump(ConstraintSolver* cs, ToStringOptions& opts)`
//! (`Analysis/src/ConstraintSolver.cpp:4315-4335`, hand-ported faithfully).

use alloc::vec::Vec;
use core::ptr::NonNull;

use ulua_common::fflag;

use crate::{
  functions::to_string_to_string::to_string_constraint_to_string_options,
  records::{
    constraint::Constraint, constraint_solver::ConstraintSolver, to_string_options::ToStringOptions,
  },
};
/// # Safety
/// `cs` 须指向调用方持有的存活 `ConstraintSolver`（非空、对齐，比本次 dump 长寿）；`opts` 为独占
/// 可变引用。本函数只读遍历 cs 的约束/类型图，要求 dump 期内无并发写。对应 C++
/// `ConstraintSolver::dump` 系列 (`cpp/Analysis/src/ConstraintSolver.cpp:4315`)。
pub unsafe fn dump(cs: *mut ConstraintSolver, opts: &mut ToStringOptions) {
  // Safety: cs 为调用方传入的活跃 ConstraintSolver 裸指针（cpp `dump(ConstraintSolver*)`
  // 无条件解引用），非空、对齐且比本次同步 dump 长寿；此处独占重建 &mut，单线程无并发借用。
  let cs_ref = unsafe { &mut *cs };

  if fflag::LuauConstraintGraph.get() {
    let unsolved: Vec<NonNull<Constraint>> = cs_ref
      .unsolved_constraints
      .iter()
      // Safety: unsolved_constraints 值由 push_constraint 以 Box 堆地址登记，恒非空。
      .map(|c| {
        NonNull::new(*c as *mut Constraint)
          .expect("unsolved 表值由 Box 堆地址登记，恒非空")
      })
      .collect();
    // Safety: cgraph 在 ConstraintSolver 构造时接线（`result.cgraph = cgraph`），非空且
    // 比 solver 长寿；此分支仅在 constraint graph 特性开启、cgraph 已接好时进入，只读导出。
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
        to_string_constraint_to_string_options(
          unsafe {
            // Safety: c 指向 solver_constraints 中 Box<Constraint> 的堆对象（地址稳定、
            // 比 solver 长寿），unsolved_constraints 仅存其裸指针，此处只读借用。
            &*c
          },
          opts
        )
      );
      // Safety: 同上 c 为存活非空 Constraint 指针，只读借用其 deprecated_dependencies 字段。
      for dep in unsafe { &(*c).deprecated_dependencies } {
        let dep_const = *dep as *const Constraint;
        if cs_ref.unsolved_constraints.contains(&dep_const) {
          println!(
            "\t\t|\t{}",
            to_string_constraint_to_string_options(
              unsafe {
                // Safety: dep 指向 Box<Constraint> 堆对象（地址稳定、存活），dep_const
                // 为同一对象的只读视图，仅只读格式化。
                &*dep_const
              },
              opts
            )
          );
        }
      }
    }
  }
}
