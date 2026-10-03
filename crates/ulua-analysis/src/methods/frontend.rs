//! `frontend` 方法汇总：原先按 cpp 符号逐方法拆分的同前缀小文件合并至此，行为逐字保留。

use alloc::{rc::Rc, string::String, sync::Arc, vec::Vec};
use core::{ptr::null, sync::atomic::Ordering};
use std::slice::from_ref;

use ulua_ast::methods::ast_stat_block_visit::ast_stat_block_visit;
use ulua_common::macros::{
  luau_assert::LUAU_ASSERT,
  luau_timetrace_scope::{LUAU_TIMETRACE_ARGUMENT, LUAU_TIMETRACE_SCOPE},
};
use ulua_config::records::{config::Config, lint_warning::LintWarning};

use crate::{
  enums::solver_mode::SolverMode,
  functions::{freeze::freeze, shared_mut::shared_mut, unfreeze::unfreeze},
  records::{
    arena_handle::{Handle, alias, alias_ref},
    build_queue_item::BuildQueueItem,
    build_queue_work_state::{BuildQueueWorkState, Task},
    expected_type_visitor::ExpectedTypeVisitor,
    frontend::Frontend,
    lint_result::LintResult,
    module::Module,
    scope::Scope,
    scope_registry::register_scope,
    source_module::SourceModule,
    source_node::SourceNode,
  },
  type_aliases::{
    frontend_callbacks::TaskQueue, module_name_type::ModuleName, scope_ptr_type::ScopePtr,
  },
};

impl Frontend {
  pub fn add_environment(&mut self, environment_name: String) -> ScopePtr {
    if let Some(scope) = self.environments.get(&environment_name) {
      return scope.clone();
    }
    let scope = ScopePtr::new(Scope::new(&self.globals.global_scope, 0));
    register_scope(&scope);
    self.environments.insert(environment_name, scope.clone());
    scope
  }
}

impl Frontend {
  pub fn all_module_dependencies_valid(&self, name: &ModuleName, for_autocomplete: bool) -> bool {
    if let Some(node) = self.source_nodes.get(name) {
      !node.has_invalid_module_dependency(for_autocomplete)
    } else {
      false
    }
  }
}

impl Frontend {
  pub fn check_build_queue_items(&mut self, items: &mut [BuildQueueItem]) {
    for item in items.iter_mut() {
      self.check_build_queue_item(item);

      if item.module.cancelled {
        break;
      }

      self.record_item_result(item);
    }
  }
}

impl Frontend {
  pub fn classify_lints(&self, warnings: &[LintWarning], config: &Config) -> LintResult {
    let mut result = LintResult::default();

    for w in warnings.iter() {
      let should_error = config.lint_errors || config.fatal_lint.is_enabled(w.code);
      if should_error {
        result.errors.push(w.clone());
      } else {
        result.warnings.push(w.clone());
      }
    }

    result
  }
}

impl Frontend {
  pub fn clear(&mut self) {
    self.source_nodes.clear();
    self.source_modules.clear();
    self.module_resolver.clear_modules();
    self.module_resolver_for_autocomplete.clear_modules();
    self.require_trace.clear();
  }
}

impl Frontend {
  pub fn clear_stats(&mut self) {
    self.stats = Default::default();
  }
}

impl Frontend {
  pub fn get_environment_scope(&self, environment_name: String) -> ScopePtr {
    if let Some(scope) = self.environments.get(&environment_name) {
      return scope.clone();
    }

    LUAU_ASSERT!(false, "environment doesn't exist");
    let scope = Arc::new(Scope::scope_type_pack_id(null()));
    register_scope(&scope);
    scope
  }
}

impl Frontend {
  pub fn get_luau_solver_mode(&self) -> SolverMode {
    match self.use_new_luau_solver.load(Ordering::Relaxed) {
      x if x == SolverMode::Old as i32 => SolverMode::Old,
      _ => SolverMode::New,
    }
  }
}

// cpp `SourceModule* Frontend::getSourceModule(const ModuleName&)`
// （`Analysis/src/Frontend.cpp`）的 Rust 形态。
//
// 原实现两版都以裸指针返回、用 `null_mut()` 当「模块不存在」哨兵，且只读版
// 还要先把 `&self` 转铸成 `*mut Frontend` 才能复用可变版（同一对象的共享
// 借用被当作独占借用使用，属指针别名 UB 窗口）。本次收口（步⑤ 可变版 +
// 挂账② 只读版）：
//
// - 两版统一返回 [`Option<Handle<SourceModule>>`]——`Option` 承载可空性，
// `Handle` 承载「宿主表内实例的别名句柄」语义（契约见 `records/arena_handle.rs`），
// 调用点不再触碰裸指针、null 哨兵与非空证明。

