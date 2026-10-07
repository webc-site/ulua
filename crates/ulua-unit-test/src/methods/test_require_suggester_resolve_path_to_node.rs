use alloc::{string::String, vec::Vec};

use ulua_analysis::{
  records::require_node::RequireNode, type_aliases::module_name_type::ModuleName,
};
use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::{
  functions::{
    split_string_by_slashes::split_string_by_slashes, test_require_node::test_require_node,
  },
  records::test_require_suggester::TestRequireSuggester,
};

impl TestRequireSuggester {
  /// cpp `TestRequireNode::resolvePathToNode`（`Fixture.cpp:90-126`）：把 `./`、
  /// `../` 相对路径按本节点模块名归一化，命中 `source` 表才给出节点（cpp 的
  /// `nullptr` 返回对应此处的 `None`）。
  pub fn resolve_path_to_node(&self, node: &RequireNode, path: &str) -> Option<RequireNode> {
    let sources = self.sources.upgrade()?;

    let components = split_string_by_slashes(path);
    LUAU_ASSERT!((components.is_empty() || components[0] == "." || components[0] == ".."));

    let mut normalized_components: Vec<&str> = split_string_by_slashes(node.module_name.as_str());
    normalized_components.pop();
    LUAU_ASSERT!(!normalized_components.is_empty());

    for component in components {
      if component == ".." {
        if normalized_components.is_empty() {
          LUAU_ASSERT!(false);
        } else {
          normalized_components.pop();
        }
      } else if !component.is_empty() && component != "." {
        normalized_components.push(component);
      }
    }

    let module_name: String = normalized_components.join("/");
    if !sources.contains(&module_name) {
      return None;
    }

    Some(test_require_node(&ModuleName::from(module_name)))
  }
}
