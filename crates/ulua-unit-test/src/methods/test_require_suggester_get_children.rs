use alloc::vec::Vec;

use ulua_analysis::records::require_node::RequireNode;

use crate::{
  functions::test_require_node::test_require_node,
  records::test_require_suggester::TestRequireSuggester,
};

impl TestRequireSuggester {
  /// cpp `TestRequireNode::getChildren`（`Fixture.cpp:128-140`）：取 `source` 表中
  /// 直接位于本节点之下的模块（同前缀、其后紧跟 `/` 且再无 `/`）。
  ///
  /// 节点是具体数据（ulua-analysis `records/require_node.rs`），故这里直接返回
  /// `Vec<RequireNode>`——即 cpp `vector<unique_ptr<RequireNode>>` 去掉指针那层，
  /// 没有逐子节点的 vtable 调用，也没有原先的 `&mut dyn FnMut(&dyn RequireNode)`
  /// 访问者（review.md §4）。
  pub fn get_children(&self, node: &RequireNode) -> Vec<RequireNode> {
    let Some(sources) = self.sources.upgrade() else {
      return Vec::new();
    };
    let module_name = node.module_name.as_str();

    sources
      .names()
      .into_iter()
      .filter(|entry| {
        entry.len() > module_name.len()
          && entry.starts_with(module_name)
          && entry.as_bytes()[module_name.len()] == b'/'
          && entry[module_name.len() + 1..].find('/').is_none()
      })
      .map(|child| test_require_node(&child))
      .collect()
  }
}
