use alloc::{string::String, vec::Vec};

use crate::records::require_alias::RequireAlias;
impl RequireAlias {
  pub fn require_alias_string(alias: String) -> Self {
    Self {
      alias,
      tags: Vec::new(),
    }
  }
}
