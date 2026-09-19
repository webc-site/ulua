use alloc::{rc::Rc, vec::Vec};
use core::mem::take;
use std::collections::HashMap;

use ulua_common::{macros::luau_assert::LUAU_ASSERT, records::dense_hash_set::DenseHashSet};

use crate::{
  enums::solver_mode::SolverMode,
  functions::make_type_check_limits::make_type_check_limits,
  records::{
    build_queue_item::BuildQueueItem, build_queue_work_state::BuildQueueWorkState,
    frontend::Frontend, frontend_options::FrontendOptions,
  },
  type_aliases::{frontend_callbacks::TaskQueue, module_name_type::ModuleName},
};

impl Frontend {
  /// 检查 `module_queue` 里排队的模块。
  ///
  /// `execute_tasks` 是任务派发方（C++: `executeTasks`）：它收到的是构建队列**下标**，
  /// 并通过回调的执行器 `run` 在当前线程完成检查；默认（cpp 亦同）为顺序就地执行。
  ///
  /// C++: `Frontend::checkQueuedModules(optionOverride, executeTasks, progress)`。
  pub fn check_queued_modules(
    &mut self,
    option_override: Option<FrontendOptions>,
    mut execute_tasks: TaskQueue,
    progress: impl Fn(usize, usize) -> bool,
  ) -> Vec<ModuleName> {
    let mut frontend_options = option_override.unwrap_or_else(|| self.options.clone());
    if self.get_luau_solver_mode() == SolverMode::New {
      frontend_options.for_autocomplete = false;
    }

    // By taking data into locals, the queue is cleared at the end even on an ICE.
    let curr_module_queue: Vec<ModuleName> = take(&mut self.module_queue);

    let mut seen: DenseHashSet<ModuleName> = DenseHashSet::new(ModuleName::default());

    // C++ 直接往 `state->buildQueueItems` 里追加；这里先落在局部 Vec 上，
    // 到首次派发时才包进共享的 `Rc<BuildQueueWorkState>`，全程不存在 &mut 别名。
    let mut build_queue_items: Vec<BuildQueueItem> = Vec::new();

    for name in &curr_module_queue {
      if seen.contains(name) {
        continue;
      }

      if !self.is_dirty(name, frontend_options.for_autocomplete) {
        seen.insert(name.clone());
        continue;
      }

      let mut queue: Vec<ModuleName> = Vec::new();
      let cycle_detected = self.parse_graph(
        &mut queue,
        name,
        &make_type_check_limits(&frontend_options),
        frontend_options.for_autocomplete,
      );

      self.add_build_queue_items(
        &mut build_queue_items,
        &queue,
        cycle_detected,
        &mut seen,
        &frontend_options,
      );
    }

    if build_queue_items.is_empty() {
      return Vec::new();
    }

    // Mapping from modules to build queue slots.
    let mut module_name_to_queue: HashMap<ModuleName, usize> = HashMap::new();
    for (i, item) in build_queue_items.iter().enumerate() {
      module_name_to_queue.insert(item.name.clone(), i);
    }

    // Record dependencies between modules.
    // 先只读地收集每个模块指向的「脏依赖」下标，再统一写回，避免同时持有两处借用。
    let dirty_deps: Vec<Vec<usize>> = build_queue_items
      .iter()
      .map(|item| {
        item
          .source_node
          .require_set
          .iter()
          .filter(|dep| {
            self
              .source_nodes
              .get(*dep)
              .is_some_and(|node| node.has_dirty_module(frontend_options.for_autocomplete))
          })
          .map(|dep| module_name_to_queue[dep])
          .collect()
      })
      .collect();

    for (i, positions) in dirty_deps.iter().enumerate() {
      if positions.is_empty() {
        continue;
      }
      build_queue_items[i].dirty_dependencies += positions.len() as i32;
      for &dep_pos in positions {
        build_queue_items[dep_pos].reverse_deps.push(i);
      }
    }

    let mut next_items: Vec<usize> = Vec::new();

    // First pass: check all modules with no pending dependencies.
    for (i, item) in build_queue_items.iter().enumerate() {
      if item.dirty_dependencies == 0 {
        next_items.push(i);
      }
    }

    // 从这里开始队列成为共享句柄：`Rc` + `Mutex<QueueState>` + `Condvar`，
    // 派发侧（`perform_queue_item_task`）与回收侧（下面的等待循环）都只经锁访问。
    let state = Rc::new(BuildQueueWorkState::new(build_queue_items));

    if !next_items.is_empty() {
      self.send_queue_item_tasks(state.clone(), take(&mut next_items), &mut execute_tasks);
    }

    // If not a single item was found, a cycle in the graph was hit.
    if state.processing() == 0 {
      self.send_queue_cycle_item_task(state.clone(), &mut execute_tasks);
    }

    let mut item_with_exception: Option<usize> = None;
    let mut cancelled = false;

    while state.remaining() != 0 {
      {
        let guard = state.lock();

        // If nothing is ready yet, wait.
        let mut guard = state.wait_for_ready_tasks(guard);

        // Handle checked items. 原地取空就绪队列，与 C++ 处理后清空等价
        let ready: Vec<usize> = take(&mut guard.ready_queue_items);
        for i in ready.iter().copied() {
          let (has_exception, is_cancelled) = {
            let item = &guard.build_queue_items[i];
            (item.exception.is_some(), item.module.cancelled)
          };
          if has_exception {
            item_with_exception = Some(i);
          }
          if is_cancelled {
            cancelled = true;
          }

          if item_with_exception.is_some() || cancelled {
            break;
          }

          self.record_item_result(&guard.build_queue_items[i]);

          // Notify items waiting on this dependency.
          let reverse_deps: Vec<usize> = guard.build_queue_items[i].reverse_deps.clone();
          for reverse_dep in reverse_deps {
            let dep = &mut guard.build_queue_items[reverse_dep];
            LUAU_ASSERT!(dep.dirty_dependencies != 0);
            dep.dirty_dependencies -= 1;

            if !dep.processing && dep.dirty_dependencies == 0 {
              next_items.push(reverse_dep);
            }
          }
        }

        let ready_len = ready.len();
        LUAU_ASSERT!(guard.processing >= ready_len);
        guard.processing -= ready_len;

        LUAU_ASSERT!(guard.remaining >= ready_len);
        guard.remaining -= ready_len;
      }

      let (done, total) = state.progress_counts();
      if !progress(done, total) {
        cancelled = true;
      }

      // Items cannot be submitted while holding the lock.
      if !next_items.is_empty() {
        self.send_queue_item_tasks(state.clone(), take(&mut next_items), &mut execute_tasks);
      }

      if state.processing() == 0 {
        // Typechecking might have been cancelled by user; don't return partial results.
        if cancelled {
          return Vec::new();
        }

        // We might have stopped because of a pending exception.
        if let Some(idx) = item_with_exception {
          let guard = state.lock();
          self.record_item_result(&guard.build_queue_items[idx]);
        }
      }

      // If we aren't done but have nothing processing, we hit a cycle.
      if state.remaining() != 0 && state.processing() == 0 {
        self.send_queue_cycle_item_task(state.clone(), &mut execute_tasks);
      }
    }

    let mut checked_modules: Vec<ModuleName> = Vec::new();
    {
      let mut guard = state.lock();
      checked_modules.reserve(guard.build_queue_items.len());
      for item in guard.build_queue_items.iter_mut() {
        checked_modules.push(take(&mut item.name));
      }
    }

    checked_modules
  }
}
