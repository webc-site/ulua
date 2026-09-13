use crate::records::frontend::Frontend;

impl Frontend {
  pub fn clear(&mut self) {
    self.source_nodes.clear();
    self.source_modules.clear();
    self.module_resolver.clear_modules();
    self.module_resolver_for_autocomplete.clear_modules();
    self.require_trace.clear();
  }
}
