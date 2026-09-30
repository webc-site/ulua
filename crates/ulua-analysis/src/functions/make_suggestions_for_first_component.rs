extern crate alloc;

use alloc::{string::ToString, vec::Vec};

use crate::{
  functions::make_suggestions_from_aliases::make_suggestions_from_aliases,
  records::{require_node::RequireNode, require_suggestion::RequireSuggestion},
  type_aliases::require_suggestions::RequireSuggestions,
};

// `dyn` 保留：`RequireNode` 实现方（测试替身等）跨 crate、运行期开放。
pub(crate) fn make_suggestions_for_first_component(node: &dyn RequireNode) -> RequireSuggestions {
  let mut result = make_suggestions_from_aliases(node.get_available_aliases());

  // cpp makeSuggestionsForFirstComponent：仅当节点允许相对路径时补 ./ 与 ../
  if node.permits_relative_require_paths() {
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