impl Frontend {
  /// 可变形态：`source_modules` 表内该模块的别名句柄，模块未检查/未保留时为
  /// `None`（取代原 `null_mut()` 哨兵）。
  ///
  /// 句柄不绑定 `&mut self` 生命周期，与原裸指针返回值的借用检查行为同构
  /// （否则调用点 `frontend.module_resolver.get_module(..)` 一类后续访问无法
  /// 与已取出的模块共存），解引用契约集中在 `records/arena_handle.rs`。
  /// 指针直接取自 `Arc::as_ptr`（非经共享借用转化），保持其堆分配的原始 provenance。
  pub fn get_source_module_mut(
    &mut self,
    module_name: &ModuleName,
  ) -> Option<Handle<SourceModule>> {
    self
      .source_modules
      .get_mut(module_name)
      .map(|module| unsafe { Handle::from_raw(Arc::as_ptr(module) as *mut SourceModule) })
  }

  /// 只读形态（cpp 同一函数的 `const` 用法）：与可变版同一 `Option<Handle>`
  /// 交付——`Handle` 派生自共享引用，下游只读消费（`get`）；需要写穿语义的
  /// 调用点走 [`Frontend::get_source_module_mut`]。
  pub fn get_source_module(&self, module_name: &ModuleName) -> Option<Handle<SourceModule>> {
    self
      .source_modules
      .get(module_name)
      .map(|module| Handle::from_ref(&**module))
  }
}

impl Frontend {
  pub fn is_dirty(&self, name: &ModuleName, for_autocomplete: bool) -> bool {
    match self.source_nodes.get(name) {
      Some(node) => node.has_dirty_module(for_autocomplete),
      None => true,
    }
  }
}

impl Frontend {
  pub fn mark_dirty(&mut self, name: &ModuleName, marked_dirty: Option<&mut Vec<ModuleName>>) {
    LUAU_TIMETRACE_SCOPE!("Frontend::markDirty", "Frontend");
    LUAU_TIMETRACE_ARGUMENT!("name", name.as_str());

    let marked_dirty_ptr = marked_dirty.map(|v| v as *mut Vec<ModuleName>);

    self.traverse_dependents(name, move |source_node: &mut SourceNode| {
      if let Some(marked_dirty) = marked_dirty_ptr {
        alias(marked_dirty).push(source_node.name.clone());
      }

      if source_node.dirty_source_module
        && source_node.dirty_module
        && source_node.dirty_module_for_autocomplete
      {
        return false;
      }

      source_node.dirty_source_module = true;
      source_node.dirty_module = true;
      source_node.dirty_module_for_autocomplete = true;

      true
    });
  }
}

impl Frontend {
  /// 执行一个队列项并把下标回报到就绪队列。
  ///
  /// C++: `Frontend::performQueueItemTask(state, itemPos)`。
  pub fn perform_queue_item_task(&mut self, state: Rc<BuildQueueWorkState>, item_pos: usize) {
    // C++ 在锁外原地改写 `state->buildQueueItems[itemPos]`，靠「每个任务只碰自己的项」
    // 规避竞争；这里队列项与同步状态同属一个 `Mutex`，检查期间持锁即可。
    // `check_build_queue_item` 不会回到队列（既不发任务也不读就绪队列），无重入死锁。
    {
      let mut queue = state.lock();
      self.check_build_queue_item(&mut queue.build_queue_items[item_pos]);
    }

    // C++: `try { checkBuildQueueItem(item); } catch (const InternalCompilerError&) {
    //   item.exception = std::current_exception(); }`
    // Rust 端口把 ICE 建模为 panic（与仓库其余部分一致），这里不捕获：
    // panic 直接 unwind 出 `check_queued_modules`，故 cpp 的 presence flag
    // （`item.exception`）无对应物且已删除。
    {
      let mut queue = state.lock();
      queue.ready_queue_items.push(item_pos);
    }

    state.notify_task_ready();
  }
}

