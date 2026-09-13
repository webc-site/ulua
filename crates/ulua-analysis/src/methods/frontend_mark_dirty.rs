use alloc::{boxed::Box, vec::Vec};

use ulua_common::macros::{
  luau_timetrace_argument::LUAU_TIMETRACE_ARGUMENT, luau_timetrace_scope::LUAU_TIMETRACE_SCOPE,
};

use crate::{
  records::{frontend::Frontend, source_node::SourceNode},
  type_aliases::module_name_type::ModuleName,
};

impl Frontend {
  pub fn mark_dirty(&mut self, name: &ModuleName, marked_dirty: Option<&mut Vec<ModuleName>>) {
    LUAU_TIMETRACE_SCOPE!("Frontend::markDirty", "Frontend");
    LUAU_TIMETRACE_ARGUMENT!("name", name.as_str());

    let marked_dirty_ptr = marked_dirty.map(|v| v as *mut Vec<ModuleName>);

    self.traverse_dependents(
      name,
      Box::new(move |source_node: &mut SourceNode| {
        if let Some(marked_dirty) = marked_dirty_ptr {
          unsafe {
            (*marked_dirty).push(source_node.name.clone());
          }
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
      }),
    );
  }
}
