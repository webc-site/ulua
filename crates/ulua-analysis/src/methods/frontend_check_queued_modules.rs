use alloc::{boxed::Box, sync::Arc, vec::Vec};
use core::cell::Cell;
/// Adapts a `Task` (`FnOnce`) into the `Box<dyn Fn()>` shape expected by the
/// caller-provided `execute_tasks`. The Closure runs the task at most once.
use core::mem::take;
use std::{
  collections::HashMap,
  sync::{Condvar, Mutex},
};

use ulua_common::{macros::luau_assert::LUAU_ASSERT, records::dense_hash_set::DenseHashSet};

use crate::{
  enums::solver_mode::SolverMode,
  functions::make_type_check_limits::make_type_check_limits,
  records::{
    build_queue_work_state::{BuildQueueWorkState, Task},
    frontend::Frontend,
    frontend_options::FrontendOptions,
  },
  type_aliases::{frontend_callbacks::TaskQueue, module_name_type::ModuleName},
};
fn task_to_fn(task: Task) -> Box<dyn Fn()> {
  let cell = Cell::new(Some(task));
  Box::new(move || {
    if let Some(t) = cell.take() {
      t();
    }
  })
}

/// Wraps the caller-provided (non-`Send`) executor so it can populate the
/// `Send + Sync` `execute_tasks` slot. The default single-threaded executor runs
/// every task immediately on the calling thread, so the `Send`/`Sync` assertion is
/// sound — tasks never actually cross a thread boundary.
struct ExecutorWrapper {
  inner: TaskQueue,
}

unsafe impl Send for ExecutorWrapper {}
unsafe impl Sync for ExecutorWrapper {}

impl ExecutorWrapper {
  fn run(&self, tasks: Vec<Task>) {
    let adapted: Vec<Box<dyn Fn()>> = tasks.into_iter().map(task_to_fn).collect();
    (self.inner)(adapted);
  }
}

impl Frontend {
  pub fn check_queued_modules(
    &mut self,
    option_override: Option<FrontendOptions>,
    execute_tasks: TaskQueue,
    progress: Box<dyn Fn(usize, usize) -> bool>,
  ) -> Vec<ModuleName> {
    let mut frontend_options = option_override.unwrap_or_else(|| self.options.clone());
    if self.get_luau_solver_mode() == SolverMode::New {
      frontend_options.for_autocomplete = false;
    }

    // By taking data into locals, the queue is cleared at the end even on an ICE.
    let curr_module_queue: Vec<ModuleName> = take(&mut self.module_queue);

    let mut seen: DenseHashSet<ModuleName> = DenseHashSet::new(ModuleName::default());

    let state = Arc::new(BuildQueueWorkState {
      execute_task_deprecated: None,
      execute_tasks: None,
      build_queue_items: Vec::new(),
      mtx: Mutex::new(()),
      cv: Condvar::new(),
      ready_queue_items: Vec::new(),
      processing: 0,
      remaining: 0,
    });

    // SAFETY: `state` is uniquely owned here until tasks are dispatched, and the
    // default executor runs synchronously; the C++ relies on `shared_ptr` + `mtx`
    // for the same in-place mutation.
    let state_ptr = Arc::as_ptr(&state) as *mut BuildQueueWorkState;

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

      {
        let bqi = unsafe { &mut (*state_ptr).build_queue_items };
        self.add_build_queue_items(bqi, &queue, cycle_detected, &mut seen, &frontend_options);
      }
    }

    {
      let bqi = unsafe { &(*state_ptr).build_queue_items };
      if bqi.is_empty() {
        return Vec::new();
      }
    }

    // Mapping from modules to build queue slots.
    let mut module_name_to_queue: HashMap<ModuleName, usize> = HashMap::new();
    {
      let bqi = unsafe { &(*state_ptr).build_queue_items };
      for (i, item) in bqi.iter().enumerate() {
        module_name_to_queue.insert(item.name.clone(), i);
      }
    }

    // Wire `execute_tasks` into the work state. C++ defaults a null executor to a
    // single-threaded immediate runner.
    let executor = ExecutorWrapper {
      inner: execute_tasks,
    };
    unsafe {
      (*state_ptr).execute_tasks = Some(Box::new(move |tasks: Vec<Task>| {
        executor.run(tasks);
      }));
    }
    {
      let len = {
        let bqi = unsafe { &(*state_ptr).build_queue_items };
        bqi.len()
      };
      unsafe {
        (*state_ptr).remaining = len;
      }
    }

