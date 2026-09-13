use alloc::{string::String, sync::Arc, vec::Vec};
use core::ptr::null_mut;

use ulua_common::macros::{
  luau_timetrace_argument::LUAU_TIMETRACE_ARGUMENT, luau_timetrace_scope::LUAU_TIMETRACE_SCOPE,
};
use ulua_config::records::config::Config;

use crate::{
  functions::{get_timestamp::get_timestamp, trace_requires::trace_requires},
  records::{
    file_resolver::FileResolver, frontend::Frontend, source_module::SourceModule,
    source_node::SourceNode, type_check_limits::TypeCheckLimits,
  },
  type_aliases::module_name_type::ModuleName,
};
impl Frontend {
  pub fn get_source_node(
    &mut self,
    name: &ModuleName,
    limits: &TypeCheckLimits,
  ) -> (*mut SourceNode, *mut SourceModule) {
    let already_present = match self.source_nodes.get(name) {
      Some(node) if !node.has_dirty_source_module() => {
        let node_ptr = Arc::as_ptr(node) as *mut SourceNode;
        if let Some(module) = self.source_modules.get(name) {
          return (node_ptr, Arc::as_ptr(module) as *mut SourceModule);
        } else {
          // LUAU_ASSERT(!"Everything in sourceNodes should also be in sourceModules");
          return (node_ptr, null_mut());
        }
      }
      Some(_) => true,
      None => false,
    };

    LUAU_TIMETRACE_SCOPE!("Frontend::getSourceNode", "Frontend");
    LUAU_TIMETRACE_ARGUMENT!("name", name.as_str());

    let timestamp = get_timestamp();

    let source = unsafe { FileResolver::read_source(self.file_resolver, name) };
    let environment_name =
      unsafe { FileResolver::get_environment_for_module(self.file_resolver, name) };

    self.stats.time_read += get_timestamp() - timestamp;

    let source = match source {
      Some(source) => source,
      None => {
        self.source_modules.remove(name);
        return (null_mut(), null_mut());
      }
    };

    let mut opts = {
      let config: &Config = unsafe {
        let get_config = (*self.config_resolver)
          .get_config
          .expect("ConfigResolver::getConfig is not set");
        &*get_config(self.config_resolver, name, limits)
      };
      config.parse_options.clone()
    };
    opts.capture_comments = true;
    let mut result = self.parse_module_name_string_view_parse_options(name, &source.source, &opts);
    result.r#type = source.r#type;

    let require = unsafe { trace_requires(self.file_resolver, result.root, name.clone(), limits) };
    self.require_trace.insert(name.clone(), require.clone());

    // std::shared_ptr<SourceNode>& sourceNode = sourceNodes[name];
    // if (!sourceNode) sourceNode = std::make_shared<SourceNode>();
    let source_node_arc = self
      .source_nodes
      .entry(name.clone())
      .or_insert_with(|| Arc::new(default_source_node()));
    let source_node_ptr = Arc::as_ptr(source_node_arc) as *mut SourceNode;

    let source_module_arc = self
      .source_modules
      .entry(name.clone())
      .or_insert_with(|| Arc::new(SourceModule::new()));
    let source_module_ptr = Arc::as_ptr(source_module_arc) as *mut SourceModule;

    unsafe {
      // *sourceModule = std::move(result);
      *source_module_ptr = result;
      (*source_module_ptr).environment_name = environment_name;

      (*source_node_ptr).name = (*source_module_ptr).name.clone();
      (*source_node_ptr).human_readable_name = (*source_module_ptr).human_readable_name.clone();

      // clear all prior dependents; re-add after parsing the rest of the graph.
      // C++: `depIt->second->dependents.erase(sourceNode->name);`
      // The Rust `dependents` set has no `erase`, so we rebuild it without `self_name`
      // (behaviorally identical to a single-key erase).
      let prior_locations = (*source_node_ptr).require_locations.clone();
      let self_name = (*source_node_ptr).name.clone();
      for (module_name, _) in prior_locations.iter() {
        if let Some(dep) = self.source_nodes.get(module_name) {
          let dep_ptr = Arc::as_ptr(dep) as *mut SourceNode;
          let retained: Vec<ModuleName> = (*dep_ptr)
            .dependents
            .iter()
            .filter(|n| **n != self_name)
            .cloned()
            .collect();
          (*dep_ptr).dependents.clear();
          for n in retained {
            (*dep_ptr).dependents.insert(n);
          }
        }
      }

      (*source_node_ptr).require_set.clear();
      (*source_node_ptr).require_locations.clear();
      (*source_node_ptr).dirty_source_module = false;

      // `it == sourceNodes.end()` in the C++ corresponds to the node not having
      // existed prior to this call (a brand-new source node).
      if !already_present {
        (*source_node_ptr).dirty_module = true;
        (*source_node_ptr).dirty_module_for_autocomplete = true;
      }

      for (module_name, _) in require.require_list.iter() {
        (*source_node_ptr).require_set.insert(module_name.clone());
      }

      (*source_node_ptr).require_locations = require.require_list.clone();
    }

    (source_node_ptr, source_module_ptr)
  }
}

fn default_source_node() -> SourceNode {
  use ulua_common::records::dense_hash_set::DenseHashSet;
  SourceNode {
    name: ModuleName::default(),
    human_readable_name: String::new(),
    require_set: DenseHashSet::new(ModuleName::default()),
    require_locations: Vec::new(),
    dependents: DenseHashSet::new(ModuleName::default()),
    dirty_source_module: true,
    dirty_module: true,
    dirty_module_for_autocomplete: true,
    invalid_module_dependency: true,
    invalid_module_dependency_for_autocomplete: true,
    autocomplete_limits_mult: 1.0,
  }
}
