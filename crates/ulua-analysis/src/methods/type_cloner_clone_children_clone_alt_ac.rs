use crate::{records::type_cloner::TypeCloner, type_aliases::error_type_pack::ErrorTypePack};

impl TypeCloner {
  pub fn clone_children_error_type_pack(&mut self, _t: *mut ErrorTypePack) {
    // noop.
  }
}