    // Record dependencies between modules.
    {
      let count = {
        let bqi = unsafe { &(*state_ptr).build_queue_items };
        bqi.len()
      };
      for i in 0..count {
        let deps: Vec<ModuleName> = {
          let bqi = unsafe { &(*state_ptr).build_queue_items };
          bqi[i].source_node.require_set.iter().cloned().collect()
        };

        for dep in deps {
          if let Some(node) = self.source_nodes.get(&dep)
            && node.has_dirty_module(frontend_options.for_autocomplete)
          {
            let dep_pos = module_name_to_queue[&dep];
            let bqi = unsafe { &mut (*state_ptr).build_queue_items };
            bqi[i].dirty_dependencies += 1;
            bqi[dep_pos].reverse_deps.push(i);
          }
        }
      }
    }

    let mut next_items: Vec<usize> = Vec::new();

    // First pass: check all modules with no pending dependencies.
    {
      let bqi = unsafe { &(*state_ptr).build_queue_items };
      for (i, item) in bqi.iter().enumerate() {
        if item.dirty_dependencies == 0 {
          next_items.push(i);
        }
      }
    }

    if !next_items.is_empty() {
      self.send_queue_item_tasks(state.clone(), take(&mut next_items));
    }

    // If not a single item was found, a cycle in the graph was hit.
    if unsafe { (*state_ptr).processing } == 0 {
      self.send_queue_cycle_item_task(state.clone());
    }

    let mut item_with_exception: Option<usize> = None;
    let mut cancelled = false;

    while unsafe { (*state_ptr).remaining } != 0 {
      {
        let mtx = unsafe { &(*state_ptr).mtx };
        let cv = unsafe { &(*state_ptr).cv };
        let guard = mtx.lock().unwrap();

        // If nothing is ready yet, wait.
        let _guard = cv
          .wait_while(guard, |_| {
            let ready = unsafe { &(*state_ptr).ready_queue_items };
            ready.is_empty()
          })
          .unwrap();

        // Handle checked items.
        let ready: Vec<usize> = {
          let r = unsafe { &(*state_ptr).ready_queue_items };
          r.clone()
        };
        for i in ready.iter().copied() {
          let (has_exception, is_cancelled) = {
            let bqi = unsafe { &(*state_ptr).build_queue_items };
            (bqi[i].exception.is_some(), bqi[i].module.cancelled)
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

          {
            let bqi = unsafe { &(*state_ptr).build_queue_items };
            let item_ref = &bqi[i];
            self.record_item_result(item_ref);
          }

          // Notify items waiting on this dependency.
          let reverse_deps: Vec<usize> = {
            let bqi = unsafe { &(*state_ptr).build_queue_items };
            bqi[i].reverse_deps.clone()
          };
          for reverse_dep in reverse_deps {
            let bqi = unsafe { &mut (*state_ptr).build_queue_items };
            LUAU_ASSERT!(bqi[reverse_dep].dirty_dependencies != 0);
            bqi[reverse_dep].dirty_dependencies -= 1;

            if !bqi[reverse_dep].processing && bqi[reverse_dep].dirty_dependencies == 0 {
              next_items.push(reverse_dep);
            }
          }
        }

        {
          let ready_len = {
            let r = unsafe { &(*state_ptr).ready_queue_items };
            r.len()
          };
          unsafe {
            LUAU_ASSERT!((*state_ptr).processing >= ready_len);
            (*state_ptr).processing -= ready_len;

            LUAU_ASSERT!((*state_ptr).remaining >= ready_len);
            (*state_ptr).remaining -= ready_len;
          }
          let r = unsafe { &mut (*state_ptr).ready_queue_items };
          r.clear();
        }
      }

      {
        let total = {
          let bqi = unsafe { &(*state_ptr).build_queue_items };
          bqi.len()
        };
        let done = total - unsafe { (*state_ptr).remaining };
        if !progress(done, total) {
          cancelled = true;
        }
      }

      // Items cannot be submitted while holding the lock.
      if !next_items.is_empty() {
        self.send_queue_item_tasks(state.clone(), take(&mut next_items));
      }

      if unsafe { (*state_ptr).processing } == 0 {
        // Typechecking might have been cancelled by user; don't return partial results.
        if cancelled {
          return Vec::new();
        }

        // We might have stopped because of a pending exception.
        if let Some(idx) = item_with_exception {
          let bqi = unsafe { &(*state_ptr).build_queue_items };
          let item_ref = &bqi[idx];
          self.record_item_result(item_ref);
        }
      }

      // If we aren't done but have nothing processing, we hit a cycle.
      if unsafe { (*state_ptr).remaining } != 0 && unsafe { (*state_ptr).processing } == 0 {
        self.send_queue_cycle_item_task(state.clone());
      }
    }

    let mut checked_modules: Vec<ModuleName> = Vec::new();
    {
      let count = {
        let bqi = unsafe { &(*state_ptr).build_queue_items };
        bqi.len()
      };
      checked_modules.reserve(count);
      for i in 0..count {
        let bqi = unsafe { &mut (*state_ptr).build_queue_items };
        checked_modules.push(take(&mut bqi[i].name));
      }
    }

    checked_modules
  }
}
