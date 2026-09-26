use alloc::string::String;

use crate::{
  functions::{
    make_suggestions_for_first_component::make_suggestions_for_first_component,
    make_suggestions_from_node::make_suggestions_from_node,
  },
  records::require_suggester::RequireSuggester,
  type_aliases::{module_name_type::ModuleName, require_suggestions::RequireSuggestions},
};

/// C++ `RequireSuggester::getRequireSuggestionsImpl`（基类非虚成员，仅依赖
/// `getNode`）；挂在 `dyn RequireSuggester` 上供 trait object 直接调用
/// （`dyn` 保留：trait 实现方跨 crate 开放，共享行为只能挂在 trait object）。
impl dyn RequireSuggester {
  pub(crate) fn get_require_suggestions_impl(
    &self,
    requirer: &ModuleName,
    path: &Option<String>,
  ) -> Option<RequireSuggestions> {
    let path_str = path.as_deref()?;

    // cpp 的 `return nullopt`（`getNode` 落空、或两级路径都没命中）与 Rust 侧的
    // 「回调没写入 result」等价，故候选结果统一由 `result` 表达。
    let mut result: Option<RequireSuggestions> = None;

    self.with_node(requirer, &mut |requirer_node| {
      let Some(slash_pos) = path_str.rfind('/') else {
        result = Some(make_suggestions_for_first_component(requirer_node));
        return;
      };

      // If path already points at a Node, return the Node's children as paths.
      // Otherwise, recover a partial path and use this to generate suggestions.
      let partial_path = &path_str[0..slash_pos];
      for (target, is_partial_path) in [(path_str, false), (partial_path, true)] {
        if requirer_node.resolve_path_to_node(target, &mut |node| {
          result = Some(make_suggestions_from_node(node, path_str, is_partial_path));
        }) {
          break;
        }
      }
    });

    result
  }
}
