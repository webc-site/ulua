use alloc::{boxed::Box, string::String, vec::Vec};

use crate::records::require_alias::RequireAlias;

pub trait RequireNode {
  fn get_path_component(&self) -> String;

  fn get_label(&self) -> String {
    self.get_path_component()
  }

  fn get_tags(&self) -> Vec<String> {
    Vec::new()
  }

  fn resolve_path_to_node(&self, path: &str) -> Option<Box<dyn RequireNode>>;

  fn get_children(&self) -> Vec<Box<dyn RequireNode>>;

  fn get_available_aliases(&self) -> Vec<RequireAlias>;
}
