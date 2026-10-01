use alloc::{rc::Rc, string::String, vec::Vec};

use ulua_analysis::records::{require_alias::RequireAlias, require_node::RequireNode};

use crate::records::test_require_node::TestRequireNode;

impl TestRequireNode {
  /// C++ `TestRequireNode::getChildren`（`Fixture.cpp:122-134`）：取 `source` 表中
  /// 直接位于本节点之下的模块（同前缀、其后紧跟 `/` 且再无 `/`），逐个交给 `visit`。
  /// 子节点在本函数的栈帧上构造，不再有 cpp 版的 `new RequireNode`。
  ///
  /// visitor 外层闭包用 `impl FnMut` 单态化（review.md §4：固有调用点消除 `dyn`
  /// 虚分派）；内层 `&dyn RequireNode` 保留——节点类型集合在运行期开放
  /// （`ulua-analysis` 侧真实节点与本测试替身等多实现并存，trait 边界无法收敛为
  /// 单一具体类型），trait 形参正是据此钉死 `dyn` 的。
  pub fn foreach_child(&self, visit: &mut impl FnMut(&dyn RequireNode)) {
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

  // trait 边界签名带不了 `impl` 泛型，只能照抄 trait 的 `dyn` 形参（保留理由见
  // ulua-analysis `records/require_node.rs` 模块注）；转发时以 `&mut visit` 把
  // `&mut dyn FnMut` 作为 Sized 闭包类型交给固有方法单态化。
  fn foreach_child(&self, mut visit: &mut dyn FnMut(&dyn RequireNode)) {
    self.foreach_child(&mut visit);
  }

  fn resolve_path_to_node(&self, path: &str, mut visit: &mut dyn FnMut(&dyn RequireNode)) -> bool {
    self.resolve_path_to_node(path, &mut visit)
  }
}
