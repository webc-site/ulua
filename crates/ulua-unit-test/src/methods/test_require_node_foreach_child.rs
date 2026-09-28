use alloc::{rc::Rc, string::String, vec::Vec};

use ulua_analysis::records::{require_alias::RequireAlias, require_node::RequireNode};

use crate::records::test_require_node::TestRequireNode;

impl TestRequireNode {
  /// C++ `TestRequireNode::getChildren`（`Fixture.cpp:122-134`）：取 `source` 表中
  /// 直接位于本节点之下的模块（同前缀、其后紧跟 `/` 且再无 `/`），逐个交给 `visit`。
  /// 子节点在本函数的栈帧上构造，不再有 cpp 版的 `new RequireNode`。
  ///
  /// `&mut dyn FnMut(&dyn RequireNode)` 照抄 ulua-analysis 的 `RequireNode` trait
  /// 签名（固有方法与 trait impl 共用同一形态，trait impl 才能直接转发）；
  /// 该 trait 面向运行期开放的实现方集合，测试替身只能照签实现。
  pub fn foreach_child(&self, visit: &mut dyn FnMut(&dyn RequireNode)) {
    let module_name = self.module_name.as_str();

    for child_name in self.sources.names().into_iter().filter(|entry| {
      entry.len() > module_name.len()
        && entry.starts_with(module_name)
        && entry.as_bytes()[module_name.len()] == b'/'
        && entry[module_name.len() + 1..].find('/').is_none()
    }) {
      let child = TestRequireNode {
        module_name: child_name,
        sources: Rc::clone(&self.sources),
      };
      visit(&child);
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

  fn get_available_aliases(&self) -> Vec<RequireAlias> {
    self.get_available_aliases()
  }

  fn foreach_child(&self, visit: &mut dyn FnMut(&dyn RequireNode)) {
    self.foreach_child(visit);
  }

  fn resolve_path_to_node(&self, path: &str, visit: &mut dyn FnMut(&dyn RequireNode)) -> bool {
    self.resolve_path_to_node(path, visit)
  }
}
