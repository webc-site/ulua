use std::slice::from_ref;

use crate::{records::frontend::Frontend, type_aliases::module_name_type::ModuleName};

impl Frontend {
  pub fn queue_module_check_module_name(&mut self, _name: &ModuleName) {
    self.queue_module_check_vector_module_name(from_ref(_name));
  }
}
