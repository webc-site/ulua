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
/// `getNode`）；挂在 `dyn RequireSuggester` 上供 trait object 直接调用。
impl dyn RequireSuggester {
  pub(crate) fn get_require_suggestions_impl(
    &self,
    requirer: &ModuleName,
    path: &Option<String>,
  ) -> Option<RequireSuggestions> {
    let path_str = path.as_ref()?;

    let requirer_node = self.get_node(requirer)?;

    let Some(slash_pos) = path_str.rfind('/') else {
      return Some(make_suggestions_for_first_component(requirer_node));
    };

    // If path already points at a Node, return the Node's children as paths.
    if let Some(node) = requirer_node.resolve_path_to_node(path_str) {
      return Some(make_suggestions_from_node(
        node, path_str, /* is_partial_path = */ false,
      ));
    }

    // Otherwise, recover a partial path and use this to generate suggestions.
    let partial_path = &path_str[0..slash_pos];
    if let Some(partial_node) = requirer_node.resolve_path_to_node(partial_path) {
      return Some(make_suggestions_from_node(
        partial_node,
        path_str,
        /* is_partial_path = */ true,
      ));
    }

    None
  }
}
