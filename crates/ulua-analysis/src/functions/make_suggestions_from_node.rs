extern crate alloc;

use alloc::{
  string::{String, ToString},
  vec::Vec,
};

use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::{
  records::{require_node::RequireNode, require_suggestion::RequireSuggestion},
  type_aliases::require_suggestions::RequireSuggestions,
};

pub(crate) fn make_suggestions_from_node(
  // `dyn` 保留：`RequireNode` 实现方（测试替身等）跨 crate、运行期开放。
  node: &dyn RequireNode,
  path: &str,
  is_partial_path: bool,
) -> RequireSuggestions {
  LUAU_ASSERT!(!path.is_empty());

  let mut result = RequireSuggestions::new();

  let last_slash_in_path = path.rfind('/');

  if let Some(last_slash) = last_slash_in_path {
    let mut parent_suggestion = RequireSuggestion {
      label: "..".to_string(),
      full_path: String::new(),
      tags: Vec::new(),
    };

    if last_slash >= 2 && path.as_bytes().get(last_slash - 2..=last_slash) == Some(b"../") {
      let mut full_path = path[0..=last_slash].to_string();
      full_path.push_str("..");
      parent_suggestion.full_path = full_path;
    } else {
      parent_suggestion.full_path = path[0..last_slash].to_string();
    }

    result.push(parent_suggestion);
  }

  let mut full_path_prefix = String::new();
  if is_partial_path {
    if let Some(last_slash) = last_slash_in_path {
      full_path_prefix.push_str(&path[0..=last_slash]);
    }
  } else {
    if path.ends_with('/') {
      full_path_prefix.push_str(path);
    } else {
      full_path_prefix.push_str(path);
      full_path_prefix.push('/');
    }
  }

  node.foreach_child(&mut |child| {
    let path_component = child.get_path_component();

    if path_component.contains('/') {
      return;
    }

    let label = if is_partial_path || path.ends_with('/') {
      child.get_label()
    } else {
      let mut l = "/".to_string();
      l.push_str(&child.get_label());
      l
    };

    let mut full_path = full_path_prefix.clone();
    full_path.push_str(&path_component);

    let tags = child.get_tags();

    result.push(RequireSuggestion {
      label,
      full_path,
      tags,
    });
  });

  result
}
