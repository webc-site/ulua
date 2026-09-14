use crate::{records::frontend::Frontend, type_aliases::module_name_type::ModuleName};

impl Frontend {
  pub fn is_dirty(&self, name: &ModuleName, for_autocomplete: bool) -> bool {
    match self.source_nodes.get(name) {
      Some(node) => node.has_dirty_module(for_autocomplete),
      None => true,
    }
  }
}
