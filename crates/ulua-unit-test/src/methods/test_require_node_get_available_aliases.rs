use alloc::{string::String, vec::Vec};

use ulua_analysis::records::require_alias::RequireAlias;

use crate::records::test_require_node::TestRequireNode;
impl TestRequireNode {
  pub fn get_available_aliases(&self) -> Vec<RequireAlias> {
    alloc::vec![RequireAlias::require_alias_string(String::from(
      "defaultalias"
    ))]
  }
}
