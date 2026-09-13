use crate::{records::frontend::Frontend, type_aliases::module_name_type::ModuleName};

impl Frontend {
  pub fn queue_module_check_vector_module_name(&mut self, names: &[ModuleName]) {
    self.module_queue.extend_from_slice(names);
  }
}
