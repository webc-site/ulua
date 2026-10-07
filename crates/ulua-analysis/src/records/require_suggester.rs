use alloc::vec::Vec;
use core::fmt::Debug;

use crate::{records::require_node::RequireNode, type_aliases::module_name_type::ModuleName};

/// C++ `Luau::RequireSuggester` 虚基类（`Analysis/include/Luau/FileResolver.h`）
/// 的 Rust 化：虚函数接口 → trait，实现方以 `Arc<dyn RequireSuggester>` 交给宿主
/// 注入的 [`FileResolver`](crate::records::file_resolver::FileResolver)，替代原先
/// `#[repr(C)]` 手写 vtable + `unsafe fn` 指针。
///
/// `Debug` 约束：实现方会作为 `Arc<dyn RequireSuggester>` 挂在测试 resolver 等
/// 带 `#[derive(Debug)]` 的结构上。
///
/// 此处 `dyn` 保留（review.md §4「类型集合运行期开放」条款）：本 trait 是宿主
/// 注入点（cpp `FileResolver::requireSuggester` 的 `shared_ptr` 同位），实现方在
/// ulua-analysis 之外（rg 交叉核对：当前唯一实现是 ulua-unit-test 的
/// `TestRequireSuggester`，`crates/ulua-web` / `ulua-rt` 侧接真实解析器时即第二个
/// 实现方），ulua-analysis 无法命名下游类型，故既不能 `enum_dispatch` 穷举
/// （会构成 crate 循环），也不宜把 trait 泛型化（宿主注入槽 `Arc<dyn _>` 无从
/// 定型）。三个方法的形参/返回全是本 crate 内的具体类型，对象安全性不受影响：
/// 每个补全查询只有 1~3 次 vtable 调用，原先「访问者 + `&dyn RequireNode`」
/// 逐节点二次分派已随 [`RequireNode`] 具体化消失。
pub trait RequireSuggester: Debug {
  /// cpp `getNode`：按模块名取根节点；`None` 即该模块无候选节点
  /// （cpp 返回 `unique_ptr<RequireNode>`，Rust 侧按值给出具体节点，零装箱）。
  fn get_node(&self, name: &ModuleName) -> Option<RequireNode>;

  /// cpp `RequireNode::getChildren`：给出 `node` 的直接子节点（cpp 的
  /// `vector<unique_ptr<RequireNode>>` 去掉指针那层，节点自身无 Behaviour，
  /// 遍历需要的全量模块表由实现方持有）。
  fn get_children(&self, node: &RequireNode) -> Vec<RequireNode>;

  /// cpp `RequireNode::resolvePathToNode`：把相对 `node` 的 `path` 归一化后回查
  /// 节点；未命中返回 `None`（cpp 的 `nullptr`）。
  fn resolve_path_to_node(&self, node: &RequireNode, path: &str) -> Option<RequireNode>;
}
