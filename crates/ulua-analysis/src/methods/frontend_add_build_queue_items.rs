use alloc::{sync::Arc, vec::Vec};

use ulua_common::{FFlag, macros::luau_assert::LUAU_ASSERT, records::dense_hash_set::DenseHashSet};

use crate::{
  functions::{
    get_require_cycles::get_require_cycles, make_type_check_limits::make_type_check_limits,
  },
  records::{
    build_queue_item::BuildQueueItem,
    file_resolver::FileResolver,
    frontend::{Frontend, FrontendStats},
    frontend_options::FrontendOptions,
    module::Module,
    source_module::SourceModule,
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
      let source_node = self.source_nodes.get(module_name).unwrap().clone();

      if !source_node.has_dirty_module(frontend_options.for_autocomplete) {
        continue;
      }

      LUAU_ASSERT!(self.source_modules.contains_key(module_name));
      let source_module = self.source_modules.get(module_name).unwrap().clone();

      let human_readable_name =
        unsafe { FileResolver::get_human_readable_module_name(self.file_resolver, module_name) };

      let limits = make_type_check_limits(frontend_options);
      let config = unsafe {
        let get_config = (*self.config_resolver)
          .get_config
          .expect("ConfigResolver::getConfig is not set");
        (*get_config(self.config_resolver, module_name, &limits)).clone()
      };

      let environment_scope = self.get_module_environment(
        source_module.as_ref(),
        &config,
        frontend_options.for_autocomplete,
      );

      let mut require_cycles = Vec::new();
      if cycle_detected {
        require_cycles = unsafe {
          get_require_cycles(
            &*self.file_resolver,
            &self.source_nodes,
            source_node.as_ref(),
          )
        };
      }

      unsafe {
        let source_module_mut = Arc::as_ptr(&source_module) as *mut SourceModule;
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
        record_json_log: FFlag::DebugLuauLogSolverToJson.get(),
        reverse_deps: Vec::new(),
        dirty_dependencies: 0,
        processing: false,
        exception: None,
        module: Arc::new(Module::default()),
        stats: FrontendStats::default(),
      });
    }
  }
}
