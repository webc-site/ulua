use alloc::{string::String, vec::Vec};

use crate::records::require_alias::RequireAlias;

/// C++ `Luau::RequireNode`（`Analysis/include/Luau/FileResolver.h`）的 Rust 化。
///
/// cpp 侧三个产出节点的操作（`getNode` / `getChildren` / `resolvePathToNode`）都按
/// `std::unique_ptr<RequireNode>` 逐层交回所有权，此前的直译 `Box<dyn RequireNode>`
/// 意味着每枚举一个子节点、每解析一层路径都要一次堆分配 + 一次 vtable 装箱。
/// 现在改为「栈上节点 + 借用回调」：实现方在栈上构造节点，以 `&dyn RequireNode`
/// 交给访问方，节点全链路零堆分配。
///
/// `dyn` 在此保留是正当的：该接口由宿主注入（`FileResolver::require_suggester` 存
/// `Arc<dyn RequireSuggester>`），实现方位于本 crate 之外（如 ulua-unit-test 的
/// `TestRequireNode`），无法穷举成 enum。下方两个遍历方法的访问者形参同样取
/// `&mut dyn FnMut(&dyn RequireNode)`：泛形参（`impl`/泛型方法）会破坏对象安全，
/// 令 `&dyn RequireNode` 沿遍历链传递不再可行。
pub trait RequireNode {
  fn get_path_component(&self) -> String;

  fn get_label(&self) -> String {
    self.get_path_component()
  }

  fn get_tags(&self) -> Vec<String> {
    Vec::new()
  }

  fn get_available_aliases(&self) -> Vec<RequireAlias>;

  /// cpp `getChildren()`：对直接位于本节点之下的每个子节点回调一次 `visit`。
  fn foreach_child(&self, visit: &mut dyn FnMut(&dyn RequireNode));

  /// cpp `resolvePathToNode()`：`path` 命中已有节点时，以该节点回调一次 `visit`
  /// 并返回 `true`；未命中返回 `false` 且不回调。
  fn resolve_path_to_node(&self, path: &str, visit: &mut dyn FnMut(&dyn RequireNode)) -> bool;

  /// cpp `permitsRelativeRequirePaths()`：是否支持相对 require 路径（`./` / `../`），
  /// 默认 `true`（与 cpp 虚基类默认一致）。
  fn permits_relative_require_paths(&self) -> bool {
    true
  }
}
