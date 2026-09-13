use alloc::sync::Arc;

use ulua_common::macros::{
  luau_timetrace_argument::LUAU_TIMETRACE_ARGUMENT, luau_timetrace_scope::LUAU_TIMETRACE_SCOPE,
};

use crate::records::{
  build_queue_item::BuildQueueItem, frontend::Frontend, source_node::SourceNode,
};

impl Frontend {
  pub fn record_item_result(&mut self, item: &BuildQueueItem) {
    let for_autocomplete = item.options.for_autocomplete;
    let source_node = Arc::as_ptr(&item.source_node) as *mut SourceNode;

    let replaced = if for_autocomplete {
      let replaced = self
        .module_resolver_for_autocomplete
        .set_module(&item.name, item.module.clone());
      unsafe {
        (*source_node).dirty_module_for_autocomplete = false;
      }
      replaced
    } else {
      let replaced = self
        .module_resolver
        .set_module(&item.name, item.module.clone());
      unsafe {
        (*source_node).dirty_module = false;
      }
      replaced
    };

    if replaced {
      LUAU_TIMETRACE_SCOPE!("Frontend::invalidateDependentModules", "Frontend");
      LUAU_TIMETRACE_ARGUMENT!("name", item.name.as_str());
      self.traverse_dependents(
        &item.name,
        Box::new(move |source_node: &mut SourceNode| {
          let traverse_subtree = !source_node.has_invalid_module_dependency(for_autocomplete);
          source_node.set_invalid_module_dependency(true, for_autocomplete);
          traverse_subtree
        }),
      );
    }

    unsafe {
      (*source_node).set_invalid_module_dependency(false, for_autocomplete);
    }

    self.stats.time_check += item.stats.time_check;
    self.stats.time_lint += item.stats.time_lint;
    self.stats.files_strict += item.stats.files_strict;
    self.stats.files_nonstrict += item.stats.files_nonstrict;
    self.stats.types_allocated += item.stats.types_allocated;
    self.stats.type_packs_allocated += item.stats.type_packs_allocated;
    self.stats.bool_singletons_minted += item.stats.bool_singletons_minted;
    self.stats.str_singletons_minted += item.stats.str_singletons_minted;
    self.stats.unique_str_singletons_minted += item.stats.unique_str_singletons_minted;
    self.stats.dynamic_constraints_created += item.stats.dynamic_constraints_created;
  }
}
