use alloc::{sync::Arc, vec::Vec};

use ulua_common::{fflag, macros::luau_assert::LUAU_ASSERT, records::dense_hash_set::DenseHashSet};

use crate::{
  functions::{
    get_require_cycles::get_require_cycles, make_type_check_limits::make_type_check_limits,
    shared_mut::shared_mut,
  },
  records::{
    build_queue_item::BuildQueueItem,
    frontend::{Frontend, FrontendStats},
    frontend_options::FrontendOptions,
    module::Module,
  },
  type_aliases::module_name_type::ModuleName,
};

impl Frontend {
  pub fn add_build_queue_items(
    &mut self,
    items: &mut Vec<BuildQueueItem>,
    build_queue: &Vec<ModuleName>,
    cycle_detected: bool,
    seen: &mut DenseHashSet<ModuleName>,
    frontend_options: &FrontendOptions,
  ) {
    for module_name in build_queue {
      if seen.contains(module_name) {
        continue;
      }
      seen.insert(module_name.clone());

      LUAU_ASSERT!(self.source_nodes.contains_key(module_name));
      let source_node = self
        .source_nodes
        .get(module_name)
        .expect("紧邻上方 contains_key 断言蕴含取回必命中")
        .clone();

      if !source_node.has_dirty_module(frontend_options.for_autocomplete) {
        continue;
      }

      LUAU_ASSERT!(self.source_modules.contains_key(module_name));
      let source_module = self
        .source_modules
        .get(module_name)
        .expect("紧邻上方 contains_key 断言蕴含取回必命中")
        .clone();

      let human_readable_name = self
        .file_resolver_ref()
        .get_human_readable_module_name(module_name);

      let limits = make_type_check_limits(frontend_options);
      // 配置读取收敛于 `config_resolver_ref` chokepoint（`Box<dyn ConfigResolver>`
      // 独占、借用直出，零 unsafe），立即 `.clone()` 为拥有值以释放对 `self` 的借用。
      let config = self
        .config_resolver_ref()
        .get_config(module_name, &limits)
        .clone();

      let environment_scope = self.get_module_environment(
        source_module.as_ref(),
        &config,
        frontend_options.for_autocomplete,
      );

      let mut require_cycles = Vec::new();
      if cycle_detected {
        require_cycles = get_require_cycles(&self.source_nodes, source_node.as_ref());
      }

      {
        // Safety: `source_module` 是本循环从 `self.source_modules` clone 出的 `Arc<SourceModule>`，
        // 使用期内一直存活；`shared_mut` 依约定为该保活 Arc 派生写入句柄，仅在单线程独占写时使用。
        // 此刻无人持有对同一 SourceModule 的其它借用（`get_require_cycles` 只读 source_node），
        // 仅置一个 `bool` 字段，随后 source_module 移入 BuildQueueItem 继续被 Arc 保活。
        let source_module_mut = shared_mut(&source_module);
        source_module_mut.cyclic = !require_cycles.is_empty();
      }

      items.push(BuildQueueItem {
        name: module_name.clone(),
        human_readable_name,
        source_node,
        source_module,
        config,
        environment_scope,
        require_cycles,
        options: frontend_options.clone(),
        record_json_log: fflag::DebugLuauLogSolverToJson.get(),
        reverse_deps: Vec::new(),
        dirty_dependencies: 0,
        processing: false,
        module: Arc::new(Module::default()),
        stats: FrontendStats::default(),
      });
    }
  }
}
