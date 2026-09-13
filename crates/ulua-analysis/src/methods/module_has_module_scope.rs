use crate::records::module::Module;

impl Module {
  pub fn has_module_scope(&self) -> bool {
    !self.scopes.is_empty()
  }
}
