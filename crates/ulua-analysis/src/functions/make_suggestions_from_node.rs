extern crate alloc;

use alloc::{
  string::{String, ToString},
  vec::Vec,
};

use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::{
  records::{
    require_node::RequireNode, require_suggester::RequireSuggester,
    require_suggestion::RequireSuggestion,
  },
  type_aliases::require_suggestions::RequireSuggestions,
};

/// C++ `makeSuggestionsFromNode`（`Analysis/src/FileResolver.cpp`）：以 `node` 的
/// 直接子节点生成 require 候选。
///
/// `suggester: &dyn RequireSuggester` 的 `dyn` 保留：它是宿主注入的
/// [`RequireSuggester`](crate::records::require_suggester::RequireSuggester) 在
/// [`FileResolver`](crate::records::file_resolver::FileResolver) 处已擦除的边界，
/// 本函数只在边界的下游取子节点。节点本身是具体数据（见 [`RequireNode`]），
/// 故逐子节点的 vtable 调用已不存在，一次查询只剩 `get_children` 一次间接。
pub(crate) fn make_suggestions_from_node(
  suggester: &dyn RequireSuggester,
  node: &RequireNode,
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
  } else if path.ends_with('/') {
    full_path_prefix.push_str(path);
  } else {
    full_path_prefix.push_str(path);
    full_path_prefix.push('/');
  }

  for child in suggester.get_children(node) {
    if child.path_component.contains('/') {
      continue;
    }

    let label = if is_partial_path || path.ends_with('/') {
      child.label
    } else {
      let mut l = "/".to_string();
      l.push_str(&child.label);
      l
    };

    let mut full_path = full_path_prefix.clone();
    full_path.push_str(&child.path_component);

    result.push(RequireSuggestion {
      label,
      full_path,
      tags: child.tags,
    });
  }

  result
}
