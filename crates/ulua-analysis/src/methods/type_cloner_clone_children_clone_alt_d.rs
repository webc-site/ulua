use crate::{records::type_cloner::TypeCloner, type_aliases::error_type::ErrorType};

impl TypeCloner {
  pub fn clone_children_error_type(&mut self, _t: *mut ErrorType) {
    // noop
  }
}
