use alloc::{string::String, vec::Vec};

use crate::records::require_node::RequireNode;

impl dyn RequireNode {
  pub fn get_tags(&self) -> Vec<String> {
    Vec::new()
  }
}
