use alloc::sync::Arc;

use ulua_common::macros::luau_timetrace_scope::LUAU_TIMETRACE_SCOPE;

use crate::{
  records::{frontend::Frontend, source_node::SourceNode},
  type_aliases::module_name_type::ModuleName,
};

impl Frontend {
  /// C++ `Frontend::clearModules(const std::vector<ModuleName>&)`
  /// (`Analysis/src/Frontend.cpp:2922`): mark each module dirty so every
  /// dependent is rechecked, then fully erase the requested modules from the
  /// source graph, the require trace and both module resolvers.
  pub fn clear_modules(&mut self, names: &[ModuleName]) {
    LUAU_TIMETRACE_SCOPE!("Frontend::clearModules", "Frontend");

    for name in names {
      self.mark_dirty(name, None);
    }

    for name in names {
      // C++ `if (it == sourceNodes.end()) continue;`: a module that was never
      // parsed leaves every other cache untouched.
      if self.detach_source_node(name) {
        self.erase_module_caches(name);
      }
    }
  }

  /// Drops the `SourceNode` of `name`, first unlinking it from the dependent
  /// sets of its own requirements. Returns whether a node was present.
  ///
  /// Shared by [`Frontend::clear_modules`] and the `LuauFrontendSourceNodeErase`
  /// branch of `Frontend::getSourceNode`.
  pub(crate) fn detach_source_node(&mut self, name: &ModuleName) -> bool {
    let Some(source_node) = self.source_nodes.remove(name) else {
      return false;
    };

    for dep in source_node.require_set.iter() {
      if let Some(dep_node) = self.source_nodes.get(dep) {
        let dep_ptr: *mut SourceNode = Arc::as_ptr(dep_node).cast_mut();
        // SAFETY: `dep_node` is owned by `self.source_nodes` and stays alive
        // across this statement; the same Arc-aliasing idiom as
        // `Frontend::traverse_dependents`.
        unsafe { (*dep_ptr).dependents.erase(name) };
      }
    }

    true
  }

  /// C++ erasure of the per-module caches (`sourceModules` / `requireTrace` /
  /// both module resolvers), shared by `clearModules` and `getSourceNode`.
  pub(crate) fn erase_module_caches(&mut self, name: &ModuleName) {
    self.source_modules.remove(name);
    self.require_trace.remove(name);
    self.module_resolver.erase_module(name);
    self.module_resolver_for_autocomplete.erase_module(name);
  }
}
