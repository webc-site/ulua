use alloc::{rc::Rc, vec::Vec};

use ulua_analysis::{
  records::require_node::RequireNode, type_aliases::module_name_type::ModuleName,
};
use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::{
  functions::split_string_by_slashes::split_string_by_slashes,
  records::test_require_node::TestRequireNode,
};

impl TestRequireNode {
  /// C++ `TestRequireNode::resolvePathToNode`（`Fixture.cpp:84-120`）：把 `./`、
  /// `../` 相对路径按本节点模块名归一化，命中 `source` 表才给出子节点。
  /// 命中时子节点在栈上构造并交给 `visit`，返回 `true`（cpp 的 `nullptr` 返回
  /// 对应此处的 `false`）。
  ///
  /// `&mut dyn FnMut(&dyn RequireNode)` 照抄 ulua-analysis 的 `RequireNode` trait
  /// 签名（固有方法与 trait impl 共用同一形态，trait impl 才能直接转发）；
  /// 该 trait 面向运行期开放的实现方集合，测试替身只能照签实现。
  pub fn resolve_path_to_node(&self, path: &str, visit: &mut dyn FnMut(&dyn RequireNode)) -> bool {
    let components = split_string_by_slashes(path);
    LUAU_ASSERT!((components.is_empty() || components[0] == "." || components[0] == ".."));

    let mut normalized_components: Vec<&str> = split_string_by_slashes(self.module_name.as_str());
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

    let module_name = normalized_components.join("/");
    if !self.sources.contains(&module_name) {
      return false;
    }

    let node = TestRequireNode {
      module_name: ModuleName::from(module_name),
      sources: Rc::clone(&self.sources),
    };
    visit(&node);
    true
  }
}
