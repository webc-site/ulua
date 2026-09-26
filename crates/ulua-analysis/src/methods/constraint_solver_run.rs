//! `void ConstraintSolver::run()` (`Analysis/src/ConstraintSolver.cpp:506-765`,
//! the main solver loop, hand-ported faithfully). The C++ `runSolverPass` lambda
//! is lowered to the private `run_solver_pass` method below.

use alloc::{string::String, vec::Vec};
use core::ptr::NonNull;

use ulua_ast::records::location::Location;
use ulua_common::{
  fflag, fint,
  functions::get_clock::get_clock,
  macros::luau_assert::LUAU_ASSERT,
  records::{dense_hash_set::DenseHashSet, variant::Variant2},
};

use crate::{
  functions::{
    dump_bindings::dump_bindings,
    dump_constraint_solver::dump,
    follow_type,
    to_string_to_string::{to_string_constraint_to_string_options, to_string_type_id},
  },
  records::{
    arena_handle::alias, blocked_constraint_registry::register_constraint, constraint::Constraint,
    constraint_solver::ConstraintSolver,
    constraint_solving_incomplete_error::ConstraintSolvingIncompleteError,
  },
  type_aliases::{
    blocked_constraint_id::BlockedConstraintId, module_name_type::ModuleName, type_id::TypeId,
  },
};
impl ConstraintSolver {
  pub fn constraint_solver_run(&mut self) {
    // LUAU_TIMETRACE_SCOPE("ConstraintSolver::run", "Typechecking");

    if self.is_done() {
      return;
    }

    if fflag::DebugLuauLogSolver.get() {
      let (human, name) = match &self.module {
        Some(m) => (m.human_readable_name.clone(), m.name.clone()),
        None => (String::new(), ModuleName::new()),
      };
      println!("Starting solver for module {} ({})", human, name);
      let mut opts = self.opts.clone();
      // Safety: `self as *mut _` 派生自当前存活的 `&mut self` 借用，仅作 C++
      // `static dump(ConstraintSolver*, opts&)` 的临时裸指针重表达；被调方在
      // 调用期内独占再借该指针打印状态，期间 self 别无借用（opts 已先行
      // clone 脱离，返回后经 `self.opts = opts` 写回）。
      unsafe { dump(self as *mut ConstraintSolver, &mut opts) };
      self.opts = opts;
      println!("Bindings:");
      dump_bindings(self.root_scope_ref(), &mut self.opts.clone());
    }

    if !self.logger.is_null() {
      let unsolved = self.unsolved_constraints.clone();
      // is_null 短路在先；logger 为 SolverParams 注入、活过整个求解过程的可空
      // 裸句柄，alias 单点物化为独占借用。root_scope 契约见 `root_scope_ref`。
      alias(self.logger).capture_initial_solver_state(self.root_scope_ref(), &unsolved);
    }

    // Free types that have no constraints at all can be generalized right away.
    // 单一快照供两分支共用；循环期间 callee 可能回写 free_types，不可 take。
    let free_types: Vec<TypeId> = self.constraint_set.free_types.order.clone();
    if fflag::LuauConstraintGraph.get() {
      // TODO CLI-206649: We can fold constraint set into constraint graph.
      LUAU_ASSERT!(!self.cgraph.is_null());
      for ty in free_types {
        // Safety: cgraph 与 logger 同为构造期 SolverParams 注入；C++ 约束图布线
        // 决定 LuauConstraintGraph 开启时该指针非空（函数头断言钉住），指向的
        // ConstraintGraph 活过整个 run 循环；has_unsolved_dependencies 只在图内
        // 查依赖表，ty 是 Copy 句柄键。
        if !unsafe { (*self.cgraph).has_unsolved_dependencies(BlockedConstraintId::V0(ty)) } {
          self.generalize_one_type(ty);
        }
      }
    } else {
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

    loop {
      let mut progress = self.run_solver_pass(false);
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

    if fflag::DebugLuauLogSolver.get() || fflag::DebugLuauLogBindings.get() {
      dump_bindings(self.root_scope_ref(), &mut self.opts.clone());
    }

    if !self.logger.is_null() {
      let unsolved = self.unsolved_constraints.clone();
      alias(self.logger).capture_final_solver_state(self.root_scope_ref(), &unsolved);
    }
  }

  /// 两处「快照→generalizeOneType→提交」序列合一（cpp ConstraintSolver.cpp:598-612；
  /// deprecated 路径为旧版 cpp 同款重复）。
  fn generalize_logging_snapshot(&mut self, ty: TypeId) {
    let mut snap = None;
    if !self.logger.is_null() {
      let unsolved = self.unsolved_constraints.clone();
      // is_null 短路在先；prepare_* 返回按值快照，借用止于本次调用。
      snap = Some(alias(self.logger).prepare_generalization_snapshot(
        to_string_type_id(ty),
        self.root_scope_ref(),
        &unsolved,
      ));
    }

    self.generalize_one_type(ty);

    // snap 为 Some 蕴含 logger 非空。
    if let Some(mut s) = snap {
      s.after = to_string_type_id(ty);
      // snap 为 Some 蕴含 logger 非空且自构造后无改写点。
      alias(self.logger).commit_step_snapshot(Variant2::V1(s));
    }
  }

  /// C++ `auto runSolverPass = [&](bool force) { ... };`
  fn run_solver_pass(&mut self, force: bool) -> bool {
    let mut progress = false;

    let mut i: usize = 0;
    while i < self.unsolved_constraints.len() {
      // `c` 取自 unsolved_constraints——其元素是指向 solver_constraints 中
      // Box<Constraint> 的非空裸指针（NotNull 语义：入 vec 时即非空；Box 堆址
      // 稳定，且 unsolved 移除只删指针不释放 Box，Constraint 活过本循环）。
      let c: *const Constraint = self.unsolved_constraints[i];
      if fflag::LuauConstraintGraph.get() {
        // Safety: LuauConstraintGraph 开启时 cgraph 为构造期注入的非空图指针
        // （C++ 布线不变量），图与 solver 同生命周期；has_unsolved_dependencies
        // 对 c 仅按 C++ 同款 `&**c` 只读判其变体，不写约束本体。
        if !force
          && unsafe {
            (*self.cgraph)
              .has_unsolved_dependencies(BlockedConstraintId::V2(register_constraint(c)))
          }
        {
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
      if fint::LuauSolverConstraintLimit.get() > 0 && self.solver_constraint_limit == 0 {
        break;
      }

      let save_me: String = if fflag::DebugLuauLogSolver.get() {
        // Safety: c 非空且指向 solver_constraints 保活的 Box<Constraint>，
        // 共享再借仅用于只读字符串化。
        to_string_constraint_to_string_options(unsafe { &*c }, &mut self.opts.clone())
      } else {
        String::new()
      };

      let mut snapshot = None;
      if !self.logger.is_null() {
        let unsolved = self.unsolved_constraints.clone();
        // is_null 短路保证 logger 非空；root_scope 契约见 `root_scope_ref`；
        // c 存活如上；返回按值快照。
        snapshot = Some(alias(self.logger).prepare_step_snapshot(
          self.root_scope_ref(),
          c,
          force,
          &unsolved,
        ));
      }

      if fflag::DebugLuauAssertOnForcedConstraint.get() {
        LUAU_ASSERT!(!force);
      }

      // Safety: c 指向上文取出的 solver_constraints 存活 Box<Constraint>
      // （NotNull：非空，地址稳定至求解结束）；共享再借交给分发逻辑只读
      // （对应 C++ `tryDispatch(const Constraint& c, ...)`），Box 不会被并发
      // 释放——本函数持有 `&mut self`，unsolved/solver 两 vec 都不变。
      let success = self.try_dispatch_not_null_constraint_bool(unsafe { &*c }, force);

      progress |= success;

      if success {
        if !self.logger.is_null()
          && let Some(snap) = snapshot.take()
        {
          // Safety: snapshot 仅在 is_null 判空分支产生；logger 构造期注入后
          // run 期间无改写点，此处仍非空且独占再借。
          unsafe { (*self.logger).commit_step_snapshot(Variant2::V0(snap)) };
        }

        if fflag::LuauConstraintGraph.get() {
          LUAU_ASSERT!(!self.cgraph.is_null());
          // Safety: cgraph 由上方断言钉住非空（flag 开启期构造布线保证）；
          // const→mut 仅为适配形参 `NonNull<Constraint>` 的形式需要——
          // unblock_constraint 内部转回 *const 作依赖图身份键，绝不写约束
          // 本体；c 非空故 unwrap 安全。
          let unblock_result = unsafe {
            (*self.cgraph).unblock_constraint(
              NonNull::new(c as *mut Constraint)
                .expect("c 为刚 add/入队的存活约束指针，非空（见上方 Safety 注）"),
            )
          };

          // We need to handle the logger here.
          if !self.logger.is_null() {
            // Safety: is_null 判空紧邻在先；c 只作日志弹栈的身份键传递，
            // 不经该指针解引用。
            unsafe { (*self.logger).pop_block_not_null_constraint(c) };
          }

          self.unsolved_constraints.remove(i);

          let unblocked_types: Vec<TypeId> = unblock_result.types.order.clone();
          for ty in unblocked_types {
            // Safety: cgraph 非空不变量同本函数头（flag 开启期构造布线），
            // 图为 solver 生命周期内的稳定对象；ty 为 Copy 句柄键。
            if !unsafe { (*self.cgraph).has_unsolved_dependencies(BlockedConstraintId::V0(ty)) } {
              self.generalize_logging_snapshot(ty);
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
            let mut seen: DenseHashSet<TypeId> = DenseHashSet::default();
            for ty in mutated {
              // There is a high chance that this type has been rebound
              // across blocked types, rebound free types, pending
              // expansion types, etc, so we need to follow it.
              let ty = follow_type::follow(ty);
              if seen.contains(&ty) {
                continue;
              }
              seen.insert(ty);

              let present = self.deprecated_type_to_constraint_set.contains_key(&ty);
              if present {
                let (became_small, became_empty) = {
                  let set = self
                    .deprecated_type_to_constraint_set
                    .get_mut(&ty)
                    .expect("紧邻上方 contains_key(&ty) 同判据为真蕴含 get_mut 必命中");
                  set.remove(&c);
                  (set.len() <= 1, set.is_empty())
                };
                if became_small {
                  self.unblock_type_id_location(ty, Location::default());
                }

                if became_empty {
                  self.generalize_logging_snapshot(ty);
                }
              }
            }
          }
        }

        if fflag::DebugLuauLogSolver.get() {
          if force {
            std::print!("Force ");
          }
          std::print!("Dispatched\n\t{}\n", save_me);

          if force && fflag::LuauConstraintGraph.get() {
            let mut opts = self.opts.clone();
            // Safety: 同 unblock_constraint 处——cgraph 非空不变量由 flag 布线
            // 保证，const→mut 仅是身份键形参的类型适配，dump_blocked 只读打印
            // 依赖图，c 指向存活 Box。
            unsafe {
              (*self.cgraph).dump_blocked(
                NonNull::new(c as *mut Constraint)
                  .expect("c 指向存活 Box（见上方 Safety 注），非空"),
                &mut opts,
              )
            };
            self.opts = opts;
          }

          let mut opts = self.opts.clone();
          // Safety: `self as *mut _` 派生自当前存活的 `&mut self`，是 C++
          // `dump(cs, opts)` 的临时裸化；opts 已 clone 脱离、调用后写回，
          // 被调方打印期内 self 别无借用。
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
