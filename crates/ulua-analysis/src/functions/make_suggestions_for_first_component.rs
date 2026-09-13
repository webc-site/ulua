extern crate alloc;

use alloc::{boxed::Box, string::ToString, vec::Vec};

use crate::{
  functions::make_suggestions_from_aliases::make_suggestions_from_aliases,
  records::{require_node::RequireNode, require_suggestion::RequireSuggestion},
  type_aliases::require_suggestions::RequireSuggestions,
};

pub(crate) fn make_suggestions_for_first_component(
  node: Box<dyn RequireNode>,
) -> RequireSuggestions {
  let mut result = make_suggestions_from_aliases(node.get_available_aliases());

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

  result
}