impl Frontend {
  /// 前置契约（本函数体经 safe 门面完成指针借用，无 unsafe 操作；以下为文档约定）
  /// 调用方须保证满足 C++ 原实现的调用契约。
  pub(crate) fn populate_expected_types(
    &self,
    source_module: &SourceModule,
    module: *mut Module,
    root_scope: &ScopePtr,
  ) {
    let was_frozen = alias_ref(module).internal_types.types.is_frozen()
      || alias_ref(module).internal_types.type_packs.is_frozen();
    if was_frozen {
      unfreeze(&mut alias(module).internal_types);
    }

    let mut visitor = ExpectedTypeVisitor::new(
      &mut alias(module).ast_types,
      &mut alias(module).ast_expected_types,
      &mut alias(module).ast_resolved_types,
      &mut alias(module).ast_overload_resolved_types,
      Handle::from_mut(&mut alias(module).internal_types),
      self.builtin_types_handle(),
      shared_mut(root_scope),
    );

    // `source_module.root` 句柄化：物化只读遍历起点前先断言在场
    // （cpp `visit(sourceModule.root)` 的非空调用契约）。
    let root = source_module
      .root
      .expect("populateExpectedTypes: 根块应在场（cpp 直取 sourceModule.root）");
    ast_stat_block_visit(root.get_mut(), &mut visitor);

    if was_frozen {
      freeze(&mut alias(module).internal_types);
    }
  }
}

impl Frontend {
  pub fn queue_module_check_vector_module_name(&mut self, names: &[ModuleName]) {
    self.module_queue.extend_from_slice(names);
  }

  pub fn queue_module_check_module_name(&mut self, _name: &ModuleName) {
    self.queue_module_check_vector_module_name(from_ref(_name));
  }
}

impl Frontend {
  /// 图上存在环时，随便挑一个还没处理的项先跑起来。
  ///
  /// C++: `Frontend::sendQueueCycleItemTask(state)`。
  pub fn send_queue_cycle_item_task(
    &mut self,
    state: Rc<BuildQueueWorkState>,
    execute_tasks: &mut TaskQueue,
  ) {
    // 语句结束即释放锁，`send_queue_item_tasks` 里会重新加锁。
    let pending = state
      .lock()
      .build_queue_items
      .iter()
      .position(|item| !item.processing);

    if let Some(item_pos) = pending {
      self.send_queue_item_tasks(state, Vec::from([item_pos]), execute_tasks);
    }
  }
}

impl Frontend {
  /// 标记这些队列项为处理中，并把它们交给调用方的派发方。
  ///
  /// C++: `Frontend::sendQueueItemTasks(state, items)` —— cpp 在这里把
  /// `[this, state, itemPos] { performQueueItemTask(state, itemPos); }` 打包成
  /// `std::function<void()>` 交给 `state->executeTasks`。Rust 端口投递的是下标，
  /// 执行器（`run`）由本方法提供，只能在当前持有 `&mut Frontend` 的栈帧内被调用。
  pub fn send_queue_item_tasks(
    &mut self,
    state: Rc<BuildQueueWorkState>,
    items: Vec<usize>,
    execute_tasks: &mut TaskQueue,
  ) {
    let mut tasks: Vec<Task> = Vec::with_capacity(items.len());

    {
      let mut queue = state.lock();

      for &item_pos in &items {
        let item = &mut queue.build_queue_items[item_pos];

        LUAU_ASSERT!(!item.processing);
        item.processing = true;

        tasks.push(item_pos);
      }

      queue.processing += items.len();
    }

    // 锁必须在派发前释放：`run` 会进入 `perform_queue_item_task`，后者自己加锁。
    execute_tasks(tasks, &mut |task| {
      self.perform_queue_item_task(state.clone(), task)
    });
  }
}

impl Frontend {
  pub fn set_luau_solver_mode(&mut self, mode: SolverMode) {
    self
      .use_new_luau_solver
      .store(mode as i32, Ordering::Relaxed);
  }
}

impl Frontend {
  pub fn traverse_dependents(
    &mut self,
    name: &ModuleName,
    process_subtree: impl Fn(&mut SourceNode) -> bool,
  ) {
    LUAU_TIMETRACE_SCOPE!("Frontend::traverseDependents", "Frontend");

    if !self.source_nodes.contains_key(name) {
      return;
    }

    let mut queue = Vec::new();
    queue.push(name.clone());

    while let Some(next) = queue.pop() {
      debug_assert!(self.source_nodes.contains_key(&next));
      let source_node_arc = match self.source_nodes.get(&next) {
        Some(v) => v,
        None => continue,
      };

      // Clone to avoid borrowing `self.source_nodes` across callback and subsequent queue mutations.
      let source_node_ptr = shared_mut(source_node_arc);

      // SAFETY: `SourceNode` is owned by `Frontend.source_nodes` as an `Arc`. We do not free it here,
      // and callback is expected to mutate the node. We avoid aliasing `&mut` borrows by using the raw pointer.
      let keep_going = { process_subtree(&mut *source_node_ptr) };

      if !keep_going {
        continue;
      }

      let dependents = { &source_node_ptr.dependents };
      for d in dependents.iter() {
        queue.push(d.clone());
      }
    }
  }
}
