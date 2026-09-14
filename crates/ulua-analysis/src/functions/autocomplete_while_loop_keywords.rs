use ulua_ast::records::ast_node::AstNode;

use crate::records::autocomplete_entry::AutocompleteEntry;
extern crate alloc;

use alloc::{string::ToString, vec::Vec};

use crate::{
  enums::{
    autocomplete_context::AutocompleteContext, autocomplete_entry_kind::AutocompleteEntryKind,
  },
  records::autocomplete_result::AutocompleteResult,
  type_aliases::autocomplete_entry_map::AutocompleteEntryMap,
};

pub fn autocomplete_while_loop_keywords(ancestry: Vec<*mut AstNode>) -> AutocompleteResult {
  let mut entry_map: AutocompleteEntryMap = AutocompleteEntryMap::new();

  entry_map.insert(
    "do".to_string(),
    AutocompleteEntry {
      kind: AutocompleteEntryKind::Keyword,
      ..AutocompleteEntry::default()
    },
  );
  entry_map.insert(
    "and".to_string(),
    AutocompleteEntry {
      kind: AutocompleteEntryKind::Keyword,
      ..AutocompleteEntry::default()
    },
  );
  entry_map.insert(
    "or".to_string(),
    AutocompleteEntry {
      kind: AutocompleteEntryKind::Keyword,
      ..AutocompleteEntry::default()
    },
  );

  AutocompleteResult::autocomplete_result_autocomplete_entry_map_vector_ast_node_autocomplete_context(
        entry_map,
        ancestry,
        AutocompleteContext::Keyword,
    )
}
