use ulua_ast::{
  records::{ast_expr_function::AstExprFunction, ast_node::AstNode, position::Position},
  rtti::ast_node_is,
};
use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::{
  enums::autocomplete_entry_kind::AutocompleteEntryKind,
  records::autocomplete_entry::AutocompleteEntry,
  type_aliases::autocomplete_entry_map::AutocompleteEntryMap,
};
pub fn autocomplete_keywords(
  ancestry: &[*mut AstNode],
  _position: Position,
  result: &mut AutocompleteEntryMap,
) {
  LUAU_ASSERT!(!ancestry.is_empty());

  let node = *ancestry.last().unwrap();

  let is_expr_function = unsafe { ast_node_is::<AstExprFunction>(&*node) };
  let is_expr = unsafe { !(*node).as_expr().is_null() };

  if !is_expr_function && is_expr {
    result.insert(
      "and".to_string(),
      AutocompleteEntry {
        kind: AutocompleteEntryKind::Keyword,
        ..Default::default()
      },
    );
    result.insert(
      "or".to_string(),
      AutocompleteEntry {
        kind: AutocompleteEntryKind::Keyword,
        ..Default::default()
      },
    );
    result.insert(
      "not".to_string(),
      AutocompleteEntry {
        kind: AutocompleteEntryKind::Keyword,
        ..Default::default()
      },
    );
  }
}
