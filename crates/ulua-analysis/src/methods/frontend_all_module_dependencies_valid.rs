use crate::{records::frontend::Frontend, type_aliases::module_name_type::ModuleName};

impl Frontend {
  pub fn all_module_dependencies_valid(&self, name: &ModuleName, for_autocomplete: bool) -> bool {
    if let Some(node) = self.source_nodes.get(name) {
      !node.has_invalid_module_dependency(for_autocomplete)
    } else {
      false
    }
  }
}
