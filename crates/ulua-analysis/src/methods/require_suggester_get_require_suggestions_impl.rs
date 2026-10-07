use alloc::string::String;

use crate::{
  functions::{
    make_suggestions_for_first_component::make_suggestions_for_first_component,
    make_suggestions_from_node::make_suggestions_from_node,
  },
  records::require_suggester::RequireSuggester,
  type_aliases::{module_name_type::ModuleName, require_suggestions::RequireSuggestions},
};

/// C++ `RequireSuggester::getRequireSuggestionsImpl`（`Analysis/src/FileResolver.cpp`
/// 基类非虚成员，仅依赖 `getNode`）。
///
/// 挂在 `dyn RequireSuggester` 上：这是 cpp「基类非虚公共实现」的对应物——共享
/// 行为既不能做成 trait 默认方法（trait 方法可见性必须等于 trait，无法保持
/// `pub(crate)`），也不该扩到 `impl<T: RequireSuggester>`（那会让 `Arc<dyn _>`
/// 注入槽取不到本方法）。`dyn` 保留理由见
/// [`RequireSuggester`](crate::records::require_suggester::RequireSuggester) 注。
impl dyn RequireSuggester {
  pub(crate) fn get_require_suggestions_impl(
    &self,
    requirer: &ModuleName,
    path: &Option<String>,
  ) -> Option<RequireSuggestions> {
    let path_str = path.as_deref()?;

    // cpp `getNode(requirer)` 落空即 UB（解引用 nullptr）；Rust 侧把「无根节点」
    // 收为 None，与下方两级路径都没命中的 `return nullopt` 同结果。
    let requirer_node = self.get_node(requirer)?;

    let Some(slash_pos) = path_str.rfind('/') else {
      return Some(make_suggestions_for_first_component(&requirer_node));
    };

    // If path already points at a Node, return the Node's children as paths.
    // Otherwise, recover a partial path and use this to generate suggestions.
    let partial_path = &path_str[0..slash_pos];
    // cpp 先试完整 path（isPartialPath=false），落空再试去掉末段的
    // partialPath（isPartialPath=true）；两次的候选文本仍是原 path。
    for (target, is_partial_path) in [(path_str, false), (partial_path, true)] {
      if let Some(node) = self.resolve_path_to_node(&requirer_node, target) {
        return Some(make_suggestions_from_node(
          self,
          &node,
          path_str,
          is_partial_path,
        ));
      }
    }

    None
  }
}
