use alloc::{sync::Arc, vec::Vec};

use ulua_common::{fflag, macros::luau_assert::LUAU_ASSERT, records::dense_hash_set::DenseHashSet};

use crate::{
  functions::{
    arc_as_mut::arc_as_mut, get_require_cycles::get_require_cycles,
    make_type_check_limits::make_type_check_limits,
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
      let config = unsafe {
        // Safety: `self.config_resolver` 的非空解引用收敛于 `config_resolver_ref`
        // chokepoint（构造期接线的非空 `NonNull<ConfigResolver>`，比本 Frontend
        // 长寿，此处仅读 `get_config` 槽位）。`get_config` 是静态无捕获
        // 的 `unsafe fn`，以 `expect` 兜住 `None`；调用按 C++ 虚 `getConfig` ABI 传入 resolver 自身
        // 基址作 `this` 及存活的 name/limits 借用地址。返回的 `*const Config` 指向本次语句内有效的
        // 配置对象，立即 `.clone()` 为拥有值。全程单线程，无并发别名。
        let resolver = self.config_resolver_ref();
        let get_config = resolver
          .get_config
          .expect("ConfigResolver::getConfig is not set");
        (*get_config(self.config_resolver.as_ptr(), module_name, &limits)).clone()
      };

      let environment_scope = self.get_module_environment(
        source_module.as_ref(),
        &config,
        frontend_options.for_autocomplete,
      );

      let mut require_cycles = Vec::new();
      if cycle_detected {
        require_cycles = get_require_cycles(&self.source_nodes, source_node.as_ref());
      }

      unsafe {
        // Safety: `source_module` 是本循环从 `self.source_modules` clone 出的 `Arc<SourceModule>`，
        // 使用期内一直存活；`arc_as_mut` 依约定为该保活 Arc 派生写入句柄，仅在单线程独占写时使用。
        // 此刻无人持有对同一 SourceModule 的其它借用（`get_require_cycles` 只读 source_node），
        // 仅置一个 `bool` 字段，随后 source_module 移入 BuildQueueItem 继续被 Arc 保活。
        let source_module_mut = arc_as_mut(&source_module);
        (*source_module_mut).cyclic = !require_cycles.is_empty();
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
