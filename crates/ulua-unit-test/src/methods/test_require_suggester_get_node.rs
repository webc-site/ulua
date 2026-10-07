use alloc::vec::Vec;

use ulua_analysis::{
  records::{require_node::RequireNode, require_suggester::RequireSuggester},
  type_aliases::module_name_type::ModuleName,
};

use crate::{
  functions::test_require_node::test_require_node,
  records::test_require_suggester::TestRequireSuggester,
};

impl TestRequireSuggester {
  /// cpp `TestRequireSuggester::getNode`（`Fixture.cpp:147-150`）：恒给出持有全量
  /// source 表的节点（cpp 直接 `make_unique`，不校验模块是否存在）。宿主表已析构
  /// 时 `Weak::upgrade` 失败，返回 `None`（cpp 同处是悬垂指针解引用）。
  pub fn get_node(&self, name: &ModuleName) -> Option<RequireNode> {
    self.sources.upgrade()?;
    Some(test_require_node(name))
  }
}

/// `RequireSuggester` 的三个查询共用一个 impl 块，实现体按方法拆在本目录
/// `test_require_suggester_*.rs` 的固有方法上（与 `TestFileResolver` 同法）。
impl RequireSuggester for TestRequireSuggester {
  fn get_node(&self, name: &ModuleName) -> Option<RequireNode> {
    Self::get_node(self, name)
  }

  fn get_children(&self, node: &RequireNode) -> Vec<RequireNode> {
    Self::get_children(self, node)
  }

  fn resolve_path_to_node(&self, node: &RequireNode, path: &str) -> Option<RequireNode> {
    Self::resolve_path_to_node(self, node, path)
  }
}
