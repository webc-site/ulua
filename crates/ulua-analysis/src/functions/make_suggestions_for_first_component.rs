use alloc::{string::ToString, vec::Vec};

use crate::{
  functions::make_suggestions_from_aliases::make_suggestions_from_aliases,
  records::{require_node::RequireNode, require_suggestion::RequireSuggestion},
  type_aliases::require_suggestions::RequireSuggestions,
};

/// C++ `makeSuggestionsForFirstComponent`：路径首段（尚无 `/`）时给出可用别名与
/// 两个相对路径入口。节点是具体数据（见 [`RequireNode`]），无虚分派。
pub(crate) fn make_suggestions_for_first_component(node: &RequireNode) -> RequireSuggestions {
  let mut result = make_suggestions_from_aliases(node.aliases.clone());

  // cpp makeSuggestionsForFirstComponent：仅当节点允许相对路径时补 ./ 与 ../
  if node.permits_relative_require_paths {
    result.push(RequireSuggestion {
      label: "./".to_string(),
      full_path: "./".to_string(),
      tags: Vec::new(),
    });

    result.push(RequireSuggestion {
      label: "../".to_string(),
      full_path: "../".to_string(),
      tags: Vec::new(),
    });
  }

  result
}
