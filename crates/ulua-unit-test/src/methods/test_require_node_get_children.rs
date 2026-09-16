use alloc::{boxed::Box, string::String, vec::Vec};

use ulua_analysis::records::{require_alias::RequireAlias, require_node::RequireNode};

use crate::records::test_require_node::TestRequireNode;
impl TestRequireNode {
  pub fn get_children(&self) -> Vec<Box<dyn RequireNode>> {
    unsafe {
      let all_sources = &*self.all_sources;
      let mut result: Vec<Box<dyn RequireNode>> = Vec::new();

      for entry_key in all_sources.keys() {
        let entry_mod = entry_key.as_str();
        let module_name = self.module_name.as_str();

        // C++: entry.first.substr(0, module_name.size()) == module_name && entry.first.size() > module_name.size() &&
        //      entry.first[module_name.size()] == '/' && entry.first.find('/', module_name.size() + 1) == std::string::npos
        if entry_mod.len() > module_name.len()
          && entry_mod.starts_with(module_name)
          && entry_mod.as_bytes()[module_name.len()] == b'/'
          && entry_mod[module_name.len() + 1..].find('/').is_none()
        {
          let mut node = TestRequireNode {
            module_name: entry_key.clone(),
            all_sources: self.all_sources,
          };
          node.test_require_node_test_require_node();
          result.push(Box::new(node));
        }
      }

      result
    }
  }
}

impl RequireNode for TestRequireNode {
  fn get_path_component(&self) -> String {
    self.get_path_component()
  }

  fn get_label(&self) -> String {
    self.get_label()
  }

  fn get_tags(&self) -> Vec<String> {
    Vec::new()
  }

  fn resolve_path_to_node(&self, path: &str) -> Option<Box<dyn RequireNode>> {
    self.resolve_path_to_node(path)
  }

  fn get_children(&self) -> Vec<Box<dyn RequireNode>> {
    self.get_children()
  }

  fn get_available_aliases(&self) -> Vec<RequireAlias> {
    self.get_available_aliases()
  }
}
