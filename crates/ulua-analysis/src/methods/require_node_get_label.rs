use alloc::string::String;

use crate::records::require_node::RequireNode;

impl dyn RequireNode {
  pub fn get_label(&self) -> String {
    self.get_path_component()
  }
}
