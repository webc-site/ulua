use alloc::string::String;

use crate::{functions::get_node_name::get_node_name, records::test_require_node::TestRequireNode};

impl TestRequireNode {
  pub fn get_path_component(&self) -> String {
    get_node_name(self)
  }
}
