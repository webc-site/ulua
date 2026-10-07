use alloc::{string::String, vec::Vec};

use crate::{records::require_alias::RequireAlias, type_aliases::module_name_type::ModuleName};

/// C++ `Luau::RequireNode`（`Analysis/include/Luau/FileResolver.h`）的 Rust 化。
///
/// cpp 侧它是纯虚基类：`getPathComponent` / `getLabel` / `getTags` /
/// `getAvailableAliases` 四个只读查询 + `getChildren` / `resolvePathToNode` 两个
/// 交回 `unique_ptr<RequireNode>` 的遍历操作。补全管线自始至终**只读数据、不调
/// 行为**（rg 交叉核对：全仓唯一实现方是 ulua-unit-test 的测试替身，且它对这些
/// 虚函数的回答全部是字段值），故此处把虚接口具体成纯数据结构：每字段对应 cpp
/// 一个虚函数的返回值，vtable 与 fat-pointer 一并消失（review.md §0「结构随
/// 语义重写」、§4「消除可单态化的 dyn」）。`getChildren` / `resolvePathToNode`
/// 需要宿主的全量模块表，无法由节点自身回答，因此移到
/// [`RequireSuggester`](crate::records::require_suggester::RequireSuggester) 上，
/// 入参就是这个节点——与 cpp 的 `node->getChildren()` 一一对应，只是查询方从
/// 节点换成了解析器。
///
/// ## DELIBERATE DEVIATION
/// cpp `getChildren()` / `resolvePathToNode()` 是 `RequireNode` 的虚方法；Rust 侧
/// 节点无 Behaviour，这两个操作改由 `RequireSuggester::get_children` /
/// `resolve_path_to_node` 以节点为键回答（cpp `RequireNode` 的测试实现同样把
/// 二者委托给 `TestFileResolver::source`，语义等价）。此改写同时去掉了原先
/// 「栈上节点 + `&mut dyn FnMut(&dyn RequireNode)` 访问者」的两层虚分派：
/// 遍历子节点现在是 `Vec<RequireNode>` 的具体返回值，即 cpp
/// `vector<unique_ptr<RequireNode>>` 去掉指针那层。
#[derive(Clone, Debug)]
pub struct RequireNode {
  /// 节点身份：cpp `TestRequireNode::moduleName`。分析侧只把它回传给
  /// [`RequireSuggester`](crate::records::require_suggester::RequireSuggester)
  /// 用于查子节点/解相对路径，不解释其内容（宿主可用自己的键空间）。
  pub module_name: ModuleName,
  /// cpp `getPathComponent()`。
  pub path_component: String,
  /// cpp `getLabel()`：基类缺省即 `getPathComponent()`。
  pub label: String,
  /// cpp `getTags()`：基类缺省为空。
  pub tags: Vec<String>,
  /// cpp `getAvailableAliases()`。
  pub aliases: Vec<RequireAlias>,
  /// cpp `permitsRelativeRequirePaths()`：是否支持相对 require 路径（`./` / `../`），
  /// 基类缺省 `true`（见 [`RequireNode::new`]）。
  pub permits_relative_require_paths: bool,
}

impl RequireNode {
  /// 按 cpp `RequireNode` 基类缺省构造：label 取 path component、无 tags、无
  /// aliases、允许相对路径。宿主需要偏离缺省时用函数更新语法覆写对应字段
  /// （`..RequireNode::new(name, component)`），避免 `Default` 把
  /// 「允许相对路径」静默置成 `false`。
  pub fn new(module_name: ModuleName, path_component: String) -> Self {
    let label = path_component.clone();
    Self {
      module_name,
      path_component,
      label,
      tags: Vec::new(),
      aliases: Vec::new(),
      permits_relative_require_paths: true,
    }
  }
}
